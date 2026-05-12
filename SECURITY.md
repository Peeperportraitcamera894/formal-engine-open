# Security Policy

## ⚠️ Disclosure & Intent
The **Formal-Engine (Open Research)** repository is published strictly for academic review, defensive auditing, and mathematical demonstration of Satisfiability Modulo Theories (SMT) applied to computer science problems. 

This public release contains the core mathematical constraints (e.g., PQC Lattice solving, AES Fault Analysis equations) and the interactive sandbox. It **does not** contain the proprietary Binary Surgery, Automated Exploit Generation (AEG), or ELF Expansion modules required to weaponize these concepts against physical systems. 

**Do not utilize the methodologies described herein against systems, networks, or applications you do not own or possess explicit, written authorization to audit.**

## Reporting a Vulnerability
If you discover a flaw in the engine's cryptographic implementations (e.g., the `PqcLatticeCore`) or a math-based bypass in the State Machine solver that breaks the formal invariants, please do **NOT** open a public issue.

Instead, please send an email outlining the vulnerability to the project maintainers. We will acknowledge receipt within 48 hours and work with you to verify and patch the mathematical constraints before public disclosure.

## Commercial Deployment & Liability
This repository is released under the **GNU GPLv3** license. 

The creators and maintainers of Formal-Engine accept **zero liability** for damages, downtime, or compliance breaches resulting from the application of the mathematical concepts or code contained within this repository. 

If you represent an enterprise requiring the fully automated, CI/CD-integrated Binary Surgery tools, or if you require an exemption from the GPLv3 copyleft terms for commercial deployment, please contact the maintainers regarding the private `formal-engine-enterprise` license.