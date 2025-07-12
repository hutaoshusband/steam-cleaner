// ++++++++++++++++++++++++++++++++++++++++++++**+++++++++++++++++++++++++++++++++++
// +++++++++++++++++++++++++++++++++++*+++++++++++++++++++++++++++++++++++++++++++++
// +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
// ====+==++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++=+
// ==========++++++++++++++++=+++++++++++++++++++++++++++++++++++++++++++++=+=======
// =============+======++=++++++++*###%#%%%###***+++++++++++++++++++++++===+=+======
// ==================+=======+++*%%%%########***###**+++++++++++++++================
// =========================+*#%%%%******+++++++*#####*++++++++++++=================
// ========================*#%%%%*****++++*++**==+**#***+++++++++===================
// =======================*%%%#*#***+*++=++**+***********+==+==++===================
// =+===================+*#%%#*+*#**++++==========+++++***==========================
// =====================*###**+*#*+++=====-----------==++*+=========================
// ====================+#***++**++=====---------------===++=========================
// ====================**+*+=====----====-----:::-----====++========================
// ====================**++=====---=====------::::--------==========================
// ====================*++======-----------------------:::--========================
// ====================+=--============----===---------::::--=======-===========----
// =====================---===========-====----:::------::::-=========-=======------
// ====================----======--------==-----==++=-----:--===============--------
// ====================---==+*#*##*+++=-===+**#*++====-----:--==============-=--=---
// =========================+*==+=+****+=---*##*%###=---------===-======-=-----=----    // Cleaner made by HUTAOSHUSBAND
// =====-==---==========*===+++%%*###@#++=--=##*+=====-----=--==--------------------    // Image represents Fred.
// =====---=============+=+=++*+==+*#*+++=---==+**+=-----=-=+-==--------------------
// -======================+=====++++*++++==---=+===------===--=---=-----------------
// --=====================+======++*#*++==--==+#*=-------===-----=------------------
// ---====================++=====+*@%++===----=+##+=--------------------------------
// -==--=-==-===-================*%#*+**+++======+#+=----=--------------------------
// ----==-=================++++++*+==++++++=-------++=------==----------------------
// -----====================+++++++++++++++=======--==------=@@*=-------------------
// ---=======================++++++++####*****++=====-------::@@@+------------------
// ===========================+++++++++++++++====-=+==--=---..:@@@@*----------------
// ===========================++++++++++++++=====--=====---....#@@@@@#=-------------
// ==========================%@*++++++++++===------=====--.....#@@@@@@@+------------
// ===================+*#%@@@@@--++++++++++===-=--==+==-:.....:@@@@@@@@@@@#=--------
// =============+@@@@@@@@@@@@@..-==++++++++++======++=-.......=@@@@@@@@@@@@@@%*-----
// ===========#@@@@@@@@@@@@@%..:-=====+++++++++======-....:.:.*@@@@@@@@@@@@@@@@@@%+-
// =========*@@@@@@@@@@@@@@%...:--+======+++======+*-.....:...@@@@@@@@@@@@@@@@@@@@@@
// ========*@@@@@@@@@@@@@@*....::-=%============+#+........:.%@@@@@@@@@@@@@@@@@@@@@@
// =======*@@@@@@@@@@@@@@@-::...::-*#======++++**-......+.=.*@@@@@@@@@@@@@@@@@@@@@@@
// ==+===+@@@@@@@@@@@@@@@@@=::....:+##+====++++=:......-=+=-@@@@@@@@@@@@@@@@@@@@@@@@
// ======%@@@@@@@@@@@@@@@@@@%-.....:*#*+======:........--=:#@@@@@@@@@@@@@@@@@@@@@@@@
// =====*@@@@@@@@@@@@@@@@@@@*.......-***++==-.:---:.......=@@@@@@@@@%%@@@@@@@@@@@@@@
// ====+@@@@@@@@@@@@@@@@@@@%........:*****=...::----:-=-..#@@@@@@@@@@@@@@@@@@@@@@@@@
// ==++#@@@@@@@@@@@@@@@@@@%..........:*##-...:::--:..:::::@@@@@@@@@@@@@@@@@@@@@@@@@@  

