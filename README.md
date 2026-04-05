# ⚡ Async Subdomain Takeover & Pentest Toolkit

Bu proje, Rust tabanlı, yüksek performanslı ve asenkron (`Tokio` altyapısı) çalışan kapsamlı bir ofansif güvenlik (pentest) CLI (Komut Satırı) aracıdır. Zafiyet taraması, port açıklarının tespiti ve DNS/Subdomain bilgi toplamayı tek bir etkileşimli menü üzerinden saniyeler içinde yapmanızı sağlar.

## 🌟 Ana Özellikler

### 1. 🚩 Subdomain Takeover Kontrolü
* Hedef domain veya subdomain'e HTTP isteği ayarak potansiyel zaafiyetleri inceler.
* Arkasında sahipsiz bırakılmış **CNAME** kayıtlarını ve üçüncü taraf bulut servislerini (*GitHub Pages, Amazon S3, Heroku, Vercel, Zendesk*) popüler "NXDOMAIN/Not Found" parmak izi okuması (fingerprint) ile saptar.
* Asılı kalan (dangling) domainleri bularak risk ihbarı yapar.

### 2. 🚀 Asenkron Port Tarayıcı
* Herhangi bir hedef domaine/IP'ye ait `1`'den `65535`'e kadar olan tüm TCP portlarını concurrent (eşzamanlı) mimaride saniyeler/dakikalar içerisinde tarar.
* Dışarıya veya internete açılmış kritik servisleri tespit eder.

### 3. 🔍 Subdomain Keşfi (Wordlist & Brute-Force)
* Hedef domain için belirlenen wordlist'teki kayıtlara asenkron DNS istekleri yollayarak gizli subdomain'leri açığa çıkartır.
* **Akıllı İndirme Modu:** Hazırda bir dosyanız yoksa dahi, araca talimat verip saniyeler içinde **SecLists'in GitHub deposundan** en popüler veri setini (`subdomains-top1million-5000.txt`) anlık çekip, doğrudan bellek üstünde operasyon başlatabilirsiniz.
* Yerel sisteminizden özel `.txt` dosyası aktarmak da mümkündür.

### 4. 📡 DNS Kayıt Keşfi (Record Enumerator)
* Modern ve hızlı hickory resolver kullanılarak asenkron DNS sorguları yapılır.
* Hedef alan adına ait altyapı ve yetki belirten şu kayıtları doğrudan döker:
    * **A / AAAA** (IPv4 & IPv6 yönlendirmeleri)
    * **MX** (E-posta/Mail Exchange sunucuları ve öncelikleri)
    * **NS** (Kök Ad Sunucuları)
    * **TXT** (SPF, DMARC politikaları ve diğer doğrulamalar)

### 5. 🛡️ HTTP Güvenlik Başlıkları Kontrolü
* Web uygulamalarının güvenlik duruşunu güçlendiren kritik HTTP Security başlıklarını kontrol eder (`Strict-Transport-Security`, `X-Frame-Options`, `Content-Security-Policy` vb.).
* Hem mevcut korumaları ekrana listeler hem de potansiyel güvenlik sıkıntıları doğurabilecek unutulumuş (eksik) başlıkları kırmızıyla uyarır.
* Ek olarak bilgi sızdıran `Server` başlıklarını ifşa eder.

---

## 🛠️ Teknolojiler Yığını (Tech Stack)

* **[Rust](https://www.rust-lang.org/):** Güvenli ve ışık hızında derlemeli dil.
* **[Tokio](https://tokio.rs/):** Yüksek seviye IO ve asenkron işlemler.
* **[Hickory DNS](https://hickory-dns.org/):** Modern asenkron DNS çözümleyici.
* **[Reqwest](https://docs.rs/reqwest/latest/reqwest/):** Asenkron HTTP İstemci altyapısı.
* **[Colored](https://docs.rs/colored/latest/colored/):** Terminal üzerindeki estetik, kullanıcı dostu çıktı.

---

## ⚙️ Kurulum ve Kullanım

Sisteminizde [Cargo ve Rust](https://rustup.rs/) kurulu olmalıdır.

1. Depoyu bilgisayarınıza çekin:
   ```bash
   git clone https://github.com/denizpekova/subdomain-takeover.git
   cd subdomain-takeover
   ```

2. Projeyi derleyip çalıştırın:
   ```bash
   cargo run
   ```

3. Ekrana gelecek numaralı ana menü üzerinden işleminizi seçin:
   ```txt
   === ANA MENÜ ===
   1 -> Subdomain Takeover Kontrolü
   2 -> Port Tarayıcı (Tüm Portlar - 1..65535)
   3 -> Subdomain Keşfi (Wordlist)
   4 -> DNS Kayıt Keşfi (A, MX, NS, TXT vb.)
   5 -> HTTP Güvenlik Başlıkları Kontrolü
   6 -> Çıkış
   ```

---

## ⚠️ Sorumluluk Reddi (Disclaimer)
Bu araç yalnızca **bilgi güvenliği çalışmaları**, **siber güvenlik uzmanlarının araştırmaları** ve **kendi sistemlerinizi zafiyetlere karşı test etmek** amacıyla tasarlanmıştır. Başka kişi, kurum veya kuruluşların sistemlerine önceden yazılı izin almadan uygulanması yasa dışıdır. Kötü niyetli kullanımlardan dolayı proje geliştiricisi hiçbir hukuki sorumluluk kabul etmez.

---

## 📜 Lisans
Bu proje **MIT Lisansı** altında lisanslanmıştır.
