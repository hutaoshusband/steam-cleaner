use std::process::Command;
use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK};
use widestring::U16CString;


pub fn is_elevated() -> bool {
    let output = match Command::new("whoami").arg("/groups").output() {
        Ok(output) => output,
        Err(_) => return false, 
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
// admin level lol
    stdout.contains("S-1-16-12288")
}

/// Zeigt eine native Windows-Fehlermeldung an.
pub fn show_admin_error_dialog() {
    let title = U16CString::from_str("Error: Admin missing").unwrap();
    let message = U16CString::from_str(
        "This Program needs to be ran as admin.\n\nBitte rechtsklicken Sie die .exe und wählen Sie 'Als Administrator ausführen'.",
    )
    .unwrap();

    unsafe {
        MessageBoxW(0, message.as_ptr(), title.as_ptr(), MB_OK | MB_ICONERROR);
    }
}