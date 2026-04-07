<div align="center">
  <img src="assets/istinye-universitesi.svg" alt="İstinye University Logo" width="150" />
  
  # ⚡ Async Subdomain Takeover & Pentest Toolkit

  [![CI/CD Status](https://github.com/denizpekova/subdomain-takeover/actions/workflows/ci.yml/badge.svg)](https://github.com/denizpekova/subdomain-takeover/actions)
  [![Rust Version](https://img.shields.io/badge/rust-v1.77+-orange.svg)](https://www.rust-lang.org/)
  [![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
</div>

This project is a high-performance, asynchronous (`Tokio`-based), comprehensive offensive security (pentest) tool built with Rust. It allows you to perform vulnerability scanning, open port detection, and DNS/Subdomain enumeration through a single interactive menu — all within seconds.

---

**Student:** Deniz Pekova  
**Advisor:** Keyvan Arasteh  
**Institution:** İstinye University, Information Security Technology

---

## 📋 Table of Contents
- [🌟 Key Features](#-key-features)
- [🛠️ Tech Stack](#️-tech-stack)
- [⚙️ Installation & Usage](#️-installation--usage)
  - [Usage with Docker](#usage-with-docker)
- [⚠️ Disclaimer](#️-disclaimer)
- [📜 License](#-license)

---

## 🌟 Key Features

### 1. 🚩 Subdomain Takeover Detection
* Sends HTTP requests to target domains or subdomains to examine potential vulnerabilities.
* Detects abandoned **CNAME** records and third-party cloud services using popular "NXDOMAIN" fingerprint matching (e.g. *GitHub Pages, Amazon S3, Heroku*).

### 2. 🚀 Async Port Scanner
* Scans all TCP ports from `1` to `65535` concurrently using 500 parallel sockets (semaphore-limited).
* Operates with low-latency timeout settings when probing services.

### 3. 🔍 Subdomain Discovery (Wordlist & Brute-Force)
* Performs wordlist-based subdomain discovery by sending requests.
* **Smart Download:** You can instruct the tool to pull **SecLists'** top 5000-word list directly and run async tests within seconds.

### 4. 📡 DNS Record Enumeration (Record Enumerator)
* Queries **A, AAAA, MX, NS, TXT** DNS records using the modern `hickory-resolver`.

### 5. 🛡️ HTTP Security Header Checker
* Scans applications for `Strict-Transport-Security`, `CSP`, and other security policies.
* Warns about leaked `Server` headers.

---

## 🛠️ Tech Stack

* **[Rust](https://www.rust-lang.org/):** A safe and blazing-fast compiled language.
* **[Tokio](https://tokio.rs/):** High-level IO and asynchronous operations.
* **[Hickory DNS](https://hickory-dns.org/):** Modern async DNS resolver.
* **[Reqwest](https://docs.rs/reqwest/latest/reqwest/):** Async HTTP client.
* **[Docker](https://www.docker.com/):** Isolation and portable deployment. (Optional)

---

## ⚙️ Installation & Usage

### Dependencies
- [Cargo and Rust Toolchain](https://rustup.rs/) (for manual installation)

### Direct Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/denizpekova/subdomain-takeover.git
   cd subdomain-takeover
   ```

2. Copy the `.env` configuration (optional):
   ```bash
   cp .env.example .env
   ```

3. Build and run:
   ```bash
   cargo run
   ```

### Usage with Docker

You can build and run the project inside a container without polluting your local environment:

```bash
docker build -t subdomain-takeover .
docker run -it --rm subdomain-takeover
```

---

## ⚠️ Disclaimer
This tool is designed solely for **information security research** and **cybersecurity professionals' investigations**. Running it against systems without explicit written permission is illegal.

## 📜 License
This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

## 🖥️ Platform

This tool runs on all major operating systems supported by the Rust toolchain:

| Platform | Supported |
|----------|-----------|
| ![Linux](https://img.shields.io/badge/Linux-FCC624?style=flat&logo=linux&logoColor=black) | ✅ |
| ![macOS](https://img.shields.io/badge/macOS-000000?style=flat&logo=apple&logoColor=white) | ✅ |
| ![Windows](https://img.shields.io/badge/Windows-0078D6?style=flat&logo=windows&logoColor=white) | ✅ |

> **Note:** Docker usage is recommended on Windows for the best experience.

---

## 🎬 Demo

Terminal recordings showcasing the tool's async speed and features are shown below:

![Terminal Demo](demo/project-demo.gif)
