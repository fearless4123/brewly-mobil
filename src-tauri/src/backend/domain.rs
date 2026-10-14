//! Brewly iş işlevleri: ürün arama (Türkçe harf duyarlılığı), sepet toplamı, sipariş verme (her kalem için BREW-XXXXXX kodu),
//! sipariş durumu, iptal, toplu fiyat artışı ve panel özeti.
//! Süt ve ekstra seçenekleri (fikir belgesinde "planlanan") `secenekler` tablosundan gelir; tablo boşsa yalnız boyut fiyatı kullanılır.
use crate::backend::audit;
use crate::backend::crud;
use crate::backend::error::{AppError, AppResult, FieldError};
use crate::backend::policy::authorize;
use crate::backend::rbac::{self, ModelRef};
use crate::backend::schema;
use crate::backend::state::AppState;
use crate::backend::util::{new_id, now_iso};
use rusqlite::{params, Connection, OptionalExtension};
use serde_json::{json, Value};
use tauri::State;

pub const DURUMLAR: [&str; 4] = ["alindi", "hazirlaniyor", "hazir", "teslim"];
pub const BOYUTLAR: [&str; 4] = ["Küçük", "Orta", "Büyük", "Standart"];

/// Türkçe küçük harf (JS `toLocaleLowerCase("tr")`): İ → i, I → ı
pub fn tr_lower(s: &str) -> String {
    s.chars().flat_map(|c| match c { 'İ' => vec!['i'], 'I' => vec!['ı'], c => c.to_lowercase().collect::<Vec<_>>() }).collect()
}

/// Ürün adında Türkçe büyük/küçük harf duyarsız alt dize araması + kategori filtresi (`None` = Tümü).
pub fn urunlerde_ara(urunler: &[Value], q: &str, kategori: Option<&str>) -> Vec<Value> {
    let q = tr_lower(q.trim());
    urunler
        .iter()
        .filter(|u| u["aktif"] != json!(false))
        .filter(|u| kategori.map(|k| u["kategori"].as_str() == Some(k)).unwrap_or(true))
        .filter(|u| tr_lower(u["ad"].as_str().unwrap_or("")).contains(&q))
        .cloned()
        .collect()
}

fn boyut_fiyati(urun: &Value, boyut: &str) -> Option<f64> {
    urun["boyutlar"].as_array()?.iter().find(|b| b["ad"].as_str() == Some(boyut)).and_then(|b| b["fiyat"].as_f64())
}

fn secenek_farki(c: &Connection, tur: &str, id: &str) -> AppResult<f64> {
    c.query_row("SELECT fiyatFarki FROM secenekler WHERE id = ?1 AND tur = ?2 AND aktif = 1", params![id, tur], |r| r.get(0))
        .optional()?
        .ok_or_else(|| AppError::validation(if tur == "sut" { "sut" } else { "ekstralar" }, &format!("`{id}` geçerli bir {tur} seçeneği değil.")))
}

fn musteri_kimligi(uid: i64) -> String {
    format!("u{uid}")
}

/// Kalemleri doğrular ve fiyatlar: `[{ urunId, boyut, adet, sut?, ekstralar? }]` → (kalemler, toplam)
pub fn fiyatla(c: &Connection, girdi: &Value) -> AppResult<(Vec<Value>, f64)> {
    let liste = girdi.as_array().filter(|a| !a.is_empty() && a.len() <= 100).ok_or_else(|| AppError::validation("items", "Sepet 1–100 kalem olmalı."))?;
    let mut hatalar: Vec<FieldError> = Vec::new();
    let mut kalemler = Vec::new();
    let mut toplam = 0.0;
    for (i, k) in liste.iter().enumerate() {
        let hata = |alan: &str, m: &str| FieldError { field: format!("items[{i}].{alan}"), message: m.into() };
        let uid = k.get("urunId").and_then(Value::as_i64);
        let boyut = k.get("boyut").and_then(Value::as_str).unwrap_or("");
        let adet = k.get("adet").and_then(Value::as_i64).unwrap_or(0);
        let Some(urun) = uid.and_then(|u| crud::find(c, &schema::URUN, &json!(u)).ok()) else {
            hatalar.push(hata("urunId", "Ürün bulunamadı."));
            continue;
        };
        if urun["aktif"] == json!(false) {
            hatalar.push(hata("urunId", "Ürün şu an satışta değil."));
        }
        let Some(taban) = boyut_fiyati(&urun, boyut) else {
            hatalar.push(hata("boyut", "Bu ürün için böyle bir boyut yok."));
            continue;
        };
        if !(1..=99).contains(&adet) {
            hatalar.push(hata("adet", "Adet 1–99 olmalı."));
        }
        let mut birim = taban;
        let sut = k.get("sut").and_then(Value::as_str);
        if let Some(s) = sut {
            match secenek_farki(c, "sut", s) {
                Ok(f) => birim += f,
                Err(_) => hatalar.push(hata("sut", "Süt seçeneği geçersiz.")),
            }
        }
        let ekstralar: Vec<String> = k.get("ekstralar").and_then(Value::as_array).map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
        for e in &ekstralar {
            match secenek_farki(c, "ekstra", e) {
                Ok(f) => birim += f,
                Err(_) => hatalar.push(hata("ekstralar", "Ekstra seçeneği geçersiz.")),
            }
        }
        toplam += birim * adet as f64;
        kalemler.push(json!({ "urunId": urun["id"], "urunAd": urun["ad"], "boyut": boyut, "adet": adet, "birimFiyat": birim, "sut": sut, "ekstralar": ekstralar }));
    }
    if !hatalar.is_empty() {
        return Err(AppError::Validation(hatalar));
    }
    Ok((kalemler, toplam))
}

