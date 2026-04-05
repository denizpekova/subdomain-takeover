mod helper;

use anyhow::Result;
use colored::*;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<()> {
    loop {
        println!("\n{}", "=== ANA MENÜ ===".bold().blue());
        println!("1 -> Subdomain Takeover Kontrolü");
        println!("2 -> Port Tarayıcı (Tüm Portlar - 1..65535)");
        println!("3 -> Subdomain Keşfi (Wordlist)");
        println!("4 -> DNS Kayıt Keşfi (A, MX, NS, TXT vb.)");
        println!("5 -> HTTP Güvenlik Başlıkları Kontrolü");
        println!("6 -> Çıkış");

        print!("{} ", "Seçiminiz:".bold().yellow());
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                print!("{} ", "Hedef Domain (örn: sub.example.com):".bold().green());
                io::stdout().flush()?;

                let mut target_input = String::new();
                io::stdin().read_line(&mut target_input)?;
                let target = target_input.trim();

                if !target.is_empty() {
                    println!("\n[{}] {} için Takeover Kontrolü Başlatılıyor...", "+".cyan(), target.white());
                    helper::takeover::check_takeover(target).await;
                } else {
                    println!("{}", "HATA: Domain girmediniz!".red());
                }
            }
            "2" => {
                print!("{} ", "Hedef Adres (örn: example.com):".bold().green());
                io::stdout().flush()?;
                let mut target_input = String::new();
                io::stdin().read_line(&mut target_input)?;
                let target = target_input.trim();

                if target.is_empty() {
                    println!("{}", "HATA: Hedef girmediniz!".red());
                    continue;
                }

                println!("\n[{}] Tüm portların (1..65535) taraması başlatılıyor. Bu işlem vakit alabilir...", "+".cyan());
                
                // Artık aralık istemeden direkt taratıyoruz
                if let Err(e) = helper::scanner::scan_ports(target.to_string()).await {
                    println!("{} {}", "Tarama Hatası:".red(), e);
                }
            }
            "3" => {
                print!("{} ", "Hedef Domain (örn: example.com):".bold().green());
                io::stdout().flush()?;
                let mut target_input = String::new();
                io::stdin().read_line(&mut target_input)?;
                let target = target_input.trim().to_string();

                if target.is_empty() {
                    println!("{}", "HATA: Hedef domain girmediniz!".red());
                    continue;
                }

                print!("{} Wordlist'i internetten mi (SecLists 5000) çekelim, yoksa kendi dosyanızı mı kullanacaksınız?\n", "[-]".cyan());
                println!("1 -> İnternetten doğrudan çek (Tavsiye edilen)");
                println!("2 -> Kendi dosyamı gireceğim");
                print!("{} Seçiminiz: ", "=>".bold().yellow());
                io::stdout().flush()?;
                
                let mut wl_choice = String::new();
                io::stdin().read_line(&mut wl_choice)?;
                
                let wordlist_source = match wl_choice.trim() {
                    "1" => "default_url".to_string(),
                    "2" => {
                        print!("{} ", "Wordlist Dosya Yolu (örn: subdomains.txt):".bold().green());
                        io::stdout().flush()?;
                        let mut wordlist_input = String::new();
                        io::stdin().read_line(&mut wordlist_input)?;
                        let wordlist = wordlist_input.trim().to_string();
                        if wordlist.is_empty() {
                            println!("{}", "HATA: Dosya yolu girmediniz!".red());
                            continue;
                        }
                        wordlist
                    }
                    _ => {
                        println!("{}", "HATA: Geçersiz seçim!".red());
                        continue;
                    }
                };

                if let Err(e) = helper::subdomain::run(target, wordlist_source).await {
                    println!("{} {}", "Keşif Hatası:".red(), e);
                }
            }
            "4" => {
                print!("{} ", "Hedef Domain (örn: example.com):".bold().green());
                io::stdout().flush()?;
                let mut target_input = String::new();
                io::stdin().read_line(&mut target_input)?;
                let target = target_input.trim();

                if target.is_empty() {
                    println!("{}", "HATA: Hedef domain girmediniz!".red());
                    continue;
                }

                if let Err(e) = helper::dns::run(target).await {
                    println!("{} {}", "DNS Keşif Hatası:".red(), e);
                }
            }
            "5" => {
                print!("{} ", "Hedef Domain (örn: example.com):".bold().green());
                io::stdout().flush()?;
                let mut target_input = String::new();
                io::stdin().read_line(&mut target_input)?;
                let target = target_input.trim();

                if target.is_empty() {
                    println!("{}", "HATA: Hedef domain girmediniz!".red());
                    continue;
                }

                if let Err(e) = helper::header::run(target).await {
                    println!("{} {}", "Başlık Kontrol Hatası:".red(), e);
                }
            }
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
