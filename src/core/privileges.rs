use std::process::Command;

use windows_sys::Win32::UI::Shell::ShellExecuteW;
use windows_sys::Win32::Foundation::HWND;
use std::env;


pub fn is_elevated() -> bool {
    let output = match Command::new("whoami").arg("/groups").output() {
        Ok(output) => output,
        Err(_) => return false, 
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
// admin level lol
    stdout.contains("S-1-16-12288")
}

/// Shows a native Windows error message.
pub fn show_admin_error_dialog() {
     let exe = env::current_exe().unwrap();
    let args: String = env::args().skip(1).collect::<Vec<_>>().join(" ");

    let exe_w: Vec<u16> = exe.to_string_lossy().encode_utf16().chain(Some(0)).collect();
    let args_w: Vec<u16> = args.encode_utf16().chain(Some(0)).collect();
    let verb = "runas\0".encode_utf16().collect::<Vec<u16>>();

    unsafe {
        ShellExecuteW(
            0 as HWND,
            verb.as_ptr(),
            exe_w.as_ptr(),
            args_w.as_ptr(),
            std::ptr::null(),
            1,
        );
    }

    std::process::exit(0);
}