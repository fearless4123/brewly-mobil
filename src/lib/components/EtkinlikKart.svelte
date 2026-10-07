<script lang="ts">
  import { type Etkinlik, tl } from "$lib/data";
  let { etkinlik }: { etkinlik: Etkinlik } = $props();
  const enUcuz = $derived(Math.min(...etkinlik.biletler.map((b) => b.fiyat)));
</script>
<a href="/etkinlik/{etkinlik.id}" class="kart etkinlik">
  <div class="gorsel" style:background={etkinlik.renk}><span>{etkinlik.kategori}</span></div>
  <div class="bilgi">
    <h3>{etkinlik.baslik}</h3>
    <p>{etkinlik.aciklama}</p>
    <strong>{tl(enUcuz)}'den başlayan</strong>
  </div>
</a>
<style>
.etkinlik{display:block;overflow:hidden}.gorsel{height:110px;padding:10px;display:flex;align-items:flex-start}.gorsel span{padding:4px 10px;border-radius:999px;background:#ffffffdd;color:#2B211B;font-size:12px;font-weight:600}.bilgi{padding:12px 14px 14px}h3{margin:0 0 6px;font-size:17px}p{margin:2px 0;font-size:14px;color:var(--yazi-soluk)}strong{display:block;margin-top:8px;font-size:14px;color:var(--renk-ana)}
</style>