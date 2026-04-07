use colored::*;
use reqwest::Client;
use std::time::Duration;

/// Essential security headers to look for in the HTTP response.
const SECURITY_HEADERS: &[&str] = &[
    "strict-transport-security",
    "x-frame-options",
    "x-content-type-options",
    "content-security-policy",
    "x-xss-protection",
    "referrer-policy",
    "permissions-policy",
];

/// Checks the target website for common missing web security headers.
/// Useful for identifying potential clickjacking, XSS, or other misconfigurations.
/// It automatically prefixes the target with HTTPS if no scheme is provided.
pub async fn run(target: &str) -> anyhow::Result<()> {
    let client = Client::builder().timeout(Duration::from_secs(5)).build()?;

    let url = if target.starts_with("http://") || target.starts_with("https://") {
        target.to_string()
    } else {
        format!("https://{}", target) // Security features are generally checked over HTTPS
    };

    println!(
        "\n🌐 Checking HTTP Security Headers for {}...",
        url.bold().cyan()
    );

    match client.get(&url).send().await {
        Ok(response) => {
            let headers = response.headers();
            let mut missing_headers = vec![];

            println!("\n[{}] Found Security Headers:", "+".green());
            for &sec_h in SECURITY_HEADERS {
                if let Some(val) = headers.get(sec_h) {
                    println!(
                        "  [✓] {}: {}",
                        sec_h.bright_green(),
                        val.to_str().unwrap_or("unreadable").yellow()
                    );
                } else {
                    missing_headers.push(sec_h);
                }
            }

            if !missing_headers.is_empty() {
                println!("\n[{}] Missing or Neglected Security Headers:", "-".red());
                for h in missing_headers {
                    println!("  [!] {} not found!", h.red());
                }
            } else {
                println!(
                    "\n[{}] Great! All standard security headers checked are present.",
                    "+".bright_green()
                );
            }

            // Also print the server info for informational purposes
            if let Some(server) = headers.get("server") {
                println!(
                    "\n[{}] Server: {}",
                    "i".cyan(),
                    server.to_str().unwrap_or("Unknown").yellow()
                );
            }
        }
        Err(e) => {
            println!(
                "  [{}] Connection could not be established (Target Error): {}",
                "ERROR".red(),
                e
            );
        }
    }

    println!("\n✅ Header analysis completed.\n");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_header_run_invalid_url() {
        // Just verify it doesn't crash on completely invalid non-resolving domains
        let res = run("http://this-does-not-exist.test-domain").await;
        assert!(res.is_ok());
    }
}
