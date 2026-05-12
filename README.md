# Formal-Engine v2.0 (Open Research)

![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)
![Z3](https://img.shields.io/badge/Engine-Microsoft_Z3-blue.svg)
![License](https://img.shields.io/badge/License-GPLv3-blue.svg)

**Formal-Engine** is a research framework built to demonstrate the application of Satisfiability Modulo Theories (SMT) and formal methods to vulnerability discovery and cryptanalysis. 

By treating computer science problems as algebraic constraints and leveraging solvers like Microsoft Z3, we can eliminate the $2^{160}$ search spaces that traditional fuzzers fail to penetrate.

> **Mission:** To transition cybersecurity from statistical guessing (fuzzing) to absolute mathematical certainty.

## 🚀 The Public Demo: Zero Dependencies

The cybersecurity industry is plagued by tools that are impossible to install, requiring massive Docker containers and broken dependency chains. 

This repository proves the math natively. **There are zero outside Rust dependencies required to run the interactive demo**—we rely entirely on the Rust standard library (e.g., `std::net::TcpListener`) and Microsoft's official Z3 C-bindings.

If you are on a Mac, you can give it a spin in about 15 seconds.

### Quick Start (macOS / Linux)

1. **Install Microsoft Z3:**
   ```bash
   # macOS
   brew install z3
   export Z3_SYS_Z3_HEADER=/opt/homebrew/include/z3.h
   export LIBRARY_PATH=/opt/homebrew/lib

   # Ubuntu/Debian
   sudo apt-get install libz3-dev
   ```

2. **Run the Interactive Sandbox:**
   ```bash
   cargo run --example dynamic_grail_demo
   ```

Open your browser to `http://127.0.0.1:8080`. 

You will be presented with a **3-Step Pipeline** allowing you to build a bounded logic bomb, attempt to defeat it with a PRNG fuzzer, watch the SMT engine crack it algebraically in milliseconds, and download a forensic receipt of the execution. A second tab features the **Cryptanalysis Hub**, demonstrating real-time AES Fault Analysis (DFA) and Post-Quantum Lattice (ML-KEM) breaching.

## 📚 Security & Disclosure

This repository contains the mathematical proofs and the theoretical framework. The advanced, weaponized binary surgery modules (Autonomous ELF Expansion, PLT/GOT ASLR bypasses) are maintained privately in the Enterprise repository.

- [**Security Policy**](SECURITY.md) - Guidelines on responsible disclosure and use.

*Code is Math. Math is Absolute.*