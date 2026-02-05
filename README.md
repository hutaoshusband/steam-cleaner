# Steam Cleaner v0.1.9

A Windows GUI tool for Steam cache cleaning, system identifier spoofing, and trust factor reset.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Platform: Windows](https://img.shields.io/badge/Platform-Windows-0078D4.svg)](https://www.microsoft.com/windows)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

---


**This tool CAN performs DESTRUCTIVE operations:**

- Logs you out of **ALL** Steam accounts (deletes login files)
- Deletes Steam userdata, configs, and cloud sync data
- Removes orphaned game folders **permanently**
- Modifies Windows Registry keys
- Changes system identifiers (MAC, Volume ID, MachineGuid, etc.)
- Deletes anti-cheat data (EAC, BattlEye, FACEIT)

**BACKUP FIRST! (if dangerous options used)**

---

## Table of Contents

- [How Steam Trust Factor Works](#how-steam-trust-factor-works)
- [Features](#features)
- [Project Structure](#project-structure)
- [GUI Layout](#gui-layout)
- [Safety & Backups](#safety--backups)
- [Installation](#installation)
- [Usage Guide](#usage-guide)
- [Detailed Log Examples](#detailed-log-examples)
- [FAQ](#faq)
- [Requirements](#requirements)
- [Troubleshooting](#troubleshooting)
- [Disclaimer](#disclaimer)
- [License](#license)

---

## How Steam Trust Factor Works

Valve's trust factor system analyzes multiple data points to determine your account's trust score.

### 1. Steam Userdata (Local Files)

Steam stores unique IDs for every account that has logged into a specific machine.

**Path:** `C:\Program Files (x86)\Steam\userdata\`

**Mechanism:** Inside this folder, you will find subfolders. When you launch CS2, the game can see which other accounts have active folders here. If one folder belongs to a VAC-banned or "Red Trust" account, the system flags the current account as likely belonging to the same user.

**What Steam Cleaner Does:** Deletes the entire `userdata/` directory, removing all traces of previously logged-in accounts.

### 2. Steam Registry Keys

Valve uses registry entries to track the "Last User" and account associations.

**Key Path:** `HKEY_CURRENT_USER\Software\Valve\Steam`

**Active Keys:**
- `AutoLoginUser`: Tracks the last account used
- `RememberPassword`: Often used to link sessions

**Registry Hives:** Valve may also check `HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Cryptography` (MachineGuid) to identify the specific hardware fingerprint.

**What Steam Cleaner Does:**
- Deletes all Valve/Steam registry keys from HKCU, HKLM, and HKU
- Randomizes MachineGuid and other hardware identifiers in the registry

### 3. Configuration & Metadata

**Files:** `config.vdf` and `loginusers.vdf` (located in `\Steam\config\`)


**What Steam Cleaner Does:** Deletes `config.vdf`, `loginusers.vdf`, `localconfig.vdf`, and SSFN files.

### Non-File Factors that "Transfer" Low Trust

Even if you wipe your registry and files, these account-level links are often more powerful:

| Factor | Impact | What Steam Cleaner Does |
|--------|--------|-------------------------|
| **Shared Phone Number** | Linking the same phone number via Steam Guard to a low-trust account will immediately tank the trust of the second account. | ❌ Cannot fix (account-level) |
| **Common Email** | Using the same recovery email for multiple accounts. | ❌ Cannot fix (account-level) |
| **Hardware ID (HWID) & IP** | Valve officially states they focus on "behavior," but community analysis suggests that frequent switching between a red-trust account and a clean account on the same IP/Hardware eventually synchronizes their trust levels. | ✅ Partially fixes (spoofs MAC, Volume ID, MachineGuid) |
| **Behavioral Patterns** | Cheating, griefing, suspicious gameplay, reports. | ❌ Cannot fix (your responsibility) |
| **MOST IMPORTANT: Account Balance** | Cannot be affected by Steam Cleaner! |
---

## Features

### System Identifier Spoofing

**Registry-based identifier randomization:**
- `MachineGuid` (`HKLM\SOFTWARE\Microsoft\Cryptography`)
- `HwProfileGuid` (`HKLM\SYSTEM\CurrentControlSet\Control\IDConfigDB`)
- Windows Product ID (`DigitalProductId`)
- Registered Owner (`RegisteredOwner`)
- Install Date (`InstallDate`)
- Computer Name (`ComputerName`)

**Hardware identifier spoofing:**
- MAC addresses (all physical network adapters)
- Volume ID (C: drive and other volumes)
- Windows Security Identifier (SID) related keys

### Steam Cleaning Operations

**Login files removed (logs you out):**
- `loginusers.vdf` - Steam account login cache
- `config.vdf` - Main configuration
- `localconfig.vdf` - Local user settings
- `Steam AppData.vdf` - AppData configuration
- `ssfn*.dll` - Steam sentry files

**Directories deleted:**
- `userdata/` - All account data and saves
- `config/` - Steam configuration directory
- `logs/` - Steam logs
- `appcache/` - Application cache
- `dump/` - Crash dumps
- `shadercache/` - Shader cache
- `depotcache/` - Depot cache
- **Orphaned game folders** - Games not in libraryacf

### System Cache Cleaning

**Windows temp directories:**
- `Local\Temp` contents
- `LocalLow\Temp` contents
- `Temp` contents
- `Windows\Temp` contents
- Crash dumps

**Windows Explorer caches:**
- Web cache
- INet cache
- Thumbnail cache
- Icon cache
- Windows Explorer cache

**Recent files tracking:**
- Recent documents list
- Automatic Destinations (jump lists)
- Custom Destinations
- Tracing directory

### Registry Cleaning

**Game tracking removal:**
- Steam registry keys (HKCU)
- Valve registry keys (HKLM & HKU)
- FACEIT registry keys (HKCU)
- Riot Games registry keys (HKCU)
- ESEA registry keys (HKCU)
- EasyAntiCheat registry keys (HKCU)
- BattlEye registry keys (HKCU)
- Startup Run entries

**System cache removal:**
- AppCompat Cache
- Shim Cache
- AppCompat Flags

### Anti-Cheat & Game Data Removal

- EasyAntiCheat folders
- BattlEye folders
- FACEIT folders
- `My Games` folder contents

### GPU Cache Cleaning

- NVIDIA shader cache deletion
- AMD shader cache deletion

### Additional Features

- **Backup System**: Create backups of Steam config and userdata
- **Hardware Profiles**: Save/load MAC and Volume ID profiles
- **Simulation Mode**: Dry-run to see what would be deleted
- **Multi-language Support**: English, German, Spanish, French, Italian, Japanese, Portuguese, Russian, Chinese
- **Granular Options**: Choose exactly what to clean
- **Redistributable Cleaner**: Remove Steam redistributable installers
- **Inspector**: View system information

---

## Project Structure

```
steam-cleaner/
├── Cargo.toml                 # Rust project configuration (Iced 0.12, tokio, etc.)
├── src/
│   ├── main.rs               # Application entry point
│   ├── core/                 # Business logic modules
│   │   ├── backup.rs         # Backup/restore functionality
│   │   ├── executor.rs       # Cleaning operations orchestration
│   │   ├── file_cleaner.rs   # File and directory cleaning
│   │   ├── hardware_profile.rs  # Hardware profile management
│   │   ├── inspector.rs      # System information gathering
│   │   ├── mac_spoofer.rs    # MAC address spoofing
│   │   ├── redist.rs         # Steam redistributable cleaner
│   │   ├── registry_cleaner.rs  # Registry operations
│   │   ├── sid_spoofer.rs    # SID spoofing
│   │   ├── steam.rs          # Steam installation detection
│   │   └── volumeid_wrapper.rs  # Volume ID spoofing
│   ├── ui/                   # UI components
│   │   ├── app.rs            # Main application UI (~1600 lines)
│   │   ├── mod.rs
│   │   ├── redist_view.rs    # Redistributable cleaner view
│   │   └── style.rs          # Styling definitions (~1600 lines)
│   └── i18n/                 # Internationalization
│       ├── mod.rs            # Language enum and translation loading
│       └── locales/          # Translation files (JSON)
│           ├── en.json
│           ├── de.json
│           ├── es.json
│           ├── fr.json
│           ├── it.json
│           ├── pt.json
│           ├── ru.json
│           ├── ja.json
│           └── zh.json
└── assets/
    └── icon.ico
```

---

## GUI Layout

```
┌─────────────────────────────────────────────────────────────────┐
│                    Steam Cleaner v0.1.9                          │
├─────────────────────────────────────────────────────────────────┤
│  Left Panel (30%)         │   Right Panel (70%)                  │
│  ┌──────────────────────┐│  ┌─────────────────────────────────┐ │
│  │ Main Options         ││  │  Runtime Log Output             │ │
│  │                      ││  │  (Scrollable)                   │ │
│  │ ☑ Spoof System IDs   ││  │                                 │ │
│  │ ☑ Spoof MAC          ││  │ [✓] Successfully spoofed...    │ │
│  │ ☑ Spoof Volume ID    ││  │ [*] Adapter detected...         │ │
│  │ ☑ Clean Steam        ││  │ [File] Deleted: ...             │ │
│  │ ☑ Clean Aggressive   ││  │                                 │ │
│  │                      ││  │ [Real-time log streaming]       │ │
│  │ ☑ Simulation Mode    ││  │                                 │ │
│  │                      ││  │                                 │ │
│  ├──────────────────────┤│  └─────────────────────────────────┘ │
│  │ Language: [English ▼]││                                     │
│  ├──────────────────────┤│                                     │
│  │ Theme:    [Nord ▼]   ││                                     │
│  ├──────────────────────┤│                                     │
│  │                      ││                                     │
│  │ ┌──────────────────┐ ││                                     │
│  │ │  🧹 START CLEAN  │ ││                                     │
│  │ └──────────────────┘ ││                                     │
│  │                      ││                                     │
│  │ ┌──────────────────┐ ││                                     │
│  │ │  ⚙️  Granular     │ ││                                     │
│  │ └──────────────────┘ ││                                     │
│  │                      ││                                     │
│  │ ┌──────────────────┐ ││                                     │
│  │ │  💾 Create Backup│ ││                                     │
│  │ └──────────────────┘ ││                                     │
│  │                      ││                                     │
│  │ ┌──────────────────┐ ││                                     │
│  │ │  🔍 Inspector    │ ││                                     │
│  │ └──────────────────┘ ││                                     │
│  │                      ││                                     │
│  │ ┌──────────────────┐ ││                                     │
│  │ │  🎮 Redist Clean │ ││                                     │
│  │ └──────────────────┘ ││                                     │
│  │                      ││                                     │
│  │ ┌──────────────────┐ ││                                     │
│  │ │  👤 Profiles     │ ││                                     │
│  │ └──────────────────┘ ││                                     │
│  │                      ││                                     │
│  └──────────────────────┘│                                     │
└─────────────────────────────────────────────────────────────────┘
```

### Navigation Buttons

| Button | Function |
|--------|----------|
| **Start Clean** | Execute selected cleaning operations |
| **Granular** | Open detailed options for selective cleaning |
| **Create Backup** | Create ZIP backup of Steam config/userdata |
| **Inspector** | View system information (Steam path, adapters, etc.) |
| **Redist Clean** | Clean Steam redistributable installers |
| **Profiles** | Manage hardware profiles (save/load MAC, Volume ID) |

---

## Safety & Backups

### What Gets Deleted (Permanent Loss)

```
📁 Files/Folders Deleted Permanently:
├── Steam Login Files (re-enter password required)
│   ├── C:\Program Files (x86)\Steam\config\loginusers.vdf
│   ├── C:\Program Files (x86)\Steam\config\config.vdf
│   ├── C:\Program Files (x86)\Steam\config\localconfig.vdf
│   ├── C:\Program Files (x86)\Steam\config\SteamAppData.vdf
│   └── C:\Program Files (x86)\Steam\ssfn*.dll
├── Steam Userdata (saves, settings, screenshots)
│   └── C:\Program Files (x86)\Steam\userdata\
│       └── [Steam3ID]/ (all subdirectories)
├── Orphaned Game Folders
│   └── C:\Program Files (x86)\Steam\steamapps\common\
│       └── [folders not in libraryacf]
├── Anti-Cheat Data
│   ├── EasyAntiCheat/
│   ├── BattlEye/
│   └── FACEIT/
└── System Caches
    ├── C:\Users\[User]\AppData\Local\Temp\
    ├── C:\Users\[User]\AppData\LocalLow\Temp\
    ├── C:\Windows\Temp\
    ├── C:\Windows\Prefetch\
    └── Browser caches
```

### What You Should Backup

```
📦 BACKUP BEFORE RUNNING:
├── Steam Userdata (IMPORTANT!)
│   └── C:\Program Files (x86)\Steam\userdata\[YOUR_STEAM3ID]\
│       ├── [GameID]/ (save game locations)
│       ├── 760/ (screenshots)
│       ├── 730/ (CS2 settings)
│       └── config/ (local configurations)
├── Steam Config
│   └── C:\Program Files (x86)\Steam\config\
│       └── Any custom configurations you've modified
└── Orphaned Game Folders (if you want to keep them)
    └── C:\Program Files (x86)\Steam\steamapps\common\
        └── [games not in Steam library]
```

### Using the Built-in Backup Feature

The tool can create a ZIP backup of:
- `Steam/config/` directory
- `Steam/userdata/` directory

**Recommended workflow:**
1. Run tool in **Simulation Mode** first (review what would be deleted)
2. Click **Create Backup** button
3. Save backup to a safe location (external drive recommended)
4. Verify backup file exists and is not corrupt
5. Run cleaning operations
6. Restart computer

**To restore from backup:**
- Extract the ZIP backup
- Copy `config/` and `userdata/` back to your Steam directory
- Note: This will restore all traces you just tried to remove

---

## Installation

### Option A: Download Pre-built Binary (Recommended)

1. Go to [Releases](https://github.com/hutaoshusband/steam_cleaner/releases)
2. Download `steam-cleaner-v0.1.9.exe` (or latest version)
3. Right-click → **Run as Administrator**
4. Windows Defender may flag it - allow if you trust the source
5. The tool requires administrator privileges for registry and system operations

### Option B: Build from Source

**Prerequisites:**
- Windows 10/11
- Rust toolchain (1.70 or later) - [Install Rust](https://rustup.rs/)
- Visual C++ Build Tools (for Windows)

**Build steps:**
```bash
# Clone the repository
git clone https://github.com/hutaoshusband/steam_cleaner.git
cd steam_cleaner

# Build in release mode
cargo build --release

# Run the binary
.\target\release\steam_cleaner.exe
```

**Output location:** `target\release\steam_cleaner.exe`

---

## Usage Guide

### Quick Start

1. **Launch as Administrator**: Right-click → Run as Administrator
2. **Select Options**: Check the boxes for operations you want to perform
3. **Enable Simulation Mode** (first time): Check "Simulation Mode" to preview changes
4. **Click Start Clean**: Review the log output on the right panel
5. **Verify**: Check that the simulation results look correct
6. **Create Backup** (optional but recommended): Click "Create Backup"
7. **Disable Simulation Mode**: Uncheck "Simulation Mode"
8. **Run for Real**: Click "Start Clean" again
9. **Restart Computer**: Required for all changes to take effect

### Granular Options

For more control over what gets cleaned, click the **Granular** button to access:

- **Registry Spoofing**: Choose which identifiers to randomize
- **Game Tracking Removal**: Select which anti-cheat/game registry keys to delete
- **Steam Login Files**: Select specific login files to delete
- **Steam Directories**: Choose which Steam folders to clean
- **System Caches**: Select specific cache directories to clean
- **Deep Cleaning**: Enable/disable aggressive cleaning options

### Inspector

Click **Inspector** to view:
- Steam installation path
- Steam library folders
- Network adapters (for MAC spoofing)
- Volume IDs
- System information

### Profiles

Click **Profiles** to:
- **Save Profile**: Save current MAC addresses and Volume IDs
- **Load Profile**: Restore previously saved hardware identifiers
- **Manage Profiles**: Rename, delete, or export profiles

This is useful for:
- Switching between different "identities"
- Testing different configurations
- Reverting to a known-good state

---

## Detailed Log Examples

### Example 1: Full Clean

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
           STEAM CLEANER v0.1.9
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

--- SIMULATION MODE (DRY RUN) ---
[*] Starting asynchronous cleaning...

━━━ REGISTRY SPOOFING ━━━
[Registry] Would spoof HKLM\SOFTWARE\Microsoft\Cryptography\MachineGuid
    Old: {a1b2c3d4-e5f6-7890-abcd-ef1234567890}
    New: {f7e8d9c0-b1a2-3456-7890-abcdef123456}
[Registry] Would spoof HwProfileGuid
    Old: {12345678-1234-1234-1234-123456789012}
    New: {98765432-4321-4321-4321-210987654321}
[Registry] Would spoof HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\DigitalProductId
[Registry] Would spoof HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\RegisteredOwner
    Old: User
    New: Owner-7f2a9c
[Registry] Would spoof HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\InstallDate
    Old: 1634567890
    New: 1678901234
[Registry] Would spoof HKLM\SYSTEM\CurrentControlSet\Control\ComputerName\ComputerName\ComputerName
    Old: DESKTOP-ABC123
    New: DESKTOP-XYZ789

━━━ MAC ADDRESS SPOOFING ━━━
[*] Adapter detected: 'Realtek Gaming 2.5GbE Family Controller' (Key: 0001)
    → New MAC: 2A:3B:4C:5D:6E:7F
    [DRY-RUN] Would execute: reg add "HKLM\SYSTEM\CurrentControlSet\Control\Class\{4d36e972-e325-11ce-bfc1-08002be10318}\0001" /v NetworkAddress /d 2A3B4C5D6E7F /f
[–] Skipped: 'Hyper-V Virtual Ethernet Adapter' (virtual adapter)
[–] Skipped: 'WAN Miniport (IP)' (wan miniport)
[–] Skipped: 'Bluetooth Device (Personal Area Network)' (bluetooth)
[*] Adapter detected: 'Intel(R) Wi-Fi 6 AX200' (Key: 0002)
    → New MAC: 8A:9B:AC:BD:CE:DF
    [DRY-RUN] Would execute: reg add "HKLM\SYSTEM\CurrentControlSet\Control\Class\{4d36e972-e325-11ce-bfc1-08002be10318}\0002" /v NetworkAddress /d 8A9BACBDCE /f
[✓] MAC spoofing complete. 2 adapters would be changed. Reboot needed!

━━━ VOLUME ID SPOOFING ━━━
[*] Changing Volume ID for C: drive
    Old: 1234-ABCD
    New: 5678-EF90
    [DRY-RUN] Would execute: volumeid64.exe C: 5678-EF90
[✓] Volume ID would be changed. Reboot needed!

━━━ STEAM CLEANING ━━━
[Process] Would execute: taskkill /F /IM steam.exe
[Process] Would execute: taskkill /F /IM steamwebhelper.exe
[Process] Would execute: taskkill /F /IM gameoverlayui.exe

[Directory] Would delete contents of: C:\Program Files (x86)\Steam\userdata
    → Found 8 user directories
    → Estimated size: ~2.3 GB

[Directory] Would delete contents of: C:\Program Files (x86)\Steam\config
    → Files: loginusers.vdf, config.vdf, localconfig.vdf

[File] Would delete: C:\Program Files (x86)\Steam\config\loginusers.vdf
[File] Would delete: C:\Program Files (x86)\Steam\config\config.vdf
[File] Would delete: C:\Program Files (x86)\Steam\config\localconfig.vdf
[File] Would delete: C:\Program Files (x86)\Steam\config\SteamAppData.vdf

[File] Would delete SSFN files:
    C:\Program Files (x86)\Steam\ssfnxxxxxxxx.dll
    C:\Program Files (x86)\Steam\ssfnyyyyyyyy.dll

[Directory] Would delete: C:\Program Files (x86)\Steam\logs\
[Directory] Would delete: C:\Program Files (x86)\Steam\appcache\
[Directory] Would delete: C:\Program Files (x86)\Steam\dump\
[Directory] Would delete: C:\Program Files (x86)\Steam\shadercache\
[Directory] Would delete: C:\Program Files (x86)\Steam\depotcache\

━━━ ORPHANED GAME FOLDERS ━━━
[Steam] Checking libraryfolders.vdf...
[Steam] Found 3 library folders:
    → C:\Program Files (x86)\Steam\steamapps
    → D:\Games\Steam\steamapps
    → E:\SteamLibrary\steamapps

[Steam] Scanning for orphaned game folders...
[!] Orphaned game folder detected: C:\Program Files (x86)\Steam\steamapps\common\UninstalledGame1
    → Not in libraryacf
    → Would delete entire folder (~15.2 GB)
[!] Orphaned game folder detected: D:\Games\Steam\steamapps\common\OldGame
    → Not in libraryacf
    → Would delete entire folder (~8.7 GB)

━━━ SYSTEM CACHE CLEANING ━━━
[Directory] Would delete contents of: C:\Users\YourName\AppData\Local\Temp
    → 1,247 files found (~1.8 GB)
[Directory] Would delete contents of: C:\Users\YourName\AppData\LocalLow\Temp
    → 89 files found (~124 MB)
[Directory] Would delete contents of: C:\Users\YourName\Temp
    → 456 files found (~890 MB)
[Directory] Would delete contents of: C:\Windows\Temp
    → 23 files found (~45 MB)

[File] Would delete crash dumps:
    C:\Users\YourName\AppData\Local\CrashDumps\steam.exe.1234.dmp
    C:\Users\YourName\AppData\Local\CrashDumps\steam.exe.5678.dmp

━━━ WINDOWS EXPLORER CACHES ━━━
[Directory] Would delete: C:\Users\YourName\AppData\Local\Microsoft\Windows\WebCache
    → ~234 MB
[Directory] Would delete: C:\Users\YourName\AppData\Local\Microsoft\Windows\INetCache
    → ~567 MB
[Directory] Would delete icon cache files:
    → ~12 MB
[Directory] Would delete thumbnail cache files:
    → ~89 MB

━━━ RECENT FILES ━━━
[File] Would delete: C:\Users\YourName\AppData\Roaming\Microsoft\Windows\Recent\*
    → 156 automatic destination files
[Directory] Would delete: C:\Users\YourName\AppData\Local\Microsoft\Windows\History\*
    → 89 custom destination files
[Directory] Would delete: C:\Users\YourName\AppData\Local\Microsoft\Windows\Tracing\*
    → ~34 MB

━━━ GPU CACHE ━━━
[Directory] Would delete: C:\Users\YourName\AppData\Local\NVIDIA\DXCache
    → ~1.2 GB
[Directory] Would delete: C:\Users\YourName\AppData\Local\NVIDIA\GLCache
    → ~890 MB

━━━ DEEP CLEANING ━━━
[Directory] Would delete: C:\Windows\Prefetch\*
    → 234 files (~45 MB)
[Directory] Would delete: C:\Users\YourName\Documents\My Games\*
    → ~567 MB (various game configs)
[Directory] Would delete EasyAntiCheat folders (multiple games)
[Directory] Would delete BattlEye folders (multiple games)
[Directory] Would delete FACEIT folders (multiple games)

━━━ REGISTRY CLEANING ━━━
[Registry] Would delete: HKCU\Software\Valve\Steam
[Registry] Would delete: HKLM\Software\Valve
[Registry] Would delete: HKU\Software\Valve
[Registry] Would delete: HKCU\Software\FaceIt
[Registry] Would delete: HKCU\Software\Faceit Ltd
[Registry] Would delete: HKCU\Software\Riot Games
[Registry] Would delete: HKCU\Software\ESEA
[Registry] Would delete: HKCU\Software\EasyAntiCheat
[Registry] Would delete: HKCU\Software\BattlEye
[Registry] Would delete: HKCU\Software\Microsoft\Windows\CurrentVersion\Run
[Registry] Would clean: HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\AppCompatCache
[Registry] Would clean: HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\AppCompatFlags
[Registry] Would clean: HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Caches

━━━ REDISTRIBUTABLE CLEANING ━━━
[*] Scanning for redistributables...
[!] Found 23 CommonRedist folders
    → C:\Program Files (x86)\Steam\steamapps\common\Game1\_CommonRedist\DirectX\
    → C:\Program Files (x86)\Steam\steamapps\common\Game1\_CommonRedist\DotNet\
    → C:\Program Files (x86)\Steam\steamapps\common\Game2\_CommonRedist\VCRedist\
    → (20 more folders...)
[!] Would delete ~2.3 GB of redistributable installers

━━━ SUMMARY ━━━
Total operations: 2,456
Estimated disk space freed: ~12.8 GB
Registry keys modified: 18
Registry keys deleted: 12
Adapters spoofed: 2
Volume IDs changed: 1

⚠️  CRITICAL: Review all changes above carefully!
⚠️  This will log you out of ALL Steam accounts!
⚠️  Orphaned game folders will be PERMANENTLY deleted!

--- END OF SIMULATION ---
```

---

## FAQ

### General Questions

**Q: Will this get me VAC banned?**
A: No. This tool only removes local traces. It does not interact with Steam games or anti-cheat systems in any way. VAC bans are issued server-side based on detected cheats, not for running system cleanup tools.

**Q: Do I need to restart my computer?**
A: YES. For MAC address, Volume ID, registry changes, and system identifier spoofing to take effect, you MUST restart your computer.

**Q: Can I restore my deleted files?**
A: No. Deleted files are permanently removed. Always create a backup first using the built-in backup feature or manually copying important files.

**Q: Will this affect my installed games?**
A: No, games in your Steam library will not be touched. However, game folders NOT in your `libraryacf` (orphaned folders) will be permanently deleted.

**Q: Can I use this on my main Steam account?**
A: Yes, but you'll be logged out and need to re-enter your password. Your cloud saves should sync when you log back in, but local saves in `userdata/` may be lost unless backed up.

**Q: Does this work with Steam Guard?**
A: Yes, but you'll need to enter your Steam Guard code when logging back in.

**Q: How often should I run this?**
A: Only run this when you need to reset your system identifiers. Running it frequently is not recommended and may cause unnecessary data loss.

**Q: Can I undo the changes?**
A: Only by restoring from a backup you created before running. System identifier changes (MAC, Volume ID, registry) cannot be undone otherwise.

### Trust Factor Questions

**Q: Does this prevent Red Trust in CS2?**
A: **No.** Steam Cleaner only removes traces that influence trust scores for *new accounts*. If your old account was flagged by VAC, this tool cannot help. If your new account receives Red Trust due to **cheating, griefing, or suspicious behavior**, that's entirely your responsibility.

**Q: Will this improve my trust factor?**
A: It may help for new accounts by removing traces of previously banned accounts from your system. However, trust factor is based on many factors including:
- Your behavior in-game
- Reports from other players
- Account-linked phone number
- Email associations
- Purchase history
- Account age

**Q: Does Steam Cleaner fix account-level links?**
A: No. Account-level links like:
- Shared phone number across accounts
- Same email address
- Same payment method
- Friends list connections

These cannot be fixed by Steam Cleaner. You would need to create completely separate accounts with no links.

**Q: What if I have a phone number linked to a banned account?**
A: Any account using that same phone number will inherit the low trust. You need to use a different phone number for your new account.

### Technical Questions

**Q: Why does it need Administrator privileges?**
A: The tool modifies:
- Windows Registry (HKLM, HKCU, HKU hives)
- Network adapter settings (MAC addresses)
- Volume IDs
- System files and caches

All these operations require administrator access.

**Q: Does this modify firmware?**
A: No. This tool does not modify BIOS, UEFI, SMBIOS, or any firmware. It only changes software-level identifiers.

**Q: Is this a cheat or hack?**
A: No. This is a system cleanup utility, similar to CCleaner but specifically designed for Steam and system identifiers.

**Q: Will this work with Windows 7?**
A: It may work, but Windows 7 is not officially supported. Windows 10/11 is recommended.

**Q: Can I run this on a Mac/Linux?**
A: No. This tool is Windows-only. It uses Windows-specific APIs, registry operations, and Windows executables (volumeid64.exe). (Work in progress for Linxu.)

---

## Requirements

### System Requirements

| Requirement | Specification |
|-------------|---------------|
| **Operating System** | Windows 10/11 (Windows 7 may work but not supported) |
| **Privileges** | Administrator (required for registry/system operations) |
| **Architecture** | 64-bit (x64) |
| **Disk Space** | At least 100 MB for the tool itself |
| **RAM** | 512 MB minimum |

### Build Requirements (if compiling from source)

| Requirement | Version |
|-------------|---------|
| **Rust** | 1.70 or later |
| **Cargo** | Comes with Rust |
| **Visual C++ Build Tools** | For Windows |
| **git** | For cloning the repository |

### Runtime Dependencies

The following are usually pre-installed on Windows:
- VC++ Redistributable (2015-2022)
- Windows APIs (user32, kernel32, etc.)

---

## Troubleshooting

### Common Issues and Solutions

| Issue | Cause | Solution |
|-------|-------|----------|
| **"Access Denied" errors** | Not running as Administrator | Right-click → Run as Administrator |
| **"Steam is still running"** | Steam processes active | Enable "Kill Steam Processes" option or close Steam manually |
| **"Could not find Steam installation"** | Steam in non-standard location | Make sure Steam is in standard location (Program Files) or the tool will search common paths |
| **Changes not taking effect** | Didn't restart computer | Restart your computer (REQUIRED for all changes) |
| **Backup creation failed** | Insufficient disk space or permissions | Free up disk space, ensure write permissions to backup location |
| **Tool won't start** | Missing VC++ Redistributable | Install [VC++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe) |
| **Windows Defender blocks it** | False positive | Click "More info" → "Run anyway" (or add to exclusions) |
| **MAC spoof not working** | Adapter doesn't support registry MAC change | Some adapters don't support this method. The tool will skip them. |

### Getting Help

If you encounter issues not listed above:

1. Check the log output in the right panel for specific error messages
2. Make sure you're running as Administrator
3. Try running in Simulation Mode first to identify the issue
4. Check Windows Event Viewer for crash logs
5. [Open an issue on GitHub](https://github.com/hutaoshusband/steam_cleaner/issues)

---

## Disclaimer

**This tool is provided for educational and testing purposes only.**

It performs destructive operations on your system that may result in:
- Loss of Steam account data and configurations
- Loss of game saves and settings
- Changes to system identifiers
- Deletion of system files and caches
- Being logged out of all Steam accounts

**The authors are not responsible for:**
- Any damage to your system
- Loss of data
- Steam account issues
- Game issues
- VAC bans or trust factor issues
- Any other problems resulting from the use of this tool

**This tool:**
- ❌ Does NOT modify firmware (BIOS, UEFI, SMBIOS)
- ❌ Does NOT perform kernel-level operations
- ❌ Does NOT interact with games or anti-cheat systems
- ❌ Is NOT a cheat or hack
- ✅ IS a system cleanup utility

**Use at your own risk. Always create backups before running.**

This is a system cleanup utility, similar to CCleaner but specifically designed for Steam and system identifiers. It helps remove local traces that may influence trust scores for new Steam accounts, but it cannot fix account-level links or behavior-based trust issues.

---

## Changelog

### Version 0.1.9 (Current)
- ✨ Added simulation mode (dry run) for previewing changes
- ✨ Added granular options for selective cleaning
- ✨ Added backup/restore functionality
- ✨ Added multi-language support (9 languages)
- ✨ Added orphaned game folder detection
- ✨ Added redistributable cleaner
- ✨ Added hardware profiles (save/load MAC, Volume ID)
- ✨ Added system inspector
- ✨ Improved logging with detailed output
- ✨ Enhanced GUI with real-time log streaming
- 🐛 Fixed various bugs

### Version 0.1.0
- 🎉 Initial release
- ✨ Basic cleaning and spoofing functionality
- ✨ MAC address spoofing
- ✨ Volume ID spoofing
- ✨ Registry cleaning

---

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Make your changes
4. Add tests if applicable
5. Ensure code follows Rust best practices
6. Update documentation as needed
7. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
8. Push to the branch (`git push origin feature/AmazingFeature`)
9. Open a Pull Request

**Please note:**
- This is a Windows-only tool
- Follow Rust best practices
- Add tests for new features
- Update documentation

---

## Support

If you find this tool helpful:

- ⭐ Star the repository on GitHub
- 🐛 Report bugs and issues
- 💡 Suggest new features
- 📖 Improve documentation

---

## License

This project is licensed under the MIT License – see the [LICENSE](LICENSE) file for full terms.

```
MIT License

Copyright (c) 2024 Steam Cleaner Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

**Made with ❤️ and ☕ by HUTAOSHUSBAND**
