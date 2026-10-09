# Brewly — Batch 01 İlerleme Matrisi

Görev 09'daki dokuz maddelik kontrol matrisinin repodaki karşılığı. Durum 09.10.2026 itibarıyladır.

> **Kim yaptı** sütunu önemlidir: "eğitmen" yazan satırlar ilk değerlendirmede (09.10.2026) eksik olduğu için eğitmen tarafından eklendi. Bu satırların puanı değerlendirmede (`docs/k1/01.review.md`) kesilmiş olarak kalır.

| No | Kontrol | Durum | Kanıt | Kim yaptı |
|---|---|---|---|---|
| 1 | Fork ve `keyvanarasteh` collaborator daveti | ✅ | GitHub: `hello-mobil` fork'u, eğitmen collaborator | öğrenci |
| 2 | Blackboard'a kullanıcı adı ve fork linki | ✅ | Form 07.10.2026 21:24 | öğrenci |
| 3 | Proje fikri ve üç ekran | ✅ | [`proje-fikri.md`](proje-fikri.md) | öğrenci |
| 4 | Kurumsal README | ✅ | [`README.md`](../README.md) | öğrenci |
| 5 | Üç ajan kural dosyası | ✅ | `AGENTS.md`, `CLAUDE.md`, `GEMINI.md` | öğrenci |
| 6 | Markalama ve `app.css` renkleri | ✅ | [`branding.md`](branding.md), `src/styles/app.css`; ikon seti ve favicon [PR #12](https://github.com/fearless4123/brewly-mobil/pull/12) | öğrenci; ikon seti ve favicon eğitmen |
| 7 | Bilgi sayfaları (4 dil, RTL) | ✅ | `src/pages/` altında 16 sayfa; dil rotaları [PR #13](https://github.com/fearless4123/brewly-mobil/pull/13) | sayfa içerikleri öğrenci; dil rotaları eğitmen |
| 8 | Mimari ağaç ve platform matrisi | ✅ | [`mimari-agac.md`](mimari-agac.md); rota düzeltmesi [PR #13](https://github.com/fearless4123/brewly-mobil/pull/13) | öğrenci; rota düzeltmesi eğitmen |
| 9 | `bun run build` 0 hata | ✅ | [`kanit/bun-run-build.txt`](kanit/bun-run-build.txt) | eğitmen |

## Derleme kanıtı

- `bun run build` çıktısı: [`kanit/bun-run-build.txt`](kanit/bun-run-build.txt) — **0 hata, 27 sayfa**.
- Ana sayfa önizlemesi (390×844): [`kanit/onizleme-anasayfa.png`](kanit/onizleme-anasayfa.png)
- Arapça Hakkında sayfası, sağdan sola: [`kanit/onizleme-ar-hakkinda.png`](kanit/onizleme-ar-hakkinda.png)
- İngilizce İletişim sayfası: [`kanit/onizleme-en-iletisim.png`](kanit/onizleme-en-iletisim.png)
- Üç sayfada da yatay taşma ve konsol hatası yok.
- `bun run tauri dev` ile masaüstü penceresinin ekran görüntüsü **eklenmedi**; yukarıdaki görüntüler derlenmiş arayüzün tarayıcı önizlemesidir. Pencere görüntüsünü öğrenci kendi bilgisayarında alıp `docs/kanit/` altına eklemelidir.

## Etiket

`v0.1.0-batch-01` etiketi öğrenci tarafından atıldı (09.10.2026). Etiket, eğitmenin eklediği PR'lardan önceki commit'i gösterir; PR'lar merge edildikten sonra etiketi yeniden atmak gerekmez.
