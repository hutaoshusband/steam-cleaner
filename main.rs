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

use raw_window_handle::HasRawWindowHandle;
mod registry_cleaner;
mod mac_spoofer;
mod volumeid_wrapper;
mod file_cleaner;
mod sid_spoofer;
mod win_icon {
    use windows_sys::Win32::Foundation::HWND;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        LoadImageW, SendMessageW, IMAGE_ICON, LR_DEFAULTSIZE, LR_LOADFROMFILE,
        WM_SETICON, ICON_BIG, ICON_SMALL,
    };
    use widestring::U16CString;

    pub fn set_window_icon(hwnd: HWND, icon_path: &str) {
        unsafe {
            let icon = LoadImageW(
                0,
                U16CString::from_str(icon_path).unwrap().as_ptr(),
                IMAGE_ICON,
                0,
                0,
                LR_LOADFROMFILE | LR_DEFAULTSIZE,
            );
            if icon != 0 {
                SendMessageW(hwnd, WM_SETICON, ICON_BIG as usize, icon as isize);
                SendMessageW(hwnd, WM_SETICON, ICON_SMALL as usize, icon as isize);
            }
        }
    }
}

mod ui;


use eframe::{NativeOptions, run_native};
use std::process::Command;
use image::ImageReader;



fn main() -> eframe::Result<()> {
    if !is_elevated() {
        restart_as_admin();
     }
    let native_options = NativeOptions::default();

    let result = run_native(
        "Steam Cleaner by HUTAOSHUSBAND 💀",
        native_options,
        Box::new(|cc| {

            Box::new(ui::CleanerApp::default())
        }),
    );

    println!("Press Enter to exit...");
    let _ = std::io::stdin().read_line(&mut String::new());

    result
}


pub(crate) fn is_elevated() -> bool {
    let output = Command::new("whoami")
        .arg("/groups")
        .output()
        .expect("Failed to execute 'whoami /groups'");

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.contains("S-1-16-12288")
}

pub(crate) fn restart_as_admin() {
    use widestring::U16CString;
    use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK};
    use windows_sys::Win32::Foundation::HWND;

    let title = U16CString::from_str("Steam Cleaner").unwrap();
    let message = U16CString::from_str("Please run the loader as administrator.").unwrap();

    unsafe {
        MessageBoxW(
            0,
            message.as_ptr(),
            title.as_ptr(),
            MB_OK,
        );
    }

    std::process::exit(1);
}


pub(crate) fn run_all() {
    println!("[*] Running all operations...");

    match registry_cleaner::clean_registry() {
        Ok(_) => println!("[+] Registry cleaned successfully."),
        Err(e) => eprintln!("[!] Registry cleaning failed: {}", e),
    }

    match mac_spoofer::spoof_mac_all() {
        Ok(_) => println!("[+] MAC spoofed successfully."),
        Err(e) => eprintln!("[!] MAC spoofing failed: {}", e),
    }

    match volumeid_wrapper::change_volume_id("C") {
        Ok(_) => println!("[+] Volume ID changed successfully."),
        Err(e) => eprintln!("[!] Volume ID change failed: {}", e),
    }

    match file_cleaner::clean_cache() {
        Ok(_) => println!("[+] Cache cleaned successfully."),
        Err(e) => eprintln!("[!] Cache cleaning failed: {}", e),
    }

    match sid_spoofer::spoof_hkcu() {
        Ok(_) => println!("[+] HKCU spoofed successfully."),
        Err(e) => eprintln!("[!] HKCU spoofing failed: {}", e),
    }

    println!("[~] Finished. Please restart your system.");
}
