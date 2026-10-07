// Adım 12: Satın alınan biletler — Rust'tan kod alır, localStorage'a kaydeder
import { invoke, isTauri } from "@tauri-apps/api/core";
import type { SepetKalemi } from "./sepet.svelte";

export interface Siparis {
  kod: string;
  baslik: string;
  tarih: string;
  mekan: string;
  kategori: string;
  adet: number;
}

const ANAHTAR = "siparislerim";

function yukle(): Siparis[] {
  try {
    if (typeof localStorage === "undefined") return [];
    return JSON.parse(localStorage.getItem(ANAHTAR) ?? "[]");
  } catch {
    return [];
  }
}

// Tauri içinde çalışıyorsak Rust komutunu çağır, tarayıcıda ise JS ile üret
async function biletKoduAl(): Promise<string> {
  if (typeof window !== "undefined" && isTauri()) {
    // Rust tarafındaki etkinlik_id parametresi JS'te camelCase yazılır: etkinlikId
    return invoke<string>("siparis_olustur");
  }
  return `BREW-WEB-${Date.now().toString(36).toUpperCase()}`;
}

class Siparislerim {
  liste = $state<Siparis[]>(yukle());

  async satinAl(kalemler: SepetKalemi[]) {
    for (const k of kalemler) {
      const kod = await biletKoduAl();
      this.liste.unshift({
        kod,
        baslik: k.etkinlik.baslik,
        tarih: k.etkinlik.tarih,
        mekan: k.etkinlik.mekan,
        kategori: k.bilet.ad,
        adet: k.adet,
      });
    }
    if (typeof localStorage !== "undefined") {
      localStorage.setItem(ANAHTAR, JSON.stringify(this.liste));
    }
  }
}

export const siparislerim = new Siparislerim();
