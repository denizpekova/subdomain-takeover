use colored::*;
use hickory_resolver::TokioAsyncResolver;

/// Retrieves and prints public DNS records (A, AAAA, MX, NS, TXT) for the given target domain.
/// Makes use of TokioAsyncResolver for fast non-blocking UDP/TCP DNS queries.
pub async fn run(target: &str) -> anyhow::Result<()> {
    let resolver = TokioAsyncResolver::tokio_from_system_conf()?;

    println!("\n🌐 Querying DNS Records for {}...", target.bold().cyan());

    // A records (IPv4)
    if let Ok(res) = resolver.ipv4_lookup(target).await {
        for ip in res.iter() {
            println!(
                "  [{}] {} -> {}",
                "A".cyan(),
                target.blue(),
                ip.to_string().yellow()
            );
        }
    }

    // AAAA records (IPv6)
    if let Ok(res) = resolver.ipv6_lookup(target).await {
        for ip in res.iter() {
            println!(
                "  [{}] {} -> {}",
                "AAAA".cyan(),
                target.blue(),
                ip.to_string().yellow()
            );
        }
    }

    // MX Records
    if let Ok(res) = resolver.mx_lookup(target).await {
        for mx in res.iter() {
            println!(
                "  [{}] {} -> {} (Priority: {})",
                "MX".cyan(),
                target.blue(),
                mx.exchange().to_string().yellow(),
                mx.preference()
            );
        }
    }

    // NS Records
    if let Ok(res) = resolver.ns_lookup(target).await {
        for ns in res.iter() {
            println!(
                "  [{}] {} -> {}",
                "NS".cyan(),
                target.blue(),
                ns.to_string().yellow()
            );
        }
    }

    // TXT Records
    if let Ok(res) = resolver.txt_lookup(target).await {
        for txt in res.iter() {
            let texts: Vec<String> = txt
                .txt_data()
                .iter()
                .map(|d| String::from_utf8_lossy(d).to_string())
                .collect();
            let full_text = texts.join(" ");
            println!(
                "  [{}] {} -> {}",
                "TXT".cyan(),
                target.blue(),
                full_text.yellow()
            );
        }
    }

    println!("✅ DNS discovery completed.\n");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dns_lookup() {
        // Test with localhost which should fail gracefully and not panic
        let res = run("localhost").await;
        assert!(res.is_ok());
    }
}
