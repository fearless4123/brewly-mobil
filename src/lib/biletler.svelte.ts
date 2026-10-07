import { invoke, isTauri } from "@tauri-apps/api/core";
import type { SepetKalemi } from "./sepet.svelte";

export interface Bilet {
  kod: string;
  baslik: string;
  tarih: string;
  mekan: string;
  kategori: string;
  adet: number;
}
const ANAHTAR = "siparislerim";
function yukle(): Bilet[] {
  try { return typeof localStorage === "undefined" ? [] : JSON.parse(localStorage.getItem(ANAHTAR) ?? "[]"); }
  catch { return []; }
}
async function siparisKoduAl(): Promise<string> {
  if (typeof window !== "undefined" && isTauri()) return invoke<string>("siparis_olustur");
  return `BREW-WEB-${Date.now().toString(36).toUpperCase()}`;
}
class Siparislerim {
  liste = $state<Bilet[]>(yukle());
  async satinAl(kalemler: SepetKalemi[]) {
    for (const k of kalemler) {
      const kod = await siparisKoduAl();
      this.liste.unshift({ kod, baslik:k.etkinlik.baslik, tarih:k.etkinlik.tarih, mekan:k.etkinlik.mekan, kategori:k.bilet.ad, adet:k.adet });
    }
    if (typeof localStorage !== "undefined") localStorage.setItem(ANAHTAR, JSON.stringify(this.liste));
  }
}
export const biletlerim = new Siparislerim();
