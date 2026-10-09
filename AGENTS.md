# AGENTS.md — Brewly

Bu belge, bu depoda çalışan tüm yapay zeka ajanları (Antigravity, Cursor, Claude Code, Gemini) için bağlayıcı sistem talimatlarını içerir.

## 1. Temel Proje Haritası ve Tek Kaynak Kuralı (DRY Docs)

Dokümantasyon tekrar edilmez, link edilir. Dizin ağaçları, renk tabloları, sayfa haritaları ve görev metinleri README veya ajan dosyalarına kopyalanmaz.

| Belge | Kapsam |
|---|---|
| [docs/branding.md](docs/branding.md) | Brewly marka kimliği, renk tokenları ve logo kuralları |
| [docs/klasor-mimarisi.md](docs/klasor-mimarisi.md) | Ana klasör ve dosya sorumlulukları |
| [docs/mimari-agac.md](docs/mimari-agac.md) | Sayfa, özellik, platform ve responsive haritası |
| [docs/proje-fikri.md](docs/proje-fikri.md) | Brewly konsepti, hedef kitle ve temel ekranlar |
| [docs/kurulum.md](docs/kurulum.md) | Kurulum ve çalıştırma yönergeleri |
| [docs/kurallar.md](docs/kurallar.md) | Git ve kodlama kuralları |
| [docs/kaynaklar.md](docs/kaynaklar.md) | Kaynak ve referans bağlantıları |
| [docs/teslim.md](docs/teslim.md) | Teslim kontrol ve kanıt notları |
| [docs/tasks/](docs/tasks/) | Eğitmen görevleri ve haftalık görev belgeleri |
| [docs/k1/01.review.md](docs/k1/01.review.md) | Eğitmen değerlendirme notu |
| [docs/indeks.md](docs/indeks.md) | Tüm proje dokümanlarının merkezi indeksi |

Yeni veya değişen dokümanlarda önce bu indeksi ve ilgili kaynak belgeyi kontrol et.

## 2. Teknoloji Yığını ve Komutlar

- **Platform:** Tauri v2 (Rust çekirdek + WebView)
- **Web:** Astro static output
- **Arayüz:** Svelte 5 Runes + React + MDX
- **Paket yöneticisi:** Bun

```text
bun run dev
bun run tauri dev
bun run build
```

Teslim öncesi `bun run build` 0 hata ile tamamlanmalıdır.

## 3. Geliştirme ve Git Kuralları

1. `master`/main dalına doğrudan geliştirme commit'i atılmaz.
2. Her iş için `feature/<isim>` veya `fix/<isim>` branch'i açılır.
3. Her iş Pull Request ile incelenip master'a merge edilir.
4. PR merge edilmeden önce değişiklik kapsamı ve build sonucu kontrol edilir.
5. İstenmeyen refactoring yapılmaz.

## 4. Kod ve UI Kuralları

- Yeni Svelte bileşenlerinde yalnızca Svelte 5 Runes (`$state`, `$derived`, `$props`) kullanılır.
- `window` ve `localStorage` erişimleri SSR-safe tutulur.
- Renkler `docs/branding.md` ve `src/styles/app.css` tokenları üzerinden kullanılır.
- Yeni rota veya özellik `docs/mimari-agac.md` ile uyumlu olmalıdır.
- İletişim, gizlilik ve koşullar sayfalarında TR/EN/AR/FA kapsamı korunmalı; AR/FA içerikleri `dir="rtl"` ile işaretlenmelidir.

## 5. Teslim Doğrulama

- `bun run build` → 0 hata.
- `bun run tauri dev` → Brewly masaüstü penceresi açılmalı.
- Final ZIP yalnızca master dalındaki son durumdan alınmalıdır.
- Git tag: `v0.1.0-batch-01`.
