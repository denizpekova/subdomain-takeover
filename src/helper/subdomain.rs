use colored::*;
use hickory_resolver::TokioAsyncResolver;
use std::fs::read_to_string;

/// Reads a local wordlist or downloads SecLists Top 1 Million 5000 records dynamically,
/// then concurrently resolves subdomains against the provided target domain using `hickory_resolver`.
pub async fn run(target: String, source: String) -> anyhow::Result<()> {
    let resolver = TokioAsyncResolver::tokio_from_system_conf()?;

    let content = if source == "default_url" {
        let url = "https://raw.githubusercontent.com/danielmiessler/SecLists/master/Discovery/DNS/subdomains-top1million-5000.txt";
        let res = reqwest::get(url).await?.text().await?;
        println!(
            "{} Varsayılan wordlist başarıyla indirildi (5000 satır).",
            "[+]".green()
        );
        res
    } else {
        read_to_string(&source)
            .map_err(|e| anyhow::anyhow!("Wordlist okuma hatası ({}): {}", source, e))?
    };

    println!(
        "\n🌐 {} için Subdomain keşfi başlıyor...",
        target.bold().cyan()
    );
    let mut tasks = vec![];

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let subdomain = format!("{}.{}", line, target);
        let resolver = resolver.clone();

        tasks.push(tokio::spawn(async move {
            if let Ok(res) = resolver.lookup_ip(subdomain.clone()).await {
                if let Some(ip) = res.iter().next() {
                    println!("  [+] {} -> {}", subdomain.blue(), ip.to_string().yellow());
                }
            }
        }));
    }

    futures::future::join_all(tasks).await;

    println!("✅ Subdomain keşfi tamamlandı.\n");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_subdomain_run_invalid_file() {
        let res = run(
            "example.com".to_string(),
            "nonexistent_file_path.txt".to_string(),
        )
        .await;
        assert!(res.is_err(), "Should error on nonexistent local file");
    }
}
