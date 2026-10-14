# Brewly — Geçici Yerel Arka Uç (SQLite)

> **Durum:** Sunucu yok. Veri, uygulamanın kendi veri klasöründeki bir **SQLite** dosyasında (`app.sqlite`) tutulur; kod `src-tauri/src/backend/` altındadır ve Tauri komutları olarak dışarı açılır. Sunucuya geçilince yalnız bu klasörün içi değişir; ekranların kullandığı komut adları ve veri biçimi aynı kalacak şekilde tasarlandı.

Rol ve izin düzeni, eğitmenin Laravel projelerindeki `spatie/laravel-permission` (+ Filament Shield) mantığındadır.

## 1. Laravel karşılıkları

| Laravel | Bu depoda |
|---|---|
| `database/migrations` | `src-tauri/src/backend/migrations/*.sql` + `migrations.rs`; uygulama açılışında bekleyenler çalışır (`migrations` tablosu: id, migration, batch) |
| `spatie/laravel-permission` | `rbac.rs`: roller, izinler, `assign_role`, `sync_roles`, `give_permission_to`, `can` … (tablolar Spatie ile aynı) |
| Filament Shield izin adları | `Eylem:Kaynak` (örn. `ViewAny:Order`), eylemler: ViewAny, View, Create, Update, Delete, DeleteAny |
| Süper yönetici (`Gate::before`) | `admin` rolü her izin denetimini geçer |
| Policy / `authorize()` | `policy.rs`; her denetleyici işlevi önce izni denetler |
| Resource controller | `controllers.rs` → `crud_index/show/store/update/destroy` (her kaynak için) |
| `spatie/laravel-activitylog` | `audit.rs` → `audit_logs` tablosu (kim, ne, önce/sonra, neden) |
| `artisan qtoefl:admin` | `user_store` (kullanıcıyı oluşturur/günceller ve rol atar) |
| Eloquent modeli | `schema.rs` içindeki `Entity` tanımı (sütun adları ön yüz tipleriyle aynı) |
| Form request doğrulaması | `crud.rs` tür/zorunluluk denetimi + `domain.rs` iş kuralı doğrulamaları → `{ code: "validation", errors: [{ field, message }] }` |

## 2. Veritabanı dosyası ve ilk açılış

- Dosya: uygulama veri klasörü / `app.sqlite` (Tauri `app_data_dir`). Depoya girmez.
- Açılışta migration'lar sırayla çalışır; roller, izinler ve başlangıç verisi **veri migration'larıyla** gelir (seeder gerekmez).
- **İlk kayıt olan kullanıcı `admin` olur.** Sonraki kayıtlar `customer` rolünü alır.

## 3. Tablolar

Ortak tablolar: `users`, `roles`, `permissions`, `model_has_roles`, `model_has_permissions`, `role_has_permissions`, `contact_messages`, `content_pages`, `audit_logs`.

| Kaynak (izin adı) | Tablo | Birincil anahtar | Sütunlar |
|---|---|---|---|
| `Urun` | `urunler` | `id` (otomatik artan) | ad*, adCeviri, kategori*, aciklama, gradyan, gorsel, boyutlar*, aktif, populer, sira |
| `Secenek` | `secenekler` | `id` (metin) | tur*, ad*, fiyatFarki, aktif |
| `Musteri` | `musteriler` | `id` (metin) | ad*, eposta*, siparisSayisi, durum, kayit |
| `Sube` | `subeler` | `id` (metin) | ad*, sehir*, saatler, acik |
| `Kampanya` | `kampanyalar` | `id` (metin) | ad*, tur*, deger*, kategoriler, baslangic*, bitis*, durum |
| `Siparis` | `siparisler` | `id` (metin) | musteriId, subeId, kalemler*, toplam*, durum, olusturma, iptalNedeni |

`*` zorunlu alan. JSON türündeki sütunlar (liste/nesne) metin olarak saklanır, okunurken nesneye çevrilir. Sütun adları, ön yüz tipleriyle birebir eşleşir.

