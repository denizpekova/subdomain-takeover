use hickory_resolver::TokioAsyncResolver;
use colored::*;

pub async fn run(target: &str) -> anyhow::Result<()> {
    let resolver = TokioAsyncResolver::tokio_from_system_conf()?;
    
    println!("\n🌐 {} için DNS Kayıtları Sorgulanıyor...", target.bold().cyan());

    // A kayıtları (IPv4)
    if let Ok(res) = resolver.ipv4_lookup(target).await {
        for ip in res.iter() {
            println!("  [{}] {} -> {}", "A".cyan(), target.blue(), ip.to_string().yellow());
        }
    }

    // AAAA kayıtları (IPv6)
    if let Ok(res) = resolver.ipv6_lookup(target).await {
        for ip in res.iter() {
            println!("  [{}] {} -> {}", "AAAA".cyan(), target.blue(), ip.to_string().yellow());
        }
    }

    // MX Kayıtları
    if let Ok(res) = resolver.mx_lookup(target).await {
        for mx in res.iter() {
            println!("  [{}] {} -> {} (Öncelik: {})", "MX".cyan(), target.blue(), mx.exchange().to_string().yellow(), mx.preference());
        }
    }

    // NS Kayıtları
    if let Ok(res) = resolver.ns_lookup(target).await {
        for ns in res.iter() {
            println!("  [{}] {} -> {}", "NS".cyan(), target.blue(), ns.to_string().yellow());
        }
    }

    // TXT Kayıtları
    if let Ok(res) = resolver.txt_lookup(target).await {
        for txt in res.iter() {
            let texts: Vec<String> = txt.txt_data().iter().map(|d| String::from_utf8_lossy(d).to_string()).collect();
            let full_text = texts.join(" ");
            println!("  [{}] {} -> {}", "TXT".cyan(), target.blue(), full_text.yellow());
        }
    }
    
    println!("✅ DNS keşfi tamamlandı.\n");
    Ok(())
}
