use reqwest::Client;
use scraper::{Html, Selector};
use std::fs::create_dir_all;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::collections::HashSet;

pub async fn mirror_website(
    url: &str, 
    reject_types: Option<&str>, 
    exclude_dirs: Option<&str>, 
    convert_links: bool
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    
    // Si la requête réussit
    if response.status().is_success() {
        let html = response.text().await?;
        let document = Html::parse_document(&html);

        // Créer un répertoire local basé sur le nom de domaine
        let domain = url.split("://").nth(1).unwrap().split('/').next().unwrap();
        let dir_name = format!("./{}", domain);
        create_dir_all(&dir_name)?;

        // Sauvegarder la page HTML principale
        let html_file_path = format!("{}/index.html", &dir_name);
        let mut file = File::create(&html_file_path).await?;
        let mut modified_html = html.clone(); // On va modifier le HTML si `convert_links` est activé

        // Sélectionner les liens, images, et fichiers CSS
        let link_selector = Selector::parse("a[href], img[src], link[href]").unwrap();

        // Récupérer les types de fichiers rejetés
        let reject_types_set = if let Some(types) = reject_types {
            types.split(',').collect::<HashSet<&str>>() // Conserve les extensions rejetées dans un ensemble
        } else {
            HashSet::new()
        };

        // Exclure certains répertoires
        let exclude_dirs_set = if let Some(dirs) = exclude_dirs {
            dirs.split(',').collect::<HashSet<&str>>() // Conserve les dossiers exclus dans un ensemble
        } else {
            HashSet::new()
        };

        for element in document.select(&link_selector) {
            let attr = if element.value().name() == "img" {
                element.value().attr("src")
            } else {
                element.value().attr("href")
            };

            if let Some(link) = attr {
                // Filtrage par types de fichiers rejetés
                if let Some(extension) = link.split('.').last() {
                    if reject_types_set.contains(extension) {
                        println!("Skipping file: {} due to reject rules.", link);
                        continue;
                    }
                }

                // Filtrage par répertoires exclus
                if let Some(folder) = link.split('/').nth(1) {
                    if exclude_dirs_set.contains(folder) {
                        println!("Skipping folder: {} due to exclusion rules.", folder);
                        continue;
                    }
                }

                // Si le lien commence par "http", on le télécharge
                if link.starts_with("http") {
                    let file_url = link.to_string();
                    let file_name = link.split('/').last().unwrap();
                    let save_path = format!("{}/{}", dir_name, file_name);

                    // Téléchargement de la ressource
                    download_resource(&client, &file_url, &save_path).await?;

                    // Convertir les liens pour utilisation hors ligne si `convert_links` est activé
                    if convert_links {
                        modified_html = modified_html.replace(link, &save_path);
                    }
                }
            }
        }

        // Si les liens doivent être convertis, sauvegarder la version modifiée du HTML
        if convert_links {
            file.write_all(modified_html.as_bytes()).await?;
        } else {
            file.write_all(html.as_bytes()).await?;
        }

        println!("Mirroring completed: {}", dir_name);
        Ok(())
    } else {
        eprintln!("Error: Failed to fetch the website. Status: {}", response.status());
        Err(Box::from("Failed to download the website"))
    }
}

// Téléchargement des ressources individuelles
async fn download_resource(client: &Client, file_url: &str, save_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = client.get(file_url).send().await?;

    if response.status().is_success() {
        let mut file = File::create(save_path).await?;
        let content = response.bytes().await?;
        file.write_all(&content).await?;
        println!("Downloaded: {}", save_path);
    } else {
        eprintln!("Error downloading: {}", file_url);
    }

    Ok(())
}
