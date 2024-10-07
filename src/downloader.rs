use std::path::Path;
use std::fs::{self, File};
use std::io::Write;
use chrono::Local;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::get;

use crate::utils::{format_size, get_file_name};

// Ajout des paramètres `output_file` et `output_dir` pour les options des flags
pub fn download_file(url: &str, output_file: Option<String>, output_dir: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Local::now();
    println!("start at {}", start_time.format("%Y-%m-%d %H:%M:%S"));

    // Requête HTTP pour récupérer le fichier
    println!("sending request, awaiting response...");
    let response = get(url)?;

    if !response.status().is_success() {
        eprintln!("Error: received status {}", response.status());
        std::process::exit(1);
    }
    println!("status {}", response.status());

    let content_length = response
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|len| len.to_str().ok())
        .and_then(|len| len.parse().ok())
        .unwrap_or(0);

    println!(
        "content size: {} [{}]",
        content_length,
        format_size(content_length)
    );

   // Déterminer le nom du fichier et le répertoire de sortie
let file_name = output_file.unwrap_or_else(|| get_file_name(url).to_string());

// Créer une String pour `output_dir` si nécessaire afin qu'elle vive assez longtemps
let download_dir_string = output_dir.unwrap_or_else(|| "downloads".to_string());
let download_dir = Path::new(&download_dir_string);


    // Créer le répertoire si nécessaire
    if !download_dir.exists() {
        fs::create_dir_all(download_dir)?;
    }

    let file_path = download_dir.join(file_name);

    // Sauvegarder le fichier
    let mut file = File::create(&file_path)?;
    println!("saving file to: {:?}", file_path);

    let pb = ProgressBar::new(content_length);
    pb.set_style(ProgressStyle::default_bar()
        .template("{bytes}/{total_bytes} [{wide_bar}] {percent}% {bytes_per_sec} {eta}")
        .progress_chars("=>-"));

    let content = response.bytes()?;
    let mut downloaded = 0;
    while downloaded < content_length {
        let chunk_size = std::cmp::min(8192, content_length - downloaded);
        let chunk = &content[downloaded as usize..(downloaded + chunk_size) as usize];
        file.write_all(chunk)?;
        pb.inc(chunk_size as u64);
        downloaded += chunk_size;
    }

    pb.finish_with_message("download completed");

    let end_time = Local::now();
    println!("Downloaded [{}]", url);
    println!("finished at {}", end_time.format("%Y-%m-%d %H:%M:%S"));

    Ok(())
}
