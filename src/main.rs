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
        &format!("{} ", "Target Domain (e.g. sub.example.com):".bold().green()),
        stdin,
    )
    .await?;
    if !target.is_empty() {
        println!(
            "\n[{}] Starting Takeover Check for {}...",
            "+".cyan(),
            target.white()
        );
        helper::takeover::check_takeover(&target).await;
    } else {
        println!("{}", "ERROR: You didn't enter a domain!".red());
    }
    Ok(())
}

/// Helper function to handle async port scanning from the main menu.
async fn handle_port_scan(stdin: &mut io::BufReader<io::Stdin>) -> Result<()> {
    let target = get_input(
        &format!("{} ", "Target Address (e.g. example.com):".bold().green()),
        stdin,
    )
    .await?;
    if target.is_empty() {
        println!("{}", "ERROR: You didn't enter a target!".red());
        return Ok(());
    }
    println!(
        "\n[{}] Starting scan of all ports (1..65535). This may take a while...",
        "+".cyan()
    );
    if let Err(e) = helper::scanner::scan_ports(target).await {
        println!("{} {}", "Scan Error:".red(), e);
    }
    Ok(())
}

/// Helper function to handle subdomain discovery using a wordlist.
async fn handle_subdomain_discovery(stdin: &mut io::BufReader<io::Stdin>) -> Result<()> {
    let target = get_input(
        &format!("{} ", "Target Domain (e.g. example.com):".bold().green()),
        stdin,
    )
    .await?;
    if target.is_empty() {
        println!("{}", "ERROR: You didn't enter a target domain!".red());
        return Ok(());
    }
    println!(
        "{} Should we download the wordlist from the internet (SecLists 5000), or will you use your own file?",
        "[-]".cyan()
    );
    println!("1 -> Download directly from internet (Recommended)");
    println!("2 -> I'll provide my own file");

    let wl_choice = get_input(&format!("{} Your Choice: ", "=>".bold().yellow()), stdin).await?;
    let wordlist_source = match wl_choice.as_str() {
        "1" => "default_url".to_string(),
        "2" => {
            let wordlist = get_input(
                &format!(
                    "{} ",
                    "Wordlist File Path (e.g. subdomains.txt):".bold().green()
                ),
                stdin,
            )
            .await?;
            if wordlist.is_empty() {
                println!("{}", "ERROR: You didn't enter a file path!".red());
                return Ok(());
            }
            wordlist
        }
        _ => {
            println!("{}", "ERROR: Invalid choice!".red());
            return Ok(());
        }
    };
    if let Err(e) = helper::subdomain::run(target, wordlist_source).await {
        println!("{} {}", "Discovery Error:".red(), e);
    }
    Ok(())
}

/// Helper function to handle DNS record lookup operations.
async fn handle_dns_discovery(stdin: &mut io::BufReader<io::Stdin>) -> Result<()> {
    let target = get_input(
        &format!("{} ", "Target Domain (e.g. example.com):".bold().green()),
        stdin,
    )
    .await?;
    if target.is_empty() {
        println!("{}", "ERROR: You didn't enter a target domain!".red());
        return Ok(());
    }
    if let Err(e) = helper::dns::run(&target).await {
        println!("{} {}", "DNS Discovery Error:".red(), e);
    }
    Ok(())
}

/// Helper function to deal with HTTP security headers checking.
async fn handle_header_check(stdin: &mut io::BufReader<io::Stdin>) -> Result<()> {
    let target = get_input(
        &format!("{} ", "Target Domain (e.g. example.com):".bold().green()),
        stdin,
    )
    .await?;
    if target.is_empty() {
        println!("{}", "ERROR: You didn't enter a target domain!".red());
        return Ok(());
    }
    if let Err(e) = helper::header::run(&target).await {
        println!("{} {}", "Header Check Error:".red(), e);
    }
    Ok(())
}

/// Main entry point for the Async Pentest Toolkit.
#[tokio::main]
async fn main() -> Result<()> {
    let mut stdin = io::BufReader::new(io::stdin());

    loop {
        println!("\n{}", "=== MAIN MENU ===".bold().blue());
        println!("1 -> Subdomain Takeover Check");
        println!("2 -> Port Scanner (All Ports - 1..65535)");
        println!("3 -> Subdomain Discovery (Wordlist)");
        println!("4 -> DNS Record Discovery (A, MX, NS, TXT etc.)");
        println!("5 -> HTTP Security Headers Check");
        println!("6 -> Exit");

        let choice = get_input(&format!("{} ", "Your Choice:".bold().yellow()), &mut stdin).await?;

        match choice.as_str() {
            "1" => handle_takeover(&mut stdin).await?,
            "2" => handle_port_scan(&mut stdin).await?,
            "3" => handle_subdomain_discovery(&mut stdin).await?,
            "4" => handle_dns_discovery(&mut stdin).await?,
            "5" => handle_header_check(&mut stdin).await?,
            "6" | "q" | "quit" | "exit" => {
                println!("{}", "Exiting. Have a good day!".magenta());
                break;
            }
            _ => {
                println!("{}", "ERROR: Invalid choice, please try again.".red());
            }
        }
    }

    Ok(())
}
