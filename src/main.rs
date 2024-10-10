mod cli;
mod downloader;
mod utils;

use std::fs::OpenOptions;
use std::io::Write;
use std::thread;

fn main() {
    let cli_options = cli::parse_cli();  // Parse les options CLI
    
    // Gestion du rate-limit
    let rate_limit = if let Some(rate_limit_str) = cli_options.rate_limit {
        match utils::parse_rate_limit(&rate_limit_str) {
            Ok(limit) => Some(limit),
            Err(e) => {
                eprintln!("Invalid rate limit: {}", e);
                return;
            }
        }
    } else {
        None
    };

    if cli_options.background {
        println!("Output will be written to \"wget-log\".");
        let log_file_path = "wget-log";

        // Télécharger le fichier en arrière-plan
        let url = cli_options.url.clone();
        let output_file = cli_options.output_file.clone();
        let output_dir = cli_options.output_dir.clone();
        let rate_limit = rate_limit.clone();

        // Lancer le téléchargement dans un thread en arrière-plan
        let handle = thread::spawn(move || {
            // Ouvrir le fichier log pour écrire
            let mut log_file = OpenOptions::new()
                .create(true)
                .append(true)
                .write(true)
                .open(log_file_path)
                .expect("Unable to open log file");

            // Passer l'action de téléchargement avec le fichier de log
            let result = downloader::download_file(&url, output_file, output_dir, rate_limit, true, &mut log_file);
            
            // Si une erreur survient, l'écrire dans le fichier de log
            if let Err(e) = result {
                writeln!(log_file, "Failed to download the file: {}", e).expect("Unable to write to log");
            }
        });

        // Attendre que le thread finisse
        handle.join().expect("Failed to join the background thread");
    } else {
        // Exécution normale sans fichier log
        if let Err(e) = downloader::download_file(&cli_options.url, cli_options.output_file, cli_options.output_dir, rate_limit, false, &mut std::io::stdout()) {
            eprintln!("Failed to download the file: {}", e);
        }
    }
}
