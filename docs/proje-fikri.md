# Proje Fikri ve Konsept Belgesi

## 1. Proje Künyesi

- **Proje Adı:** Brewly
- **Slogan / Tek Cümlelik Tanım:** Kahveni seç, kişiselleştir ve siparişini hızlıca oluştur.
- **Öğrenci Adı Soyadı:** Yağız Genç
- **Öğrenci Numarası:** 2520191020
- **İlham Alınan Konsept / Platform:** Kahve Siparişi / Starbucks

## 2. Proje Amacı ve Çözülen Problem

Brewly, kullanıcıların kahve çeşitlerini inceleyip boyut, süt ve ekstra seçeneklerini belirleyerek kolayca sipariş oluşturmasını sağlayan bir masaüstü uygulamasıdır. Kullanıcı siparişini onayladığında Rust backend tarafından benzersiz bir sipariş kodu üretilir ve sipariş sonucu kullanıcıya gösterilir.

## 3. Temel Ekranlar ve İşlevler

1. **Ana Liste Ekranı (Keşfet):**
   - Espresso, Americano, Latte, Cappuccino ve Mocha gibi kahveler listelenir.
   - Arama ve kategori filtreleme bulunur.
   - Popüler kahveler ayrı bir bölümde gösterilir.

2. **Detay ve Seçim Ekranı:**
   - Kahvenin adı, açıklaması, fiyatı ve görseli gösterilir.
   - Küçük, orta ve büyük boy seçenekleri sunulur.
   - Normal, yulaf ve badem sütü gibi süt seçenekleri bulunur.
   - Ekstra shot ve şurup gibi ek seçenekler seçilebilir.
   - Kullanıcı ürünü sipariş listesine ekler.

3. **Kayıt / Kod Üretme Ekranı (Rust Backend):**
   - Seçilen kahveler ve toplam tutar gösterilir.
   - Kullanıcı siparişi onaylar.
   - Rust backend benzersiz bir sipariş kodu üretir. Örnek: `BREW-7K29XQ`.
   - Sipariş sonucu ve oluşturulan kod kullanıcıya gösterilir.

4. **Profil ve Ayarlar:**
   - Kullanıcının son siparişleri gösterilir.
   - Açık/koyu tema seçilebilir.
   - Uygulama dili TR, EN, AR ve FA olarak değiştirilebilir.

## 4. Hedef Kitle

Brewly; hızlı ve kişiselleştirilmiş kahve siparişi vermek isteyen öğrenciler, çalışanlar ve günlük kahve tüketen kullanıcılar için tasarlanmıştır.

## 5. Hedef Platform

- **Windows 10 / 11**
