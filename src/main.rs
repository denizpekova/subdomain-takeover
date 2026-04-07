mod helper;

use anyhow::Result;
use colored::*;
use std::io::Write;
use tokio::io::{self, AsyncBufReadExt};

/// Prompt the user for input and read it from stdin asynchronously.
async fn get_input(prompt: &str, stdin: &mut io::BufReader<io::Stdin>) -> Result<String> {
    print!("{}", prompt);
    std::io::stdout().flush()?;

    let mut buffer = String::new();
    stdin.read_line(&mut buffer).await?;
    Ok(buffer.trim().to_string())
}

/// Helper function to handle subdomain takeover scanning from the main menu.
async fn handle_takeover(stdin: &mut io::BufReader<io::Stdin>) -> Result<()> {
    let target = get_input(
        &format!("{} ", "Hedef Domain (örn: sub.example.com):".bold().green()),
        stdin,
    )
    .await?;
    if !target.is_empty() {
        println!(
            "\n[{}] {} için Takeover Kontrolü Başlatılıyor...",
            "+".cyan(),
            target.white()
        );
        helper::takeover::check_takeover(&target).await;
    } else {
        println!("{}", "HATA: Domain girmediniz!".red());
    }
    Ok(())
}

/// Helper function to handle async port scanning from the main menu.
async fn handle_port_scan(stdin: &mut io::BufReader<io::Stdin>) -> Result<()> {
    let target = get_input(
        &format!("{} ", "Hedef Adres (örn: example.com):".bold().green()),
        stdin,
    )
    .await?;
    if target.is_empty() {
        println!("{}", "HATA: Hedef girmediniz!".red());
        return Ok(());
    }
    println!(
        "\n[{}] Tüm portların (1..65535) taraması başlatılıyor. Bu işlem vakit alabilir...",
        "+".cyan()
    );
    if let Err(e) = helper::scanner::scan_ports(target).await {
        println!("{} {}", "Tarama Hatası:".red(), e);
    }
    Ok(())
}

/// Helper function to handle subdomain discovery using a wordlist.
async fn handle_subdomain_discovery(stdin: &mut io::BufReader<io::Stdin>) -> Result<()> {
    let target = get_input(
        &format!("{} ", "Hedef Domain (örn: example.com):".bold().green()),
        stdin,
    )
    .await?;
    if target.is_empty() {
        println!("{}", "HATA: Hedef domain girmediniz!".red());
        return Ok(());
    }
    println!(
        "{} Wordlist'i internetten mi (SecLists 5000) çekelim, yoksa kendi dosyanızı mı kullanacaksınız?",
        "[-]".cyan()
    );
    println!("1 -> İnternetten doğrudan çek (Tavsiye edilen)");
    println!("2 -> Kendi dosyamı gireceğim");

    let wl_choice = get_input(&format!("{} Seçiminiz: ", "=>".bold().yellow()), stdin).await?;
    let wordlist_source = match wl_choice.as_str() {
        "1" => "default_url".to_string(),
        "2" => {
            let wordlist = get_input(
                &format!(
                    "{} ",
                    "Wordlist Dosya Yolu (örn: subdomains.txt):".bold().green()
                ),
                stdin,
            )
            .await?;
            if wordlist.is_empty() {
                println!("{}", "HATA: Dosya yolu girmediniz!".red());
                return Ok(());
            }
            wordlist
        }
        _ => {
            println!("{}", "HATA: Geçersiz seçim!".red());
            return Ok(());
        }
    };
    if let Err(e) = helper::subdomain::run(target, wordlist_source).await {
        println!("{} {}", "Keşif Hatası:".red(), e);
    }
    Ok(())
}

/// Helper function to handle DNS record lookup operations.
async fn handle_dns_discovery(stdin: &mut io::BufReader<io::Stdin>) -> Result<()> {
    let target = get_input(
        &format!("{} ", "Hedef Domain (örn: example.com):".bold().green()),
        stdin,
    )
    .await?;
    if target.is_empty() {
        println!("{}", "HATA: Hedef domain girmediniz!".red());
        return Ok(());
    }
    if let Err(e) = helper::dns::run(&target).await {
        println!("{} {}", "DNS Keşif Hatası:".red(), e);
    }
    Ok(())
}

/// Helper function to deal with HTTP security headers checking.
async fn handle_header_check(stdin: &mut io::BufReader<io::Stdin>) -> Result<()> {
    let target = get_input(
        &format!("{} ", "Hedef Domain (örn: example.com):".bold().green()),
        stdin,
    )
    .await?;
    if target.is_empty() {
        println!("{}", "HATA: Hedef domain girmediniz!".red());
        return Ok(());
    }
    if let Err(e) = helper::header::run(&target).await {
        println!("{} {}", "Başlık Kontrol Hatası:".red(), e);
    }
    Ok(())
}

/// Main entry point for the Async Pentest Toolkit.
#[tokio::main]
async fn main() -> Result<()> {
    let mut stdin = io::BufReader::new(io::stdin());

    loop {
        println!("\n{}", "=== ANA MENÜ ===".bold().blue());
        println!("1 -> Subdomain Takeover Kontrolü");
        println!("2 -> Port Tarayıcı (Tüm Portlar - 1..65535)");
        println!("3 -> Subdomain Keşfi (Wordlist)");
        println!("4 -> DNS Kayıt Keşfi (A, MX, NS, TXT vb.)");
        println!("5 -> HTTP Güvenlik Başlıkları Kontrolü");
        println!("6 -> Çıkış");

        let choice = get_input(&format!("{} ", "Seçiminiz:".bold().yellow()), &mut stdin).await?;

        match choice.as_str() {
            "1" => handle_takeover(&mut stdin).await?,
            "2" => handle_port_scan(&mut stdin).await?,
            "3" => handle_subdomain_discovery(&mut stdin).await?,
            "4" => handle_dns_discovery(&mut stdin).await?,
            "5" => handle_header_check(&mut stdin).await?,
            "6" | "q" | "quit" | "exit" => {
                println!("{}", "Çıkış yapılıyor. İyi çalışmalar!".magenta());
                break;
            }
            _ => {
                println!("{}", "HATA: Geçersiz seçim, lütfen tekrar deneyin.".red());
            }
        }
    }

    Ok(())
}
