use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use chrono::Local;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use crate::utils::{format_size, get_file_name};

pub fn download_file(
    url: &str, 
    output_file: Option<String>, 
    output_dir: Option<String>, 
    rate_limit: Option<u64>, 
    is_background: bool,  
    log_file: &mut dyn Write  
) -> Result<(), Box<dyn std::error::Error>> {

    let start_time = Local::now();
    if !is_background {
        println!("start at {}", start_time.format("%Y-%m-%d %H:%M:%S"));
    } else {
        writeln!(log_file, "start at {}", start_time.format("%Y-%m-%d %H:%M:%S"))?;
    }

    // Créer un client HTTP avec un User-Agent
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .timeout(Duration::from_secs(600))
        .build()?;

    // Requête HTTP pour récupérer le fichier
    if !is_background {
        println!("sending request, awaiting response...");
    } else {
        writeln!(log_file, "sending request, awaiting response...")?;
    }
    
    let response = client.get(url).send()?;

    if !response.status().is_success() {
        eprintln!("Error: received status {}", response.status());
        std::process::exit(1);
    }

    if !is_background {
        println!("status {}", response.status());
    } else {
        writeln!(log_file, "status {}", response.status())?;
    }

    let content_length = response
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|len| len.to_str().ok())
        .and_then(|len| len.parse().ok())
        .unwrap_or(0);

    if !is_background {
        println!(
            "content size: {} [{}]",
            content_length,
            format_size(content_length)
        );
    } else {
        writeln!(
            log_file,
            "content size: {} [{}]",
            content_length,
            format_size(content_length)
        )?;
    }

    let file_name = output_file.unwrap_or_else(|| get_file_name(url).to_string());
    let download_dir_string = output_dir.unwrap_or_else(|| "downloads".to_string());
    let download_dir = Path::new(&download_dir_string);

    if !download_dir.exists() {
        fs::create_dir_all(download_dir)?;
    }

    let file_path = download_dir.join(file_name);
    let mut file = File::create(&file_path)?;

    if !is_background {
        println!("saving file to: {:?}", file_path);
    } else {
        writeln!(log_file, "saving file to: {:?}", file_path)?;
    }

    // Initialiser la barre de progression seulement si on n'est pas en arrière-plan
    let pb = if !is_background {
        Some(ProgressBar::new(content_length))
    } else {
        None
    };

    if let Some(ref pb) = pb {
        pb.set_style(ProgressStyle::default_bar()
            .template("{bytes}/{total_bytes} [{wide_bar}] {percent}% {bytes_per_sec} {eta}")
            .progress_chars("=>-"));
    }

    let content = response.bytes()?;
    let mut downloaded = 0;

    while downloaded < content_length {
        let chunk_size = std::cmp::min(8192, content_length - downloaded);
        let chunk = &content[downloaded as usize..(downloaded + chunk_size) as usize];
        file.write_all(chunk)?;
        
        // Mettre à jour la barre de progression seulement si elle existe
        if let Some(ref pb) = pb {
            pb.inc(chunk_size as u64);
        }

        downloaded += chunk_size;

        // Limiter la vitesse de téléchargement si `rate_limit` est spécifié
        if let Some(limit) = rate_limit {
            let delay = chunk_size as u64 * 1000 / limit; // Convertir en millisecondes
            sleep(Duration::from_millis(delay));
        }
    }

    // Finaliser la barre de progression seulement si elle existe
    if let Some(ref pb) = pb {
        pb.finish_with_message("download completed");
    }

    if !is_background {
        println!("Downloaded [{}]", url);
        println!("finished at {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
    } else {
        writeln!(log_file, "Downloaded [{}]", url)?;
        writeln!(log_file, "finished at {}", Local::now().format("%Y-%m-%d %H:%M:%S"))?;
    }

    Ok(())
}
