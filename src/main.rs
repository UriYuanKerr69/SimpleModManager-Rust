use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use zip::ZipArchive;
use sevenz_rust::{SevenZArchive, SevenZReader}; // Ensure these are correct
use sha2::{Sha256, Digest};
use eframe::egui;
use eframe::App;

struct ModManagerApp {
    config: Config,
    message: String,
}

impl ModManagerApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            config: Config::default(),
            message: String::new(),
        }
    }
}

impl App for ModManagerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mod Manager");
            ui.label("Your mod manager GUI here");
        });
    }
}

impl ModManagerApp {
    fn install_mod(&mut self, archive_path: &Path, mods_folder: &Path) -> io::Result<()> {
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
        let file = fs::File::open(archive_path)?;
        let mut archive = ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = match file.enclosed_name() {
                Some(path) => mods_folder.join(path),
                None => continue,
            };

            if file.name().ends_with('/') {
                fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p)?;
                    }
                }
                let mut outfile = fs::File::create(&outpath)?;
                io::copy(&mut file, &mut outfile)?;
            }
        }
        Ok(())
    }

    fn handle_7z(&self, archive_path: &Path, mods_folder: &Path) -> io::Result<()> {
        let reader = SevenZReader::open(&archive_path)?;
        let mut archive = SevenZArchive::new(&reader)?;

        for entry in archive.entries()? {
            let entry = entry?;
            let outpath = mods_folder.join(entry.name());

            if entry.is_directory() {
                fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p)?;
                    }
                }
                let mut outfile = fs::File::create(&outpath)?;
                let mut data = Vec::new();
                entry.read_to_end(&mut data)?;
                outfile.write_all(&data)?;
            }
        }
        Ok(())
    }
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Mod Manager",
        options,
        Box::new(|cc| Box::new(ModManagerApp::new(cc))),
    ).unwrap();
}
