use hickory_resolver::TokioAsyncResolver;
use reqwest::Client;
use std::time::Duration;

struct Fingerprint {
    service: &'static str,
    nxdomain_response: &'static str,
}

const FINGERPRINTS: &[Fingerprint] = &[
    Fingerprint {
        service: "GitHub Pages",
        nxdomain_response: "There isn't a GitHub Pages site here.",
    },
    Fingerprint {
        service: "Heroku",
        nxdomain_response: "No such app",
    },
    Fingerprint {
        service: "AWS S3",
        nxdomain_response: "NoSuchBucket",
    },
    Fingerprint {
        service: "Vercel",
        nxdomain_response: "404: NOT_FOUND",
    },
    Fingerprint {
        service: "Zendesk",
        nxdomain_response: "Help Center Closed",
    },
];

pub async fn check_takeover(domain: &str) {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("HTTP İstemcisi oluşturulamadı");

    let url = format!("http://{}", domain);

    match client.get(&url).send().await {
        Ok(response) => {
            if let Ok(text) = response.text().await {
                let mut vulnerable = false;
                for fp in FINGERPRINTS {
                    if text.contains(fp.nxdomain_response) {
                        println!("  [!!!] POTANSİYEL TAKEOVER BULUNDU! Servis: {}", fp.service);
                        vulnerable = true;
                    }
                }
                if !vulnerable {
                    println!(
                        "  [✓] Takeover zafiyeti tespit edilmedi (yanıt içeriği güvenli görünüyor)."
                    );
                }
            } else {
                println!("  [!] Sayfa okunamadı.");
            }
        }
        Err(e) => {
            let resolver = TokioAsyncResolver::tokio_from_system_conf().unwrap();
            match resolver.ipv4_lookup(domain).await {
                Ok(_) => {
                    println!(
                        "  [!] DNS çözümlendi ancak HTTP isteği başarısız oldu.\nHata: {}",
                        e
                    );
                }
                Err(_) => {
                    println!("  [!] DNS çözümlenemedi (NXDOMAIN). Potansiyel CNAME takeover! Manuel kontrol ediniz.");
                }
            }
        }
    }
}
