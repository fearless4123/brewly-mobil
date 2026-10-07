# Brewly — Marka ve Tasarım Kılavuzu

## 1. Marka Renk Paleti

| Token | Açık (light) hex | Koyu (dark) hex | Kullanım yeri | Kontrast |
|---|---|---|---|---|
| `--renk-ana` | `#6F452D` | `#D09A73` | Butonlar, aktif sekmeler, marka vurgusu | Her temada uygun metin rengiyle ≥4.5:1 |
| `--renk-koyu` | `#3A2418` | `#241810` | Üst bar ve marka alanları | Yüksek kontrast |
| `--zemin` | `#FBF7F2` | `#17110D` | Sayfa zemini | Ana yazıyla ≥4.5:1 |
| `--kart` | `#FFFDFC` | `#241A14` | Kartlar ve form alanları | Ana yazıyla ≥4.5:1 |
| `--yazi` | `#2B211B` | `#FFF8F2` | Başlıklar ve gövde metni | Zemin üzerinde ≥4.5:1 |
| `--yazi-soluk` | `#6F625A` | `#B9AAA0` | Açıklamalar ve ikincil metin | Okunabilir ikincil metin |
| `--kenar` | `#E4D8CE` | `#49392F` | Kart ve input sınırları | Yüzey ayrımı |
| `--radius` | `14px` | `14px` | Ortak köşe yuvarlaklığı | — |

## 2. Tipografi

- **Yazı tipi:** System UI (`-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif`)
- **Köşe yuvarlaklığı:** `14px`
- Tasarım dili sıcak, sade ve okunabilirdir.

## 3. Logo ve İkon Konsepti

- **Marka:** Brewly
- **Sembol:** Stilize kahve çekirdeği ve buhar çizgileri.
- **Slogan:** Kahveni seç, kişiselleştir ve siparişini hızlıca oluştur.
- **Web logosu:** `public/brewly-logo.svg`
- **Favicon:** `public/brewly-favicon.svg`
- **Tauri launcher:** `src-tauri/icons/`

## 4. Tema Kuralları

Açık temada sıcak krem zemin ve kahverengi marka rengi; koyu temada koyu kahve zemin ve açık kahve vurgu rengi kullanılır. UI renkleri sabit hex değerler yerine CSS tokenları üzerinden kullanılır.
