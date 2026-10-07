# AGENTS.md — Brewly

Bu belge, bu depoda çalışan tüm yapay zeka ajanları (Antigravity, Claude, Cursor, Gemini) için bağlayıcı geliştirme kurallarını içerir.

## 1. Dokümantasyon ve Tek Kaynak Kuralı (DRY Docs)

- Dokümanlar tekrarlanmaz, link edilir. Dizin ağaçları, marka kuralları ve sayfa haritaları başka dosyalara kopyalanmaz.
- Bağlayıcı belgeler:
  - [Klasör mimarisi](docs/klasor-mimarisi.md)
  - [Marka kimliği ve renkler](docs/branding.md)
  - [Sayfa ve özellik haritası](docs/mimari-agac.md)
  - [Proje konsepti](docs/proje-fikri.md)
- Yeni dosya ve sayfalar eklenirken bu belgelerdeki yapıya uyulur.

## 2. Teknoloji Yığını ve Çalıştırma

- **Çekirdek:** Tauri v2 (Rust) + Astro (Statik)
- **Arayüz:** Svelte 5, React bileşenleri ve MDX
- **Paket yöneticisi:** Bun
- **Geliştirme:** `bun run dev`
- **Tauri:** `bun run tauri dev`
- **Derleme:** `bun run build`

## 3. Git ve Geliştirme Disiplini

1. Doğrudan `master`/main dalına commit atılmaz.
2. Her özellik veya düzeltme için `feature/<ozellik-adi>` veya `fix/<hata-adi>` dalı açılır.
3. Değişiklikler PR üzerinden ana dala birleştirilir.
4. Teslimden önce `bun run build` ile 0 hata doğrulanır.
5. Yalnızca görevin gerektirdiği dosyalar değiştirilir; izinsiz büyük refactoring yapılmaz.

## 4. Kod Yazım Kuralları

- Svelte 5 Runes yaklaşımı kullanılır; Svelte 4'ün `export let` ve `$:` kalıpları kullanılmaz.
- SSR güvenliği korunur. `window` ve `localStorage` yalnızca client ortamında veya güvenli kontrollerle kullanılır.
- UI renkleri için `docs/branding.md` ve CSS değişkenleri kullanılır; rastgele renk değerleri eklenmez.
- Yeni sayfa veya özellik eklenirse [mimari ağaç](docs/mimari-agac.md) ile uyumu kontrol edilir.
