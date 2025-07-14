use eframe::{
    egui::{self, Align, Color32, FontId, RichText, Visuals},
    App,
};

use crate::{is_elevated, restart_as_admin, run_all};
use crate::{registry_cleaner, mac_spoofer, volumeid_wrapper, file_cleaner, sid_spoofer};

#[derive(Default)]
pub struct CleanerApp {
    clean_registry: bool,
    spoof_mac: bool,
    change_volume_id: bool,
    clean_cache: bool,
    spoof_hkcu: bool,
    run_all: bool,
}

impl App for CleanerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals::dark());

        // Icon setzen entfernt

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(Align::Center), |ui| {
                ui.add_space(10.0);
                ui.heading(
                    RichText::new("Steam Cleaner by HUTAOSHUSBAND")
                        .font(FontId::proportional(28.0))
                        .color(Color32::LIGHT_GREEN),
                );
                ui.add_space(5.0);
                ui.label(
                    RichText::new("Select the operations to run")
                        .font(FontId::proportional(18.0))
                        .color(Color32::LIGHT_BLUE),
                );
                ui.add_space(15.0);

                ui.checkbox(&mut self.clean_registry, "🧹  Clean registry (MachineGuid, HWProfileGuid)");
                ui.checkbox(&mut self.spoof_mac, "🌐  Spoof MAC address");
                ui.checkbox(&mut self.change_volume_id, "💽  Change Volume ID (C:)");
                ui.checkbox(&mut self.clean_cache, "🗑️  Clean cache files (Steam, CS2, DXCache, etc.)");
                ui.checkbox(&mut self.spoof_hkcu, "🧬  Spoof HKCU suspicious keys");
                ui.checkbox(&mut self.run_all, "🚀  Run ALL");

                ui.add_space(20.0);

                ui.horizontal_centered(|ui| {
                    if ui.add_sized([140.0, 45.0], egui::Button::new("Execute")).clicked() {
                        if !is_elevated() {
                            println!("[!] Not running as administrator. Attempting elevation...");
                            restart_as_admin();
                            return;
                        }

                        if self.run_all {
                            run_all();
                        } else {
                            if self.clean_registry {
                                match registry_cleaner::clean_registry() {
                                    Ok(_) => println!("[+] Registry cleaned successfully."),
                                    Err(e) => eprintln!("[!] Registry cleaning failed: {}", e),
                                }
                            }
                            if self.spoof_mac {
                                match mac_spoofer::spoof_mac_all() {
                                    Ok(_) => println!("[+] MAC spoofed successfully."),
                                    Err(e) => eprintln!("[!] MAC spoofing failed: {}", e),
                                }
                            }
                            if self.change_volume_id {
                                match volumeid_wrapper::change_volume_id("C") {
                                    Ok(_) => println!("[+] Volume ID changed successfully."),
                                    Err(e) => eprintln!("[!] Volume ID change failed: {}", e),
                                }
                            }
                            if self.clean_cache {
                                match file_cleaner::clean_cache() {
                                    Ok(_) => println!("[+] Cache cleaned successfully."),
                                    Err(e) => eprintln!("[!] Cache cleaning failed: {}", e),
                                }
                            }
                            if self.spoof_hkcu {
                                match sid_spoofer::spoof_hkcu() {
                                    Ok(_) => println!("[+] HKCU spoofed successfully."),
                                    Err(e) => eprintln!("[!] HKCU spoofing failed: {}", e),
                                }
                            }
                        }

                        println!("[~] Task(s) completed. Restart required.");
                    }
                });

                ui.add_space(20.0);
            });
        });
    }
}
