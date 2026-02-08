# 🌑 Moon Dynamics: CipherPixel

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Security](https://img.shields.io/badge/security-AES--256--GCM-green.svg)](#)

**CipherPixel** is a high-performance Command Line Interface (CLI) tool designed for secure, invisible data communication. By combining **LSB (Least Significant Bit) Steganography** with military-grade encryption, it allows users to hide any file inside an image without visual degradation.

Developed under the **Moon Dynamics** umbrella, this tool is engineered for reliability, data integrity, and cryptographic robustness.

---

## 🚀 Key Features

* **File Agnostic Support:** Seamlessly hide and recover any file format (`.py`, `.pdf`, `.zip`, `.exe`, etc.) by treating data as a raw binary stream.
* **Military-Grade Encryption:** Uses **AES-256-GCM** (Galois/Counter Mode) for authenticated encryption, ensuring both confidentiality and integrity.
* **Advanced Key Derivation:** Implements **Argon2id**, the winner of the Password Hashing Competition, to derive 256-bit keys from user passwords with high resistance to GPU brute-force attacks.
* **Metadata-Driven Extraction:** Uses a custom **4-byte Big-Endian header** to store payload length, ensuring precise data recovery and preventing decryption failures caused by image noise.
* **High Fidelity:** Employs LSB steganography on RGB channels, keeping the carrier image visually identical to the original.
* **Memory Safety & Speed:** Built entirely in **Rust** for blazing-fast processing and zero-cost abstractions.

---

## 🛠 Tech Stack

* **Language:** Rust (Edition 2021)
* **Cryptography:** `aes-gcm`, `argon2`
* **Image Processing:** `image` crate
* **CLI Framework:** `clap` (Command Line Argument Parser)

---

## 📦 Installation

Ensure you have the [Rust toolchain](https://rustup.rs/) installed.

1. **Clone the repository:**
   ```bash
   git clone [https://github.com/drdoom0979-dot/CipherPixel.git)
   cd cipherpixel
   cargo build --release
   ```
2. **Configure Global Alias (Mac/Linux):**
   ```bash
   Add this to your ~/.zshrc or ~/.bashrc:
   nano ~/.zshrc  or  nano  ~/.bashrc 
   alias cipherpixel='/path/to/cipherpixel/target/release/cipher_pixel_cli'
   source ~/.zshrc
   ```
   

## 💻 Usage

### 1. Hide a file (Encode)
  ```bash
  cipherpixel hide -i carrier.png -f secret.py -p "YourPassword" -o result.png
  ```

### 2. Extract a file (Decode)
  ```bash
  cipherpixel extract -i result.png -p "YourPassword" -o recovered.py
  ```
