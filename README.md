# Steam Cleaner (Rust GUI Tool)

⚠️ **This project is intended for educational and testing purposes only. You use it at your own risk.**

---

## Features

### Identifier Spoofing

The following identifiers can be randomized or spoofed:

- `MachineGuid` (`HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Cryptography`)
- `HwProfileGuid`
- Windows Product ID
- InstallDate and RegisteredOwner
- Values under `SOFTWARE\Microsoft\Windows NT\CurrentVersion`
- Computer name
- MAC addresses (network adapters)
- Volume ID (`volumeid64.exe` required)
- Windows Security Identifier (SID)
- some more stuff

### Steam-Specific Cleaning

**Note:** These actions will **log you out of all Steam accounts** and may **cause loss of local Steam configurations, cache, and possibly savedata**. Back up important files before running.

## Does this prevent Red Trust in CS2?

**No.**  
Steam Cleaner only removes traces that influences trust scores for *new accounts*. When your old account was flagged by VAC before.
If your new account receives Red Trust due to **cheating, griefing, or suspicious behavior**, that's entirely your responsibility.

Steam Cleaner **does not provide protection** against VAC or Trust Factor downgrades caused by actual gameplay behavior or reports.

---

## Requirements

- Windows 10/11 (7)
- [Rust Toolchain](https://www.rust-lang.org/tools/install) (without the binary)
- VC Runtime

---

## Installation & Build

```bash
git clone https://github.com/hutaoshusband/steam_cleaner.git
cd steam_cleaner
cargo build --release
```

## DISCLAIMER

This tool does not modify firmware (e.g. BIOS, SMBIOS, etc.)
No kernel-level operations are performed
This is not a cheat, but a cleanup utility.

## Lisense

License
MIT License – See LICENSE for full terms.
