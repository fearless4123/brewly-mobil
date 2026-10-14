-- brewly tabloları. Sütun adları ön yüz tipleriyle birebir (camelCase); tablo adları snake_case.

CREATE TABLE "urunler" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "ad" TEXT NOT NULL,
    "adCeviri" TEXT,
    "kategori" TEXT NOT NULL CHECK ("kategori" IN ('Kahve', 'Soğuk', 'Tatlı', 'Çay')),
    "aciklama" TEXT NOT NULL DEFAULT '',
    "gradyan" TEXT NOT NULL DEFAULT '',
    "gorsel" TEXT,
    "boyutlar" TEXT NOT NULL,
    "aktif" INTEGER NOT NULL DEFAULT 1 CHECK ("aktif" IN (0, 1)),
    "populer" INTEGER NOT NULL DEFAULT 0 CHECK ("populer" IN (0, 1)),
    "sira" INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX urunler_kategori_index ON "urunler" ("kategori");

CREATE TABLE "secenekler" (
    "id" TEXT PRIMARY KEY,
    "tur" TEXT NOT NULL CHECK ("tur" IN ('sut', 'ekstra')),
    "ad" TEXT NOT NULL,
    "fiyatFarki" REAL NOT NULL DEFAULT 0.0,
    "aktif" INTEGER NOT NULL DEFAULT 1 CHECK ("aktif" IN (0, 1))
);

CREATE TABLE "musteriler" (
    "id" TEXT PRIMARY KEY,
    "ad" TEXT NOT NULL,
    "eposta" TEXT NOT NULL,
    "siparisSayisi" INTEGER NOT NULL DEFAULT 0,
    "durum" TEXT NOT NULL DEFAULT 'aktif' CHECK ("durum" IN ('aktif', 'engelli')),
    "kayit" TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE TABLE "subeler" (
    "id" TEXT PRIMARY KEY,
    "ad" TEXT NOT NULL,
    "sehir" TEXT NOT NULL,
    "saatler" TEXT NOT NULL DEFAULT '[]',
    "acik" INTEGER NOT NULL DEFAULT 1 CHECK ("acik" IN (0, 1))
);

CREATE TABLE "kampanyalar" (
    "id" TEXT PRIMARY KEY,
    "ad" TEXT NOT NULL,
    "tur" TEXT NOT NULL CHECK ("tur" IN ('yuzde', 'tutar', 'n-al-m-ode')),
    "deger" REAL NOT NULL,
    "kategoriler" TEXT,
    "baslangic" TEXT NOT NULL,
    "bitis" TEXT NOT NULL,
    "durum" TEXT NOT NULL DEFAULT 'taslak' CHECK ("durum" IN ('taslak', 'aktif', 'bitti'))
);

CREATE TABLE "siparisler" (
    "id" TEXT PRIMARY KEY,
    "musteriId" TEXT REFERENCES "musteriler" ("id") ON DELETE SET NULL,
    "subeId" TEXT REFERENCES "subeler" ("id") ON DELETE SET NULL,
    "kalemler" TEXT NOT NULL,
    "toplam" REAL NOT NULL,
    "durum" TEXT NOT NULL DEFAULT 'alindi' CHECK ("durum" IN ('alindi', 'hazirlaniyor', 'hazir', 'teslim', 'iptal')),
    "olusturma" TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    "iptalNedeni" TEXT
);
CREATE INDEX siparisler_durum_index ON "siparisler" ("durum");
CREATE INDEX siparisler_olusturma_index ON "siparisler" ("olusturma");