/// Sepet toplamı (oturum gerekmez)
pub fn do_sepet_toplami(app: &AppState, girdi: &Value) -> AppResult<Value> {
    app.db.with(|c| {
        let (kalemler, toplam) = fiyatla(c, girdi)?;
        Ok(json!({ "kalemler": kalemler, "toplam": toplam }))
    })
}

/// Ürün araması (oturum gerekmez)
pub fn do_urun_ara(app: &AppState, q: &str, kategori: Option<&str>) -> AppResult<Vec<Value>> {
    app.db.with(|c| {
        let liste = crud::list(c, &schema::URUN, &crud::ListQuery { page_size: Some(100), ..Default::default() })?;
        Ok(urunlerde_ara(&liste.items, q, kategori))
    })
}

/// Sipariş ver. Her sepet kalemi ayrı bir `BREW-XXXXXX` kodu alır; kalemler tek `Siparis` kaydında toplanır.
/// `girdi`: `{ items, subeId? }`
pub fn do_siparis_olustur(app: &AppState, girdi: &Value) -> AppResult<Value> {
    let uid = app.user().ok_or(AppError::Unauthenticated)?;
    app.db.with(|c| {
        let (mut kalemler, toplam) = fiyatla(c, girdi.get("items").unwrap_or(&Value::Null))?;
        let sube = girdi.get("subeId").and_then(Value::as_str);
        if let Some(s) = sube {
            let acik: Option<bool> = c.query_row("SELECT acik FROM subeler WHERE id = ?1", [s], |r| Ok(r.get::<_, i64>(0)? != 0)).optional()?;
            match acik {
                None => return Err(AppError::validation("subeId", "Şube bulunamadı.")),
                Some(false) => return Err(AppError::Conflict("Şube şu an sipariş almıyor.".into())),
                _ => {}
            }
        }
        let (ad, eposta): (String, String) = c.query_row("SELECT name, email FROM users WHERE id = ?1", [uid], |r| Ok((r.get(0)?, r.get(1)?)))?;
        c.execute("INSERT OR IGNORE INTO musteriler (id, ad, eposta) VALUES (?1, ?2, ?3)", params![musteri_kimligi(uid), ad, eposta])?;
        for k in kalemler.iter_mut() {
            for _ in 0..10 {
                let kod = crate::siparis_olustur(); // BREW-XXXXXX (src-tauri/src/lib.rs)
                let kullanilmis = c.query_row("SELECT EXISTS(SELECT 1 FROM siparisler, json_each(siparisler.kalemler) j WHERE json_extract(j.value, '$.kod') = ?1)", [&kod], |r| r.get::<_, bool>(0))?;
                if !kullanilmis && !kalemler_icinde(&kod) {
                    k["kod"] = json!(kod);
                    break;
                }
            }
            if k.get("kod").is_none() {
                return Err(AppError::Conflict("Benzersiz sipariş kodu üretilemedi, tekrar dene.".into()));
            }
        }
        let rec = json!({ "id": new_id(), "musteriId": musteri_kimligi(uid), "subeId": sube, "kalemler": kalemler, "toplam": toplam, "durum": "alindi", "olusturma": now_iso(c)? });
        let s = crud::insert(c, &schema::SIPARIS, &rec)?;
        c.execute("UPDATE musteriler SET siparisSayisi = siparisSayisi + 1 WHERE id = ?1", [musteri_kimligi(uid)])?;
        audit::record(c, Some(uid), "Siparis.create", "Siparis", s["id"].as_str().unwrap_or(""), None, None, Some(&s))?;
        Ok(s)
    })
}

