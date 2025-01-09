use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use zip::ZipArchive;
use sevenz_rust::{SevenZArchive, SevenZReader};
use sha2::{Sha256, Digest};
use eframe::egui;
use eframe::App;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize, Default)]
struct Config {
    archive_path: String,
    mods_folder: String,
}

struct ModManagerApp {
    config: Config,
    message: String,
}

impl ModManagerApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self {
            config: Config::default(),
            message: String::new(),
        };
        app.load_config().unwrap_or_else(|e| {
            app.message = format!("Failed to load config: {}", e);
        });
        app
    }

    fn load_config(&mut self) -> io::Result<()> {
        let config_path = Path::new("mod_manager_config.json");
        if config_path.exists() {
            let config_data = fs::read_to_string(config_path)?;
            self.config = serde_json::from_str(&config_data)?;
        }
        Ok(())
    }

    fn save_config(&self) -> io::Result<()> {
        let config_path = Path::new("mod_manager_config.json");
        let json = serde_json::to_string_pretty(&self.config)?;
        let mut file = File::create(config_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}

impl App for ModManagerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mod Manager");

            if ui.button("Select Archive").clicked() {
                if let Some(path) = FileDialog::new().pick_file() {
                    self.config.archive_path = path.to_string_lossy().to_string();
                }
            }
            ui.label(format!("Selected Archive: {}", self.config.archive_path));

            if ui.button("Select Mods Folder").clicked() {
                if let Some(path) = FileDialog::new().pick_folder() {
                    self.config.mods_folder = path.to_string_lossy().to_string();
                }
            }
            ui.label(format!("Selected Mods Folder: {}", self.config.mods_folder));

            if ui.button("Install Mod").clicked() {
                let archive_path = Path::new(&self.config.archive_path);
                let mods_folder = Path::new(&self.config.mods_folder);
                match self.install_mod(archive_path, mods_folder) {
                    Ok(_) => {
                        self.message = "Installation completed!".to_string();
                        self.save_config().unwrap_or_else(|e| self.message = format!("Failed to save config: {}", e));
                    },
                    Err(e) => self.message = format!("Error: {}", e),
                }
            }

            ui.label(&self.message);
        });
    }
}

impl ModManagerApp {
    fn install_mod(&self, archive_path: &Path, mods_folder: &Path) -> io::Result<()> {
        fs::create_dir_all(mods_folder)?;

        let extension = archive_path.extension().and_then(|s| s.to_str()).unwrap_or("");

        match extension {
            "zip" => self.handle_zip(archive_path, mods_folder),
            "7z" => self.handle_7z(archive_path, mods_folder),
            _ => {
                self.message = format!("Unsupported archive format: {}", extension);
                Err(io::Error::new(io::ErrorKind::InvalidData, "Unsupported archive format"))
            }
        }
    }

    fn handle_zip(&self, archive_path: &Path, mods_folder: &Path) -> io::Result<()> {
        // ... (implementation as before)
    }

    fn handle_7z(&self, archive_path: &Path, mods_folder: &Path) -> io::Result<()> {
        // ... (implementation as before)
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Mod Manager",
        options,
        Box::new(|cc| Box::new(ModManagerApp::new(cc))),
    )
}
