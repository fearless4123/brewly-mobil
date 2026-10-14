//! Bu uygulamanın tabloları ve kaynak tanımları.
//! Sütun adları `src/` altındaki TypeScript tipleriyle ve tasarım belgelerindeki alan adlarıyla birebir aynıdır.
use crate::backend::crud::{col, req, Entity, Kind};
use crate::backend::error::{AppError, AppResult};
pub use crate::backend::schema_common::*;

/// Kayıt olan (ilk kullanıcı dışındaki) herkese verilen rol; `None` ise rolsüz kalır.
pub const DEFAULT_ROLE: Option<&str> = Some("customer");

pub static URUN: Entity = Entity {
    resource: "Urun",
    table: "urunler",
    pk: "id",
    pk_kind: Kind::Int,
    cols: &[
        req("ad", Kind::Text),
        col("adCeviri", Kind::Json),
        req("kategori", Kind::Text),
        col("aciklama", Kind::Text),
        col("gradyan", Kind::Text),
        col("gorsel", Kind::Text),
        req("boyutlar", Kind::Json),
        col("aktif", Kind::Bool),
        col("populer", Kind::Bool),
        col("sira", Kind::Int),
    ],
    search: &["ad", "aciklama"],
    filters: &["kategori", "aktif", "populer"],
    order_by: "\"sira\" ASC, \"id\" ASC",
    readonly: false,
};

pub static SECENEK: Entity = Entity {
    resource: "Secenek",
    table: "secenekler",
    pk: "id",
    pk_kind: Kind::Text,
    cols: &[
        req("tur", Kind::Text),
        req("ad", Kind::Json),
        col("fiyatFarki", Kind::Real),
        col("aktif", Kind::Bool),
    ],
    search: &[],
    filters: &["tur", "aktif"],
    order_by: "\"tur\" ASC, \"id\" ASC",
    readonly: false,
};

pub static MUSTERI: Entity = Entity {
    resource: "Musteri",
    table: "musteriler",
    pk: "id",
    pk_kind: Kind::Text,
    cols: &[
        req("ad", Kind::Text),
        req("eposta", Kind::Text),
        col("siparisSayisi", Kind::Int),
        col("durum", Kind::Text),
        col("kayit", Kind::Date),
    ],
    search: &["ad", "eposta"],
    filters: &["durum"],
    order_by: "\"kayit\" DESC",
    readonly: false,
};

pub static SUBE: Entity = Entity {
    resource: "Sube",
    table: "subeler",
    pk: "id",
    pk_kind: Kind::Text,
    cols: &[
        req("ad", Kind::Text),
        req("sehir", Kind::Text),
        col("saatler", Kind::Json),
        col("acik", Kind::Bool),
    ],
    search: &["ad", "sehir"],
    filters: &["sehir", "acik"],
    order_by: "\"ad\" ASC",
    readonly: false,
};

pub static KAMPANYA: Entity = Entity {
    resource: "Kampanya",
    table: "kampanyalar",
    pk: "id",
    pk_kind: Kind::Text,
    cols: &[
        req("ad", Kind::Text),
        req("tur", Kind::Text),
        req("deger", Kind::Real),
        col("kategoriler", Kind::Json),
        req("baslangic", Kind::Date),
        req("bitis", Kind::Date),
        col("durum", Kind::Text),
    ],
    search: &["ad"],
    filters: &["tur", "durum"],
    order_by: "\"baslangic\" DESC",
    readonly: false,
};

pub static SIPARIS: Entity = Entity {
    resource: "Siparis",
    table: "siparisler",
    pk: "id",
    pk_kind: Kind::Text,
    cols: &[
        col("musteriId", Kind::Text),
        col("subeId", Kind::Text),
        req("kalemler", Kind::Json),
        req("toplam", Kind::Real),
        col("durum", Kind::Text),
        col("olusturma", Kind::Date),
        col("iptalNedeni", Kind::Text),
    ],
    search: &["id"],
    filters: &["musteriId", "subeId", "durum", "olusturma", "toplam"],
    order_by: "\"olusturma\" DESC",
    readonly: false,
};

pub static PROJECT: &[&Entity] = &[&URUN, &SECENEK, &MUSTERI, &SUBE, &KAMPANYA, &SIPARIS];

/// Bütün kaynaklar: ortak + bu uygulamanınkiler.
pub fn all() -> Vec<&'static Entity> {
    let mut v: Vec<&'static Entity> = vec![&CONTACT_MESSAGE, &CONTENT_PAGE, &AUDIT_LOG];
    v.extend(PROJECT.iter().copied());
    v
}

pub fn entity(resource: &str) -> AppResult<&'static Entity> {
    all().into_iter().find(|e| e.resource == resource)
        .ok_or_else(|| AppError::NotFound(format!("`{resource}` adında bir kaynak yok.")))
}
