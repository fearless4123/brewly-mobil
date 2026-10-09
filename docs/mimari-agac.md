# Brewly — Mimari Ağaç Yapısı ve Kapsam

## 1. Ana Dizin Mimarisi

Dizin ağacı yalnızca ana klasörleri ve kök yapılandırmaları gösterir:

```
hello-mobil/
├── package.json
├── astro.config.mjs
├── tsconfig.json
├── public/                 # Logo, favicon ve statik varlıklar
├── src-tauri/              # Rust/Tauri çekirdeği ve native ayarlar
├── src/
│   ├── layouts/            # Ortak sayfa iskeleti
│   ├── pages/              # Astro/MDX rota sayfaları
│   ├── components/         # Svelte/React UI bileşenleri
│   ├── lib/                # İş mantığı, state ve Rust çağrıları
│   ├── types/              # TypeScript tipleri
│   └── styles/             # Global tema ve tasarım tokenları
└── docs/                   # Proje dokümantasyonu ve görevler
```

## 2. Sayfa ve Özellik Ağacı

```
Brewly
├── / (Ana Sayfa)
│   ├── Kahve arama ve kategori filtreleme
│   └── Kahve listesi ve popüler ürünler
├── /etkinlik/[id] (Kahve Detayı)
│   ├── Boyut seçimi
│   ├── Süt seçimi
│   └── Ekstra shot / şurup ve siparişe ekleme
├── /sepet (Sipariş)
│   ├── Seçilen kahveler
│   └── Toplam tutar ve Rust sipariş kodu çağrısı
├── /biletlerim (Siparişlerim)
│   └── Rust tarafından üretilen BREW-XXXXXX kodları
├── /profil (Kullanıcı & Ayarlar)
│   └── Tema ve bilgi sayfalarına erişim
└── Bilgi Sayfaları
    ├── /hakkinda (MDX)
    ├── /iletisim (Reaktif Svelte formu)
    ├── /kosullar (MDX)
    └── /gizlilik (MDX)
```

**Rota adı notu:** kahve detayı ve siparişlerim ekranları şablondan gelen `/etkinlik/[id]` ve `/biletlerim` rotalarında çalışır. Rotalar `/kahve/[id]` ve `/siparislerim` olarak yeniden adlandırılırsa bu ağaç aynı PR'da güncellenir.

**Dil rotaları:** dört bilgi sayfası dört dilde yayınlanır. Türkçe kök rotadadır; diğer diller dil önekiyle açılır ve `<html>` etiketi dile göre `lang` ve `dir` alır.

| Dil | Önek | Örnek | Yön |
|---|---|---|---|
| Türkçe | yok | `/hakkinda` | `ltr` |
| English | `/en` | `/en/hakkinda` | `ltr` |
| العربية | `/ar` | `/ar/hakkinda` | `rtl` |
| فارسی | `/fa` | `/fa/hakkinda` | `rtl` |

Sayfalar arası dil geçişi `src/components/DilSecici.astro` ile yapılır.

## 3. Hedef Platform Matrisi

| Platform | Hedef Sistemler | Paket Formatı |
|---|---|---|
| Masaüstü | macOS (Apple Silicon / Intel) | .dmg, .app |
| Masaüstü | Windows 10 / 11 x64 | .msi, .exe |
| Masaüstü | Linux (Ubuntu / Debian) | .deb, .AppImage |
| Mobil | iOS (iPhone & iPad) | .ipa (Xcode) |
| Mobil | Android (Telefon & Tablet) | .apk, .aab |

Birincil test hedefi: **Windows 10 / 11**.

## 4. Responsive Breakpoints

- **Telefon 375–430px:** tek sütun; alt menü sabit.
- **Tablet 768–1024px:** iki sütunlu içerik/ızgara.
- **Masaüstü 1200px+:** üç sütunlu düzen ve ortalanmış `max-width`.
- **Büyük ekran:** içerik genişliği sınırlandırılır; kartlar gereksiz şekilde tam genişliğe yayılmaz.

## 5. Temel Veri Akışı

Kullanıcı → kahve listesi → kişiselleştirme → sepet → Rust sipariş kodu → sipariş sonucu → profil/sipariş geçmişi.
