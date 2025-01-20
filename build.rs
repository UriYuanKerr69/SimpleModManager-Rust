fn main() {
    #[cfg(target_os = "windows")]
    {
        if let Err(e) = cargo_winres::build() {
            eprintln!("Failed to build resources: {}", e);
            std::process::exit(1);
        }
    }
}