## 4. Roller ve izinler

`admin` süper yöneticidir (atama gerekmez). Diğer roller aşağıdaki izinleri alır; `User`, `Role`, `Permission` kaynakları yalnız `admin`dedir.

| Rol | İzinler |
|---|---|
| `admin` | tümü (süper yönetici) |
| `barista` | `Siparis` (ViewAny, View, Update); `Urun` (okuma); `Secenek` (okuma); `Sube` (okuma); `Advance:Siparis` |
| `menu_manager` | `Urun` (tümü); `Secenek` (tümü); `Kampanya` (tümü); `Siparis` (okuma); `Sube` (okuma) |
| `support` | `ContactMessage` (tümü); `Siparis` (okuma); `Musteri` (okuma); `Cancel:Siparis` |
| `content` | `ContentPage` (tümü) |
| `viewer` | `Urun` (okuma); `Secenek` (okuma); `Musteri` (okuma); `Sube` (okuma); `Kampanya` (okuma); `Siparis` (okuma); `ContactMessage` (okuma); `ContentPage` (okuma) |
| `customer` | (uygulama kullanıcısı: kaynak izni yok; kendi kayıtlarını ilgili iş işleviyle yönetir) |

Özel yetenekler: `Advance:Siparis`, `Cancel:Siparis`, `Reveal:PII`.

Yeni izin/rol eklemek için yeni bir veri migration'ı yazılır (`INSERT OR IGNORE`); mevcut migration değiştirilmez.

## 5. Komutlar

Rust'taki `snake_case` parametreler JS tarafında `camelCase` yazılır. Aşağıdaki parametre adları JS adlarıdır.

### Ortak

| Komut | Parametreler | Ne yapar |
|---|---|---|
| `auth_register` | name, email, password | Kayıt (ilk kullanıcı admin olur; diğerleri varsayılan rolü alır) |
| `auth_login` | email, password | Giriş |
| `auth_logout` | — | Çıkış |
| `auth_me` | — | Oturumdaki kullanıcı + roller + izinler (yoksa null) |
| `user_index` | — | Kullanıcı listesi (`ViewAny:User`) |
| `user_store` | name, email, role, password? | Kullanıcı oluştur/güncelle + rol ata (`Create:User`) |
| `user_sync_roles` | userId, roles[] | Rolleri eşitle (`Update:User`) |
| `user_sync_permissions` | userId, permissions[] | Doğrudan izinleri eşitle (`Update:User`) |
| `user_set_status` | userId, status, reason? | active / suspended / banned (`Update:User`) |
| `role_index` | — | Roller + izinleri (`ViewAny:Role`) |
| `role_store` | name, permissions?[] | Rol oluştur (`Create:Role`) |
| `role_update` | name, newName | Rol adını değiştir (`Update:Role`) |
| `role_destroy` | name, reason? | Rol sil (`Delete:Role`) |
| `role_sync_permissions` | name, permissions[] | Rolün izinlerini eşitle (`Update:Role`) |
| `permission_index` | — | İzin listesi (`ViewAny:Permission`) |
| `permission_generate` | extraResources?[] | Kaynaklar için izinleri üret — `shield:generate` (`Create:Permission`) |
| `crud_resources` | — | Kaynak tanımları (tablo, sütun, süzgeç) |
| `crud_index` | resource, query? | Liste: arama + süzgeç + sıralama + sayfalama (`ViewAny:<Kaynak>`) |
| `crud_show` | resource, id | Tek kayıt (`View`) |
| `crud_store` | resource, data | Oluştur (`Create`) |
| `crud_update` | resource, id, data | Kısmi güncelle (`Update`) |
| `crud_destroy` | resource, id, reason? | Sil (`Delete`) |
| `crud_destroy_many` | resource, ids[], reason? | Toplu sil (`DeleteAny`) |
| `crud_aggregate` | resource, groupBy, query? | Gruplu sayım (panel grafikleri) |
| `db_migrate_status` | — | Migration durumu (yalnız admin) |
| `contact_submit` | name, email, subject?, message, lang? | İletişim formu (oturum gerekmez) |

