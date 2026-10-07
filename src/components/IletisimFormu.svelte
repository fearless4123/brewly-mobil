<script lang="ts">
  let dil = $state<"tr"|"en"|"ar"|"fa">("tr");
  let gonderildi = $state(false);
  const metin = {
    tr:{name:"Ad Soyad",email:"E-posta",subject:"Konu",message:"Mesaj",send:"Mesajı gönder",done:"Mesajınız iletildi."},
    en:{name:"Full name",email:"Email",subject:"Subject",message:"Message",send:"Send message",done:"Your message has been sent."},
    ar:{name:"الاسم الكامل",email:"البريد الإلكتروني",subject:"الموضوع",message:"الرسالة",send:"إرسال الرسالة",done:"تم إرسال رسالتك."},
    fa:{name:"نام و نام خانوادگی",email:"ایمیل",subject:"موضوع",message:"پیام",send:"ارسال پیام",done:"پیام شما ارسال شد."}
  };
  function gonder(e: SubmitEvent){ e.preventDefault(); gonderildi=true; (e.currentTarget as HTMLFormElement).reset(); }
</script>
<div class="form-wrap" dir={dil === "ar" || dil === "fa" ? "rtl" : "ltr"}>
  <div class="diller">
    {#each ["tr","en","ar","fa"] as d}
      <button type="button" class:aktif={dil===d} onclick={() => {dil=d as typeof dil; gonderildi=false;}}>{d.toUpperCase()}</button>
    {/each}
  </div>
  <form onsubmit={gonder}>
    <label>{metin[dil].name}<input name="name" required /></label>
    <label>{metin[dil].email}<input name="email" type="email" required /></label>
    <label>{metin[dil].subject}<input name="subject" required /></label>
    <label>{metin[dil].message}<textarea name="message" rows="5" required></textarea></label>
    <button class="btn" type="submit">{metin[dil].send}</button>
  </form>
  {#if gonderildi}<p class="ok" role="status">✓ {metin[dil].done}</p>{/if}
</div>
<style>
.form-wrap{display:flex;flex-direction:column;gap:16px}.diller{display:flex;gap:8px}.diller button{border:1px solid var(--kenar);background:var(--kart);border-radius:10px;padding:7px 10px}.diller .aktif{background:var(--renk-ana);color:#fff}form{display:flex;flex-direction:column;gap:14px}label{display:flex;flex-direction:column;gap:6px;font-weight:600;font-size:14px}input,textarea{padding:12px;border:1px solid var(--kenar);border-radius:10px;background:var(--zemin)}.ok{padding:12px;border-radius:10px;background:var(--kart);border:1px solid var(--kenar);font-weight:600}
</style>