use hickory_resolver::TokioAsyncResolver;
use reqwest::Client;
use std::time::Duration;

/// Representation of a known SaaS fingerprint for takeover analysis.
struct Fingerprint {
    service: &'static str,
    nxdomain_response: &'static str,
}

/// Known fingerprint signatures list.
const FINGERPRINTS: &[Fingerprint] = &[
    Fingerprint {
        service: "GitHub Pages",
        nxdomain_response: "There isn't a GitHub Pages site here.",
    },
    Fingerprint {
        service: "Heroku",
        nxdomain_response: "No securing app",
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

/// Checks the possibility of a Subdomain Takeover by comparing HTTP responses 
/// with known cloud service provider error texts (fingerprints).
/// Will also fallback to checking DNS resolution if HTTP fails.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_takeover_check() {
        // Try against a known invalid domain. It shouldn't panic.
        check_takeover("invalid-test-domain-123456.local").await;
        // If it reaches here without panic, test passes
        assert!(true);
    }
}
