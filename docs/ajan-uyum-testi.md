# Brewly — Ajan Uyum Testi

Görev 08'in son ölçütü: bir yapay zeka ajanına bir renk ve bir sayfa görevi verilir, `AGENTS.md` kurallarına uyup uymadığı denetlenir.

> Bu test 09.10.2026'da **eğitmen tarafında** yapıldı; öğrencinin kendi aracıyla yaptığı bir test değildir. Öğrenci aynı testi kendi aracıyla tekrarlayıp sonucu bu dosyaya eklemelidir.

## Verilen görevler

| Görev | İstenen | Sonuç |
|---|---|---|
| Renk görevi | Dil seçici bağlantılarını marka renkleriyle biçimlendir | `DilSecici.astro` yalnız `--kenar`, `--renk-ana` ve `--zemin` değişkenlerini kullanıyor; bileşende sabit renk kodu yok |
| Sayfa görevi | Dört bilgi sayfasını dil başına ayrı rotaya böl | 16 sayfa üretildi; `docs/mimari-agac.md` aynı PR'da güncellendi; AR ve FA sayfaları `dir="rtl"` |

## Kural bazında denetim

| `AGENTS.md` kuralı | Uyuldu mu | Kanıt |
|---|---|---|
| `master`'a doğrudan commit atılmaz; her iş ayrı dalda ve PR ile | ✅ | `fix/05-platform-ikon-seti`, `feature/06-dil-rotalari`, `docs/09-batch-01-tamamlama` |
| Renkler `docs/branding.md` ve `app.css` tokenlarından | ✅ | Yeni bileşende sabit renk kodu yok |
| Yeni rota `docs/mimari-agac.md` ile uyumlu | ✅ | Dil rotaları tablosu eklendi |
| Bilgi sayfalarında TR/EN/AR/FA; AR/FA `dir="rtl"` | ✅ | `<html lang="ar" dir="rtl">` derlenmiş çıktıda doğrulandı |
| Svelte 5 Runes | ✅ | `IletisimFormu.svelte` `$props()` ve `$state` kullanıyor |
| Teslim öncesi `bun run build` 0 hata | ✅ | 27 sayfa |
| İstenmeyen refactoring yapılmaz | ✅ | Yalnız bilgi sayfaları, yerleşim ve iletişim formu değişti |

## Bulgular

- `src/styles/app.css` içindeki `.btn` kuralında metin rengi sabit (`#fff`). Koyu temada vurgu rengi açık olduğu için kontrast düşebilir; bir "vurgu üstü metin" değişkeni eklenmesi önerilir.
- `AGENTS.md` içinde dil rotalarının nasıl ekleneceği yazmıyor; "yeni bilgi sayfası dört dil rotasıyla birlikte eklenir" kuralı eklenebilir.
