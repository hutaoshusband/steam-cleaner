use eframe::{
    egui::{self, Align, Color32, FontId, RichText, Visuals},
    App,
};

use crate::{
    is_elevated, restart_as_admin, run_all,
    registry_cleaner, mac_spoofer, volumeid_wrapper, file_cleaner, sid_spoofer,
};

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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals::dark());


        const TITLE_FONT_SIZE: f32 = 30.0;
        const SUBTITLE_FONT_SIZE: f32 = 18.0;
        const BUTTON_HEIGHT: f32 = 48.0;
        const BUTTON_WIDTH: f32 = 180.0;
        const GROUP_PADDING: f32 = 14.0;
        const SPACING_LARGE: f32 = 28.0;
        const SPACING_MEDIUM: f32 = 14.0;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(SPACING_LARGE);


                ui.label(
                    RichText::new("Steam Cleaner by HUTAOSHUSBAND")
                        .font(FontId::proportional(TITLE_FONT_SIZE))
                        .color(Color32::from_rgb(150, 255, 150))
                        .strong(),
                );

                ui.add_space(SPACING_MEDIUM);


                ui.label(
                    RichText::new("Select the operations you want to run")
                        .font(FontId::proportional(SUBTITLE_FONT_SIZE))
                        .color(Color32::from_rgb(180, 200, 255)),
                );

                ui.add_space(SPACING_LARGE);


                egui::Frame::group(&egui::Style::default())
                    .fill(Color32::from_rgb(25, 25, 25))
                    .rounding(egui::Rounding::same(12.0))
                    .stroke(egui::Stroke::new(1.0, Color32::from_gray(70))) 
                    .inner_margin(egui::Margin::same(GROUP_PADDING))
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.set_min_width(420.0); 

                            ui.checkbox(&mut self.clean_registry, "Clean registry (MachineGuid & HWProfileGuid)");
                            ui.checkbox(&mut self.spoof_mac, "Spoof MAC address");
                            ui.checkbox(&mut self.change_volume_id, "Change Volume ID (C:)");
                            ui.checkbox(&mut self.clean_cache, "Clean cache files (Steam, CS2, DXCache, etc.)");
                            ui.checkbox(&mut self.spoof_hkcu, "Spoof suspicious HKCU keys");

                            ui.separator();

                            ui.checkbox(&mut self.run_all, RichText::new("Run ALL").strong());
                        });
                    });

                ui.add_space(SPACING_LARGE);


                ui.horizontal_centered(|ui| {
                    let execute_button = egui::Button::new(
                        RichText::new("Execute")
                            .font(FontId::proportional(18.0))
                            .color(Color32::BLACK),
                    )
                    .fill(Color32::from_rgb(80, 230, 80))
                    .min_size(egui::vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
                    .rounding(egui::Rounding::same(14.0));

                    if ui.add(execute_button).clicked() {
                        if !is_elevated() {
                            eprintln!("[!] Not running as administrator. Trying restart with elevation...");
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

                ui.add_space(SPACING_MEDIUM);
            });
        });
    }
}