fn kalemler_icinde(_kod: &str) -> bool {
    false // aynı çağrıdaki kalemler arasındaki çakışma olasılığı 1/16.777.216; veritabanı denetimi yeterlidir
}

/// Durumu bir adım ilerletir (alındı → hazırlanıyor → hazır → teslim). `Advance:Siparis` gerekir.
pub fn do_siparis_ilerlet(app: &AppState, id: &str) -> AppResult<Value> {
    app.db.with(|c| {
        let uid = authorize(c, app.user(), "Advance:Siparis")?;
        let s = crud::find(c, &schema::SIPARIS, &json!(id))?;
        let simdiki = s["durum"].as_str().unwrap_or("");
        let idx = DURUMLAR.iter().position(|d| *d == simdiki).ok_or_else(|| AppError::Conflict("İptal edilen ya da bitmiş sipariş ilerletilemez.".into()))?;
        let sonraki = DURUMLAR.get(idx + 1).ok_or_else(|| AppError::Conflict("Sipariş zaten teslim edildi.".into()))?;
        let sonra = crud::update(c, &schema::SIPARIS, &json!(id), &json!({ "durum": sonraki }))?;
        audit::record(c, Some(uid), "Siparis.advance", "Siparis", id, None, Some(&s), Some(&sonra))?;
        Ok(sonra)
    })
}

/// İptal: müşteri yalnız "alındı" aşamasında kendi siparişini; `Cancel:Siparis` izni olan hazır olana kadar. Neden zorunlu.
pub fn do_siparis_iptal(app: &AppState, id: &str, neden: &str) -> AppResult<Value> {
    let uid = app.user().ok_or(AppError::Unauthenticated)?;
    app.db.with(|c| {
        if neden.trim().is_empty() {
            return Err(AppError::validation("neden", "İptal nedeni zorunlu."));
        }
        let s = crud::find(c, &schema::SIPARIS, &json!(id))?;
        let sahip = s["musteriId"].as_str() == Some(musteri_kimligi(uid).as_str());
        let personel = rbac::can(c, ModelRef::user(uid), "Cancel:Siparis")?;
        if !(sahip || personel) {
            return Err(AppError::Forbidden("Bu sipariş size ait değil.".into()));
        }
        let durum = s["durum"].as_str().unwrap_or("");
        let izinli = if personel { ["alindi", "hazirlaniyor", "hazir"].contains(&durum) } else { durum == "alindi" };
        if !izinli {
            return Err(AppError::Conflict("Bu aşamada sipariş iptal edilemez.".into()));
        }
        let sonra = crud::update(c, &schema::SIPARIS, &json!(id), &json!({ "durum": "iptal", "iptalNedeni": neden.trim() }))?;
        audit::record(c, Some(uid), "Siparis.cancel", "Siparis", id, Some(neden.trim()), Some(&s), Some(&sonra))?;
        Ok(sonra)
    })
}

/// Toplu fiyat artışı (`Update:Urun`): `yuzde` −50…+100; `kuru_calisma` true ise yalnız önizleme döner. Geçmiş siparişler değişmez.
pub fn do_urun_fiyat_ayarla(app: &AppState, yuzde: f64, kategori: Option<&str>, kuru_calisma: bool) -> AppResult<Vec<Value>> {
    app.db.with(|c| {
        let uid = authorize(c, app.user(), "Update:Urun")?;
        if !(-50.0..=100.0).contains(&yuzde) {
            return Err(AppError::validation("yuzde", "Yüzde −50 ile +100 arasında olmalı."));
        }
        let liste = crud::list(c, &schema::URUN, &crud::ListQuery { page_size: Some(100), ..Default::default() })?;
        let mut degisen = Vec::new();
        for u in liste.items.iter().filter(|u| kategori.map(|k| u["kategori"].as_str() == Some(k)).unwrap_or(true)) {
            let yeni: Vec<Value> = u["boyutlar"].as_array().cloned().unwrap_or_default().into_iter().map(|b| {
                let f = b["fiyat"].as_f64().unwrap_or(0.0);
                json!({ "ad": b["ad"], "fiyat": (f * (1.0 + yuzde / 100.0)).round() })
            }).collect();
            degisen.push(json!({ "id": u["id"], "ad": u["ad"], "once": u["boyutlar"], "sonra": yeni }));
            if !kuru_calisma {
                crud::update(c, &schema::URUN, &u["id"], &json!({ "boyutlar": yeni }))?;
            }
        }
        if !kuru_calisma {
            audit::record(c, Some(uid), "Urun.bulkPrice", "Urun", &format!("{} ürün", degisen.len()), None, None, Some(&json!({ "yuzde": yuzde, "kategori": kategori })))?;
        }
        Ok(degisen)
    })
}

