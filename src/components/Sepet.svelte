<script lang="ts">
  // Adım 10: Sepet sayfası — kalemleri listele, toplamı göster
  // Adım 12: "Siparişi oluştur" Rust'taki ürün_olustur komutunu çağırır
  import { tarihYaz, tl } from "$lib/data";
  import { sepet } from "$lib/sepet.svelte";
  import { ürünlerim } from "$lib/ürünler.svelte";

  let isleniyor = $state(false);

  async function odemeYap() {
    isleniyor = true;
    await ürünlerim.satinAl(sepet.kalemler);
    sepet.temizle();
    isleniyor = false;
    window.location.href = "/ürünlerim";
  }
</script>

<div class="sayfa">
  <h1>Sipariş Özeti</h1>

  {#each sepet.kalemler as k, i}
    <div class="kart kalem">
      <div class="bilgi">
        <strong>{k.etkinlik.baslik}</strong>
        <p>{tarihYaz(k.etkinlik.tarih)}</p>
        <p>{k.ürün.ad} · {k.adet} adet</p>
      </div>
      <div class="sag">
        <b>{tl(k.ürün.fiyat * k.adet)}</b>
        <button onclick={() => sepet.sil(i)} aria-label="Sil">🗑️</button>
      </div>
    </div>
  {:else}
    <p class="bos">Sepetiniz boş.<br /><a href="/">Kahvelere göz atın →</a></p>
  {/each}

  {#if sepet.kalemler.length > 0}
    <div class="kart ozet">
      <span>Toplam ({sepet.adet} ürün)</span>
      <b>{tl(sepet.toplam)}</b>
    </div>
    <button class="btn" onclick={odemeYap} disabled={isleniyor}>
      {isleniyor ? "İşleniyor..." : "Ödemeyi tamamla"}
    </button>
  {/if}
</div>

<style>
  h1 {
    margin: 0;
    font-size: 22px;
  }

  .kalem {
    display: flex;
    gap: 12px;
    padding: 14px;
  }

  .bilgi {
    flex: 1;
  }

  .bilgi p {
    margin: 2px 0;
    font-size: 14px;
    color: var(--yazi-soluk);
  }

  .sag {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    justify-content: space-between;
  }

  .sag button {
    border: 0;
    background: none;
    font-size: 18px;
    cursor: pointer;
  }

  .ozet {
    display: flex;
    justify-content: space-between;
    padding: 14px;
    font-size: 17px;
  }

  .bos a {
    color: var(--renk-ana);
    font-weight: 600;
  }
</style>
