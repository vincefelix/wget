use reqwest::Client;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::time::{sleep, Duration};
use futures::stream::{FuturesUnordered, StreamExt};
use std::fs::read_to_string;
use std::error::Error;
use chrono::Local;
use std::fs;


use crate::utils::log_to_file;


pub async fn download_single_file(
    url: &str, 
    file_name: Option<&str>, 
    directory: Option<&str>, 
    rate_limit: Option<&str>,  
    background: bool  
) -> Result<(), Box<dyn Error>> {


    let start_time = Local::now();
    if !background{
        println!("Start at: {}", start_time.format("%Y-%m-%d %H:%M:%S"));
    } else {
        log_to_file(&format!("Start at: {}", start_time.format("%Y-%m-%d %H:%M:%S")));
    }
    
    let client = Client::builder()
    .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
    .timeout(Duration::from_secs(600))
    .build()?;

    let response = client.get(url).send().await?;

    // Afficher le statut HTTP
    let status = response.status();

    if background{
        log_to_file(&format!("sending request, awaiting response... status {}", status));
    } else {
        println!("sending request, awaiting response... status {}", status);
    }

    if !status.is_success() {
        return Err(format!("Failed to download: {}. Status: {}", url, status).into());
    }

    let total_size = response.content_length().unwrap_or(0);
    let file_name = file_name.unwrap_or_else(|| url.split('/').last().unwrap());
    let save_path = if let Some(dir) = directory {
        format!("{}/{}", dir, file_name)
    } else {
        // Par défaut, créer le dossier "downloads" s'il n'existe pas
        fs::create_dir_all("downloads").expect("Failed to create downloads directory");
        format!("downloads/{}", file_name)
    };

    if background{
        log_to_file(&format!("content size: {} [~{:.2}MB]", total_size, total_size as f64 / (1024.0 * 1024.0)));
    } else {
        println!("content size: {} [~{:.2}MB]", total_size, total_size as f64 / (1024.0 * 1024.0));
    }
    if background {
        log_to_file(&format!("saving file to: {}", save_path));
    } else {
        println!("saving file to: {}", save_path);
    }

    let mut file = File::create(&save_path).await.expect("Failed to create file");
    let mut stream = response.bytes_stream();
    let mut downloaded = 0;

  // Gestion du rate-limit
  let rate_limit = if let Some(rate) = rate_limit {
    match parse_rate_limit(rate) {
        Ok(limit) => Some(limit),
        Err(e) => {
            eprintln!("Invalid rate limit: {}", e);
            None
        }
    }
} else {
    None
};



    // Barre de progression
    let pb = if !background {
        let pb = ProgressBar::new(total_size);
        let style = ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {wide_bar} {bytes}/{total_bytes} ({eta}) {bytes_per_sec}")
            .progress_chars("=>-");
        pb.set_style(style);
        Some(pb)
    } else {
        None
    };

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.expect("Error while downloading");
        file.write_all(&chunk).await.expect("Failed to write to file");
        downloaded += chunk.len() as u64;

        // Mise à jour de la barre de progression
        if let Some(pb) = &pb {
            pb.set_position(downloaded);
        }

        // Gestion du débit (rate limiting)
    if let Some(limit) = rate_limit {
        let chunk_size = chunk.len();
        let delay = chunk_size as u64 * 1500 / limit as u64; // Convertir en millisecondes
        sleep(Duration::from_millis(delay)).await;
    }

    }

    if let Some(pb) = &pb {
        pb.finish_with_message("Download complete");
    }
    if background {
        log_to_file(&format!("Downloaded [{}]", url));
    } else {
        println!("Downloaded [{}]", url);
    }

    // println!("Download completed: {}", save_path);
    Ok(())
}



pub async fn download_multiple_files(file_path: &str) -> Result<(), Box<dyn Error>> {
    let contents = read_to_string(file_path)?; 
    let urls: Vec<&str> = contents.lines().collect(); 
    
    let client = Client::new();
    let mut futures = FuturesUnordered::new();

    // Ajouter chaque téléchargement dans la file d'attente des tâches asynchrones
    for url in urls {
        let client = client.clone();
        let file_name = url.split('/').last().unwrap_or("unknown").to_string();

        futures.push(async move {
            let result = download_file_async(&client, url).await;
            match result {
                Ok(_) => println!("Finished downloading {}", file_name),
                Err(e) => eprintln!("Error downloading {}: {}", file_name, e),
            }
        });
    }

    // Traiter chaque téléchargement
    while futures.next().await.is_some() {}

    Ok(())
}

async fn download_file_async(client: &Client, url: &str) -> Result<(), Box<dyn Error>> {
   
    fs::create_dir_all("downloads").expect("Failed to create downloads directory");

   
    let response = client.get(url).send().await?;  // Envoie la requête HTTP
    let file_name = url.split('/').last().unwrap_or("unknown");

    let save_path = format!("downloads/{}", file_name);

    let mut file = File::create(&save_path).await?;     let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
    }

    Ok(())
}

fn parse_rate_limit(rate_limit: &str) -> Result<u64, String> {
    let len = rate_limit.len();
    if len == 0 {
        return Err("Invalid rate limit format".to_string());
    }
    let (num_part, unit) = rate_limit.split_at(len - 1);
    let num: u64 = num_part.parse().map_err(|_| "Invalid number in rate limit".to_string())?;
    match unit {
        "k" | "K" => Ok(num * 1024),
        "M" | "m" => Ok(num * 1024 * 1024),
        _ => Err("Invalid unit in rate limit, use 'k' or 'M'".to_string()),
    }
}