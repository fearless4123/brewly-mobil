export type Kategori = "Kahve" | "Soğuk" | "Tatlı" | "Çay";

export interface BiletKategorisi {
  ad: string;
  fiyat: number;
}

export interface Etkinlik {
  id: number;
  baslik: string;
  kategori: Kategori;
  tarih: string;
  mekan: string;
  sehir: string;
  renk: string;
  aciklama: string;
  biletler: BiletKategorisi[];
}

export const kategoriler: Kategori[] = ["Kahve", "Soğuk", "Tatlı", "Çay"];

export const etkinlikler: Etkinlik[] = [
  { id: 1, baslik: "Latte", kategori: "Kahve", tarih: "2026-10-07T09:00", mekan: "Brewly", sehir: "İstanbul", renk: "linear-gradient(135deg, #6F452D, #3A2418)", aciklama: "Espresso ve süt köpüğünün dengeli buluşması. Günlük kahve molası için klasik seçim.", biletler: [{ ad: "Küçük", fiyat: 95 }, { ad: "Orta", fiyat: 115 }, { ad: "Büyük", fiyat: 135 }] },
  { id: 2, baslik: "Americano", kategori: "Kahve", tarih: "2026-10-07T09:00", mekan: "Brewly", sehir: "İstanbul", renk: "linear-gradient(135deg, #8B5E3C, #4A2C1A)", aciklama: "Yoğun espresso karakterini daha uzun içimle sunan sade bir kahve.", biletler: [{ ad: "Küçük", fiyat: 80 }, { ad: "Orta", fiyat: 95 }, { ad: "Büyük", fiyat: 110 }] },
  { id: 3, baslik: "Cappuccino", kategori: "Kahve", tarih: "2026-10-07T10:00", mekan: "Brewly", sehir: "İstanbul", renk: "linear-gradient(135deg, #C79A6E, #7A4A2B)", aciklama: "Yoğun espresso, süt ve kadifemsi köpükten oluşan dengeli tarif.", biletler: [{ ad: "Küçük", fiyat: 100 }, { ad: "Orta", fiyat: 120 }, { ad: "Büyük", fiyat: 140 }] },
  { id: 4, baslik: "Mocha", kategori: "Kahve", tarih: "2026-10-07T11:00", mekan: "Brewly", sehir: "İstanbul", renk: "linear-gradient(135deg, #7B3F00, #3A2418)", aciklama: "Espresso, çikolata ve sütü bir araya getiren tatlı kahve seçeneği.", biletler: [{ ad: "Küçük", fiyat: 110 }, { ad: "Orta", fiyat: 130 }, { ad: "Büyük", fiyat: 150 }] },
  { id: 5, baslik: "Iced Latte", kategori: "Soğuk", tarih: "2026-10-07T12:00", mekan: "Brewly", sehir: "İstanbul", renk: "linear-gradient(135deg, #D6B28A, #70503A)", aciklama: "Soğuk süt ve espresso ile ferahlatıcı, yumuşak içimli seçenek.", biletler: [{ ad: "Küçük", fiyat: 105 }, { ad: "Orta", fiyat: 125 }, { ad: "Büyük", fiyat: 145 }] },
  { id: 6, baslik: "Chai Latte", kategori: "Çay", tarih: "2026-10-07T13:00", mekan: "Brewly", sehir: "İstanbul", renk: "linear-gradient(135deg, #A97142, #5A321E)", aciklama: "Baharat aromaları ve sütle hazırlanan sıcak alternatif.", biletler: [{ ad: "Küçük", fiyat: 90 }, { ad: "Orta", fiyat: 110 }, { ad: "Büyük", fiyat: 130 }] },
  { id: 7, baslik: "San Sebastian", kategori: "Tatlı", tarih: "2026-10-07T14:00", mekan: "Brewly", sehir: "İstanbul", renk: "linear-gradient(135deg, #E6C79A, #9A6B3D)", aciklama: "Kahvenin yanına yakışan kremamsı cheesecake dilimi.", biletler: [{ ad: "Standart", fiyat: 145 }] },
];

export function etkinlikBul(id: number): Etkinlik | undefined {
  return etkinlikler.find((e) => e.id === id);
}

export const tl = (tutar: number) =>
  tutar.toLocaleString("tr-TR", { style: "currency", currency: "TRY", maximumFractionDigits: 0 });

export const tarihYaz = (iso: string) =>
  new Date(iso).toLocaleString("tr-TR", { weekday: "short", day: "numeric", month: "long", hour: "2-digit", minute: "2-digit" });