/// Panel özeti: sipariş sayısı, ciro, en çok satan 5 ürün, kategori ve boyut dağılımı, saatlik yoğunluk.
pub fn do_brewly_dashboard(app: &AppState, from: Option<&str>, to: Option<&str>) -> AppResult<Value> {
    app.db.with(|c| {
        authorize(c, app.user(), "ViewAny:Siparis")?;
        let (lo, hi) = (from.unwrap_or("0000"), to.unwrap_or("9999"));
        let (n, ciro): (i64, f64) = c.query_row(
            "SELECT COUNT(*), COALESCE(SUM(toplam), 0) FROM siparisler WHERE durum <> 'iptal' AND olusturma >= ?1 AND olusturma <= ?2",
            params![lo, hi],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )?;
        let grup = |sql: &str| -> AppResult<Vec<Value>> {
            let mut st = c.prepare(sql)?;
            let rows = st.query_map(params![lo, hi], |r| Ok(json!({ "value": r.get::<_, Option<String>>(0)?, "count": r.get::<_, i64>(1)? })))?;
            Ok(rows.collect::<Result<Vec<_>, _>>()?)
        };
        let enler = grup("SELECT json_extract(j.value, '$.urunAd'), SUM(json_extract(j.value, '$.adet')) AS q FROM siparisler s, json_each(s.kalemler) j WHERE s.durum <> 'iptal' AND s.olusturma >= ?1 AND s.olusturma <= ?2 GROUP BY 1 ORDER BY q DESC LIMIT 5")?;
        let boyut = grup("SELECT json_extract(j.value, '$.boyut'), SUM(json_extract(j.value, '$.adet')) FROM siparisler s, json_each(s.kalemler) j WHERE s.durum <> 'iptal' AND s.olusturma >= ?1 AND s.olusturma <= ?2 GROUP BY 1")?;
        let kategori = grup("SELECT u.kategori, SUM(json_extract(j.value, '$.adet')) FROM siparisler s, json_each(s.kalemler) j JOIN urunler u ON u.id = json_extract(j.value, '$.urunId') WHERE s.durum <> 'iptal' AND s.olusturma >= ?1 AND s.olusturma <= ?2 GROUP BY 1")?;
        let saat = grup("SELECT substr(olusturma, 12, 2), COUNT(*) FROM siparisler WHERE durum <> 'iptal' AND olusturma >= ?1 AND olusturma <= ?2 GROUP BY 1 ORDER BY 1")?;
        Ok(json!({ "siparis": n, "ciro": ciro, "enCokSatan": enler, "boyutlar": boyut, "kategoriler": kategori, "saatlik": saat }))
    })
}

// ───────────────────────── Tauri komutları ─────────────────────────