mod registry_cleaner;
mod mac_spoofer;
mod volumeid_wrapper;
mod file_cleaner;
mod sid_spoofer;



use std::io::{self, Write};

use std::process::Command;

fn is_elevated() -> bool {
    let output = Command::new("whoami")
        .arg("/groups")
        .output()
        .expect("Failed to execute 'whoami /groups'");

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.contains("S-1-16-12288") // SID
}

fn restart_as_admin() {
    let exe_path = std::env::current_exe().unwrap();
    let exe_path_str = exe_path.to_str().unwrap();

    let _ = Command::new("powershell")
        .args([
            "-Command",
            &format!(
                "Start-Process -FilePath '{}' -Verb runAs",
                exe_path_str
            ),
        ])
        .status();

    std::process::exit(0);
}

fn prompt_restart() {
    use std::io::{self, Write};

    println!("\n[!] A system restart is required for all changes to take effect.");
    print!("[*] Do you want to restart now? (y/n): ");
    io::stdout().flush().unwrap();

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();

    if answer.trim().eq_ignore_ascii_case("y") {
        println!("[*] Restarting system...");
        let _ = std::process::Command::new("shutdown")
            .args(["/r", "/t", "0"])
            .spawn();
    } else {
        println!("[~] Please restart manually later.");
    }
}


fn main() {
    if !is_elevated() {
        println!("[!] Please run this program as administrator.");
        println!("[*] Attempting to relaunch with admin privileges...");
        restart_as_admin();
    }

    println!("============================================");
    println!("        Steam Cleaner by HUTAOSHUSBAND");
    println!("============================================\n");

    println!("Select the operations you want to run:");
    println!("1. Clean registry (MachineGuid, HWProfileGuid)");
    println!("2. Spoof MAC address");
    println!("3. Change Volume ID");
    println!("4. Clean cache files (Steam, CS2, DXCache, etc.)");
    println!("5. Run ALL");
    println!("6. Spoof / clean HKCU suspicious keys");
    print!("\nEnter the numbers (e.g. 1 3 4 5): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let choices: Vec<u8> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<u8>().ok())
        .collect();

    if choices.contains(&5) {
        run_all();
    } else {
        for choice in choices {
            match choice {
                1 => {
                    if let Err(e) = registry_cleaner::clean_registry() {
                        eprintln!("[!] Registry cleaning failed: {}", e);
                    }
                }
                2 => {
                    if let Err(e) = mac_spoofer::spoof_mac_all() {
                        eprintln!("[!] MAC spoofing failed: {}", e);
                    }
                }
                3 => {
                    if let Err(e) = volumeid_wrapper::change_volume_id("C") {
                        eprintln!("[!] Volume ID change failed: {}", e);
                    }
                }
                4 => {
                    if let Err(e) = file_cleaner::clean_cache() {
                        eprintln!("[!] Cache cleaning failed: {}", e);
                    }
                }
                6 => {
                    if let Err(e) = sid_spoofer::spoof_hkcu() {
                        eprintln!("[!] HKCU spoofing failed: {}", e);
                    }
                    }

                _ => {
                    eprintln!("Invalid option: {}", choice);
                }
            }
        }
    }

    println!("\n[+] Done.");
    prompt_restart();
}

fn run_all() {
    println!("[*] Running all cleaning tasks...");

    if let Err(e) = registry_cleaner::clean_registry() {
        eprintln!("[!] Registry cleaning failed: {}", e);
    }

    if let Err(e) = mac_spoofer::spoof_mac_all() {
        eprintln!("[!] MAC spoofing failed: {}", e);
    }

    if let Err(e) = volumeid_wrapper::change_volume_id("C") {
        eprintln!("[!] Volume ID change failed: {}", e);
    }

    if let Err(e) = file_cleaner::clean_cache() {
        eprintln!("[!] Cache cleaning failed: {}", e);
    }

    if let Err(e) = sid_spoofer::spoof_hkcu() {
    eprintln!("[!] HKCU spoofing failed: {}", e);
    }

}
