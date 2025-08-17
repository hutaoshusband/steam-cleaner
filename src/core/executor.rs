// src/core/executor.rs

use crate::core::{file_cleaner, mac_spoofer, registry_cleaner, sid_spoofer, volumeid_wrapper};
use std::io;

#[derive(Debug, Clone, Copy)]
pub struct CleaningOptions {
    pub clean_registry: bool,
    pub spoof_mac: bool,
    pub change_volume_id: bool,
    pub clean_cache: bool,
    pub spoof_hkcu: bool,
}

pub async fn run_all_selected(options: CleaningOptions) -> Vec<String> {
    let mut results = Vec::new();
    println!("Starte asynchrone Bereinigung mit Optionen: {:?}", options);

    if options.clean_registry {
        match registry_cleaner::clean_registry() {
            Ok(_) => results.push("✅ Registry erfolgreich bereinigt.".to_string()),
            Err(e) => results.push(format!("❌ Fehler bei Registry-Bereinigung: {}", e)),
        }
    }

    if options.spoof_mac {
        match mac_spoofer::spoof_mac_all() {
            Ok(_) => results.push("✅ MAC-Adressen erfolgreich geändert.".to_string()),
            Err(e) => results.push(format!("❌ Fehler bei MAC-Spoofing: {}", e)),
        }
    }

    if options.change_volume_id {
        match volumeid_wrapper::change_volume_id("C") {
            Ok(_) => results.push("✅ Volume ID erfolgreich geändert.".to_string()),
            Err(e) => results.push(format!("❌ Fehler bei Volume ID-Änderung: {}", e)),
        }
    }
    
    if options.clean_cache {
        let result = tokio::task::spawn_blocking(file_cleaner::clean_cache).await;
        match result {
            Ok(Ok(_)) => results.push("✅ Cache-Dateien erfolgreich gelöscht.".to_string()),
            Ok(Err(e)) => results.push(format!("❌ Fehler bei Cache-Bereinigung: {}", e)),
            Err(_) => results.push("❌ Kritischer Fehler im Cache-Bereinigungs-Task.".to_string()),
        }
    }

    if options.spoof_hkcu {
        match sid_spoofer::spoof_hkcu() {
            Ok(_) => results.push("✅ HKCU-Schlüssel erfolgreich bereinigt.".to_string()),
            Err(e) => results.push(format!("❌ Fehler bei HKCU-Bereinigung: {}", e)),
        }
    }

    if results.is_empty() {
        results.push("ℹ️ Keine Operationen ausgewählt.".to_string());
    } else {
        results.push("-----------------------------------".to_string());
        results.push("✅ Alle Aufgaben abgeschlossen. Ein Neustart wird empfohlen.".to_string());
    }

    results
}
