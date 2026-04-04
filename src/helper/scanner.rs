use anyhow::Result;
use colored::*;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use tokio::time::{timeout, Duration};

pub async fn scan_ports(target: String) -> Result<()> {
    // 0'dan değil, 1'den başlayarak 65535'e kadar (Tüm Network Portları)
    let start: u16 = 1;
    let end: u16 = 65535;
    
    // Taramayı inanılmaz hızlandırmak için aynı anda en fazla 500 soket bağlantısı yapmasına izin veriyoruz
    let sem = Arc::new(Semaphore::new(500));

    let mut tasks = vec![];

    for port in start..=end {
        let target = target.clone();
        
        let permit = sem.clone().acquire_owned().await?;
        
        tasks.push(tokio::spawn(async move {
            let _permit = permit;
            let addr = format!("{}:{}", target, port);
            
            // Timeout süresini 800 yerine 600 ms'ye indirdik, böylece boştaki port geçişleri hızlandı
            if timeout(Duration::from_millis(600), TcpStream::connect(&addr)).await.is_ok() {
                println!("  [+] Port {} {}", port, "AÇIK".green());
            }
        }));
    }
    
    futures::future::join_all(tasks).await;
    
    println!("{}", "[✓] Tüm portların tarama işlemi tamamlandı.".magenta());
    Ok(())
}