### Bu uygulamaya özel

| Komut | Parametreler | Ne yapar |
|---|---|---|
| `urun_ara` | q, kategori | Ürün araması (oturum gerekmez) |
| `sepet_toplami` | girdi | Sepet toplamı (oturum gerekmez) |
| `siparis_ver` | girdi | — |
| `siparis_ilerlet` | id | Durumu bir adım ilerletir (alındı → hazırlanıyor → hazır → teslim). `Advance:Siparis` gerekir. |
| `siparis_iptal` | id, neden | İptal: müşteri yalnız "alındı" aşamasında kendi siparişini; `Cancel:Siparis` izni olan hazır olana kadar. Neden zorunlu. |
| `urun_fiyat_ayarla` | yuzde, kategori, kuruCalisma | Toplu fiyat artışı (`Update:Urun`): `yuzde` −50…+100; `kuru_calisma` true ise yalnız önizleme döner. Geçmiş siparişler değişmez. |
| `brewly_dashboard` | from, to | Panel özeti: sipariş sayısı, ciro, en çok satan 5 ürün, kategori ve boyut dağılımı, saatlik yoğunluk. |

## 6. Ön yüzden çağırma

```ts
import { invoke } from "@tauri-apps/api/core";

await invoke("auth_login", { email, password });
const sayfa = await invoke("crud_index", { resource: "Urun", query: { page: 1, pageSize: 25, search: "", filters: {}, sort: { key: "id", dir: "desc" } } });
// sayfa = { items, total, page, pageSize }
```

Hatalar `{ code, message, errors? }` olarak döner. `code`: `unauthenticated` (oturum yok), `forbidden` (izin yok), `not_found`, `validation` (`errors[]` alan bazlı), `conflict`, `db`, `internal`.

Süzgeç anahtarı bir sütun adıysa tam eşleşme, dizi ise `IN`; `<sütun>Min` / `<sütun>Max` aralık verir (yalnız `filters` listesindeki sütunlar).

## 7. Yeni tablo / kaynak ekleme

1. `migrations/` altına yeni SQL dosyası (`2026_MM_GG_NNNNNN_ad.sql`); `migrations.rs` listesinin **sonuna** ekle.
2. `schema.rs` içine `Entity` tanımı ekle ve `PROJECT` listesine koy (sütun adları TypeScript tipiyle aynı).
3. İzinler için yeni bir veri migration'ı: `INSERT OR IGNORE INTO permissions …` ve rol atamaları.
4. İş kuralı gerekiyorsa `domain.rs` içine işlev + `#[tauri::command]` sarmalayıcı yaz, `lib.rs` `generate_handler!` listesine ekle.
5. `cargo test` (`src-tauri/` içinde).

## 8. Testler

`cd src-tauri && cargo test` — bellek içi SQLite ile migration'ları, rol/izin mantığını, kaynak denetleyicisini, parola özetini ve bu uygulamanın iş kurallarını sınar.

## 9. Sınırlar (geçici çözüm)

- Tek kullanıcılı yerel oturum: giriş yapan kullanıcı uygulama belleğinde tutulur; uygulama kapanınca oturum biter.
- Veri cihazdadır; cihazlar arası eşitleme ve sunucu yoktur. Silinen uygulama verisiyle birlikte veritabanı da gider.
- Parolalar Argon2id ile özetlenir; yine de bu bir **eğitim ve geliştirme altyapısıdır**, gerçek kullanıcı verisi için denetlenmiş bir sunucu gerekir.
- İzin önbelleği yoktur; her denetim veritabanından okunur.
- Arama SQLite `LIKE` ile yapılır (Türkçe büyük/küçük harf duyarlılığı sınırlıdır).

