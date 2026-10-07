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
├── /kahve/[id] (Kahve Detayı)
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
