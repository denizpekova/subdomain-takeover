# 📖 Proje Gelişim Geçmişi (Project History)

Bu belge, **Subdomain Takeover & Pentest Toolkit** aracının geliştirilme sürecini ve yapısal evrimini takip etmek amacıyla oluşturulmuştur.

## 📌 Sürüm & Geliştirme Notları

### 🚀 Nisan 2026 - v1.1.0 (Modernizasyon & Kalite Güvencesi)

* **Mimari Revizyon:** Proje analizleri sonucunda `main.rs`'teki devasa bloklar, yardımcı fonksiyon kollarına (handler) ayrılarak *Clean Code (Temiz Kod)* prensiplerine uygun hale getirildi. Fonksiyonların ortalama satır sayısı ciddi seviyede optimize edildi.
* **Kalite ve Test (QA):** Eksikliği değerlendirmede belirtilen `#[tokio::test]` birim (unit) testleri modüllere dâhil edildi.
* **Geliştirici Deneyimi (DX):** Kod okunabilirlik oranını (%3) artırmak amacıyla tüm dosyalara resmi **Rustdoc** (`///`) standartlarında işlevsel açıklamalar sağlandı.
* **Altyapı & Dağıtım (DevOps):**
  * Projenin cihaz bağımsız ve izole çalıştırılabilmesi için 'Multi-Stage Rust' kurallarına göre bir modern `Dockerfile` yazıldı.
  * Bağımlılıkların, iş parçacığı limitlerinin ve anahtar ortamlarının tutulması adına `.env.example` şablonu oluşturuldu.
  * Satır sonu kargaşalarını önleme amaçlı standart `.gitattributes` getirildi.
* **Otomasyon (CI/CD):** Projeye dahil olan `clippy` (Linter) ve `rustfmt` formatlama denetimlerinin her *Push* (PR) işleminde otomatik test edilmesi amacıyla, entegre `.github/workflows/ci.yml` **GitHub Actions** dizini yapılandırıldı.
* **Dokümantasyon:** Projenin akademik ciddiyetine uygun olması için *İstinye Üniversitesi*, Bilişim Güvenliği Teknolojisi formatı ve danışman bilgileri (**Keyvan Arasteh**) entegre edildi. Belgelendirmeye eklenen `TOC (İçindekiler)` ve akıllı demo altyapısı ile kalite standartları artırıldı.

---

### 🌱 Nisan 2026 - v1.0.0 (İlk Prototip & Çekirdek Inşası)

* **Çekirdek Sistem (Core):** Rust ile `cargo` ortamı kullanılarak proje başlatıldı. (`Initial commit` & `proje-file-created`)
* **Alt Yardımcı Servisler (Helpers):** Zafiyet keşfi yapan çekirdek asenkron Rust modülleri kodlandı:
  * Asılı/Zafiyetli etki alanları taraması.
  * Güvenli asenkron DNS (hickory-resolver tabanlı) çözümleri.
  * Güvenlik Başlıkları, Port TCP tarama algoritması ve Wordlist Brute-force aktarımı eklendi.
* **Terminal Arayüzü:** `main.rs` yapısının asenkron loop bağlantısıyla standart (1-6) terminal arayüzüne kavuşması. (`main.rs modified and async add`).

---

> *Projenin ilerleyen süreçte siber güvenlik literatürüne ve açık kaynak standartlarına uygunluğu artırılarak yola devam edilmesi hedeflenmektedir.*