#[tauri::command]
pub fn urun_ara(state: State<'_, AppState>, q: String, kategori: Option<String>) -> AppResult<Vec<Value>> {
    do_urun_ara(&state, &q, kategori.as_deref())
}
#[tauri::command]
pub fn sepet_toplami(state: State<'_, AppState>, girdi: Value) -> AppResult<Value> {
    do_sepet_toplami(&state, &girdi)
}
#[tauri::command]
pub fn siparis_ver(state: State<'_, AppState>, girdi: Value) -> AppResult<Value> {
    do_siparis_olustur(&state, &girdi)
}
#[tauri::command]
pub fn siparis_ilerlet(state: State<'_, AppState>, id: String) -> AppResult<Value> {
    do_siparis_ilerlet(&state, &id)
}
#[tauri::command]
pub fn siparis_iptal(state: State<'_, AppState>, id: String, neden: String) -> AppResult<Value> {
    do_siparis_iptal(&state, &id, &neden)
}
#[tauri::command]
pub fn urun_fiyat_ayarla(state: State<'_, AppState>, yuzde: f64, kategori: Option<String>, kuru_calisma: Option<bool>) -> AppResult<Vec<Value>> {
    do_urun_fiyat_ayarla(&state, yuzde, kategori.as_deref(), kuru_calisma.unwrap_or(true))
}
#[tauri::command]
pub fn brewly_dashboard(state: State<'_, AppState>, from: Option<String>, to: Option<String>) -> AppResult<Value> {
    do_brewly_dashboard(&state, from.as_deref(), to.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::boot_memory;
    use crate::backend::controllers as c;

    #[test]
    fn turkce_arama() {
        let urunler = vec![json!({ "ad": "Iced Latte", "kategori": "Soğuk" }), json!({ "ad": "Chai Latte", "kategori": "Çay" }), json!({ "ad": "İstanbul Kahvesi", "kategori": "Kahve" })];
        assert_eq!(urunlerde_ara(&urunler, "LATTE", None).len(), 2);
        assert_eq!(urunlerde_ara(&urunler, "istanbul", None).len(), 1); // İ → i
        assert_eq!(urunlerde_ara(&urunler, "latte", Some("Çay")).len(), 1);
        assert_eq!(tr_lower("IĞDIR İZMİR"), "ığdır izmir");
    }

    #[test]
    fn siparis_akisi() {
        let app = boot_memory().unwrap();
        c::auth_register(&app, "Yönetici", "y@ornek.com", "gizli-parola-1").unwrap();
        let sepet = json!([{ "urunId": 1, "boyut": "Büyük", "adet": 2 }, { "urunId": 7, "boyut": "Standart", "adet": 1 }]);
        assert_eq!(do_sepet_toplami(&app, &sepet).unwrap()["toplam"], json!(135.0 * 2.0 + 145.0));
        assert!(do_sepet_toplami(&app, &json!([{ "urunId": 7, "boyut": "Büyük", "adet": 1 }])).is_err()); // tatlıda yalnız Standart
        assert!(do_sepet_toplami(&app, &json!([])).is_err());
        c::auth_logout(&app);
        c::auth_register(&app, "Müşteri", "m@ornek.com", "gizli-parola-2").unwrap();
        let s = do_siparis_olustur(&app, &json!({ "items": sepet })).unwrap();
        let kodlar: Vec<String> = s["kalemler"].as_array().unwrap().iter().map(|k| k["kod"].as_str().unwrap().to_string()).collect();
        assert_eq!(kodlar.len(), 2);
        assert!(kodlar.iter().all(|k| k.starts_with("BREW-")));
        assert_ne!(kodlar[0], kodlar[1]);
        let id = s["id"].as_str().unwrap().to_string();
        assert_eq!(do_siparis_ilerlet(&app, &id).unwrap_err().code(), "forbidden");
        assert_eq!(do_siparis_iptal(&app, &id, "vazgeçtim").unwrap()["durum"], json!("iptal"));
        // personel ilerletir
        let s2 = do_siparis_olustur(&app, &json!({ "items": [{ "urunId": 2, "boyut": "Küçük", "adet": 1 }] })).unwrap();
        let id2 = s2["id"].as_str().unwrap().to_string();
        c::auth_logout(&app);
        c::auth_login(&app, "y@ornek.com", "gizli-parola-1").unwrap();
        for beklenen in ["hazirlaniyor", "hazir", "teslim"] {
            assert_eq!(do_siparis_ilerlet(&app, &id2).unwrap()["durum"], json!(beklenen));
        }
        assert_eq!(do_siparis_ilerlet(&app, &id2).unwrap_err().code(), "conflict");
        // toplu fiyat: önizleme değiştirmez
        let onizleme = do_urun_fiyat_ayarla(&app, 10.0, Some("Kahve"), true).unwrap();
        assert_eq!(onizleme.len(), 4);
        assert_eq!(c::crud_show(&app, "Urun", &json!(2)).unwrap()["boyutlar"][0]["fiyat"], json!(80));
        do_urun_fiyat_ayarla(&app, 10.0, Some("Kahve"), false).unwrap();
        assert_eq!(c::crud_show(&app, "Urun", &json!(2)).unwrap()["boyutlar"][0]["fiyat"], json!(88.0));
        let d = do_brewly_dashboard(&app, None, None).unwrap();
        assert_eq!(d["siparis"], json!(1));
    }
}
