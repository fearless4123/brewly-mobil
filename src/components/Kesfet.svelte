<script lang="ts">
  import EtkinlikKart from "$lib/components/EtkinlikKart.svelte";
  import { etkinlikler, kategoriler, type Kategori } from "$lib/data";

  const filtreler: (Kategori | "Tümü")[] = ["Tümü", ...kategoriler];
  let secili = $state<Kategori | "Tümü">("Tümü");
  let arama = $state("");
  const liste = $derived(etkinlikler.filter((e) =>
    (secili === "Tümü" || e.kategori === secili) &&
    e.baslik.toLocaleLowerCase("tr").includes(arama.toLocaleLowerCase("tr"))
  ));
</script>

<div class="sayfa">
  <input class="arama" type="search" placeholder="Kahve veya tatlı ara" bind:value={arama} />
  <div class="cipler">
    {#each filtreler as k}
      <button class:aktif={secili === k} onclick={() => (secili = k)}>{k}</button>
    {/each}
  </div>
  {#each liste as e (e.id)}
    <EtkinlikKart etkinlik={e} />
  {:else}
    <p class="bos">Aramana uygun ürün bulunamadı.</p>
  {/each}
</div>

<style>
.arama{width:100%;padding:12px 14px;border:1px solid var(--kenar);border-radius:var(--radius);background:var(--kart)}
.cipler{display:flex;gap:8px;overflow-x:auto;scrollbar-width:none}.cipler button{flex-shrink:0;padding:8px 14px;border:1px solid var(--kenar);border-radius:999px;background:var(--kart);font-size:13px;color:var(--yazi-soluk)}.cipler button.aktif{background:var(--renk-ana);border-color:var(--renk-ana);color:#fff;font-weight:600}
</style>