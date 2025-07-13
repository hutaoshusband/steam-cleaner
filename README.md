# Steam Cleaner (Rust CLI Tool)

**Steam Cleaner** is a command-line utility written in Rust that aims to clean and spoof key identifiers on your system to help with creating "clean" environments for new accounts (made for CS2, might work for some other user level anticheats).
It focuses on spoofing key registry values and removing cached traces—without requiring a full Windows reinstall.

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

Steam Cleaner removes a wide range of files and folders related to Steam activity:

- `steamapps/libraryfolders.vdf`
- `steamapps/appmanifest_*.acf` (⚠️ can result in lost game data)
- `userdata/`, `logs/`, `appcache/`, `dump/`, `shadercache/`
- All Steam-related registry keys (e.g. account history, tracking)
- `Steam` folder in `%APPDATA%` and `%LOCALAPPDATA%`

### System-Wide Trace Cleaning

Additional locations are cleared to minimize tracking and local history:

- `%TEMP%`, `%APPDATA%\Microsoft\Windows\Recent`
- `%LOCALAPPDATA%\Temp`, `CrashDumps`, `WebCache`, `INetCache`, etc.
- `C:\Windows\Temp`
- `C:\ProgramData\NVIDIA Corporation\NV_Cache`
- `C:\ProgramData\Microsoft\Windows\Caches`
- Tracing folders and jump list data

**Note:** These actions will **log you out of all Steam accounts** and may **cause loss of local Steam configurations, cache, and possibly savedata**. Back up important files before running.

---

## Does this prevent Red Trust in CS2?

**No.**  
Steam Cleaner only removes traces that influences trust scores for *new accounts*. When your old account was flagged by VAC before.
If your new account receives Red Trust due to **cheating, griefing, or suspicious behavior**, that's entirely your responsibility.

Steam Cleaner **does not provide protection** against VAC or Trust Factor downgrades caused by actual gameplay behavior or reports.

---

## Requirements

- Windows 10/11 (7)
- [Rust Toolchain](https://www.rust-lang.org/tools/install)
- Admin rights (UAC prompt appears at launch)
- `volumeid64.exe` (Microsoft Sysinternals tool included in the source)

---

## Installation & Build

```bash
git clone https://github.com/hutaoshusband/steam_cleaner.git
cd steam_cleaner
cargo build --release
mkdir tools
copy src\tools\volumeid64.exe .\tools\
steam_cleaner.exe
```

## DISCLAIMER

This tool does not modify firmware (e.g. BIOS, SMBIOS, etc.)
No kernel-level operations are performed
This is not a cheat, but a cleanup utility.

## Lisense

License
MIT License – See LICENSE for full terms.
