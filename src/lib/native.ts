// Rust komutları yalnız bu dosya üzerinden çağrılır.

/** İzin durumunu okur; web'de (Rust yoksa) boş döner, asla diyalog açmaz. */
export async function izinDurumu(): Promise<import("./types").IzinDurumu[]> {
  if (!(typeof window !== "undefined" && "__TAURI_INTERNALS__" in window)) return [];
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<import("./types").IzinDurumu[]>("permissions_status");
}
/** Verilen izin için OS diyaloğunu açar, güncel durumu döner. */
export async function izinIste(id: string): Promise<import("./types").IzinDurumu[]> {
  if (!(typeof window !== "undefined" && "__TAURI_INTERNALS__" in window)) return [];
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<import("./types").IzinDurumu[]>("permission_request", { id });
}
/** Bildirim gönderir (izin verildiyse). */
export async function bildirimGonder(baslik: string, govde: string): Promise<void> {
  if (!(typeof window !== "undefined" && "__TAURI_INTERNALS__" in window)) return;
  const { invoke } = await import("@tauri-apps/api/core");
  await invoke("bildirim_gonder", { baslik, govde });
}
