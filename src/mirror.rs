use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::{HashSet, VecDeque};
use std::fs::create_dir_all;
use std::path::Path;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use url::Url;
use std::sync::{Arc, Mutex};

// Fonction pour le mirroring récursif d'un site
pub async fn mirror_website(
    url: &str,
    reject_types: Option<&str>,
    exclude_dirs: Option<&str>,
    convert_links: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let visited_urls = Arc::new(Mutex::new(HashSet::new()));

    async fn mirror_recursive(
        url: &str,
        reject_types: Option<&str>,
        exclude_dirs: Option<&str>,
        convert_links: bool,
        visited_urls: Arc<Mutex<HashSet<String>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut visited = visited_urls.lock().unwrap();
        if visited.contains(url) {
            println!("Skipping already visited URL: {}", url);
            return Ok(());
        }
        visited.insert(url.to_string());
        drop(visited);

        let client = Client::new();
        let response = client.get(url).send().await?;

        if response.status().is_success() {
            let html = response.text().await?;
            let document = Html::parse_document(&html);

            let domain = url.split("://").nth(1).unwrap().split('/').next().unwrap();
            let relative_url = url.trim_start_matches("http://").trim_start_matches("https://");
            let dir_name = format!("./{}", relative_url);
            create_dir_all(&dir_name)?;

            let html_file_path = format!("{}/index.html", &dir_name);
            let mut file = File::create(&html_file_path).await?;
            let mut modified_html = html.clone();

            let link_selector = Selector::parse("a[href], img[src], link[href]").unwrap();
            let reject_types_set = if let Some(types) = reject_types {
                types.split(',').collect::<HashSet<&str>>()
            } else {
                HashSet::new()
            };

            let exclude_dirs_set = if let Some(dirs) = exclude_dirs {
                dirs.split(',').collect::<HashSet<&str>>()
            } else {
                HashSet::new()
            };

            let base_url = Url::parse(url)?;
            let mut subpages_to_visit: VecDeque<String> = VecDeque::new();

            // Remplacement des liens dans les balises a, img, link
            for element in document.select(&link_selector) {
                let attr = match element.value().name() {
                    "img" => element.value().attr("src"),
                    "a" => element.value().attr("href"),
                    "link" => element.value().attr("href"),
                    _ => None,
                };

                if let Some(link) = attr {
                    if let Some(extension) = link.split('.').last() {
                        if reject_types_set.contains(extension) {
                            println!("Skipping file: {} due to reject rules.", link);
                            continue;
                        }
                    }

                    if let Some(folder) = extract_parent_directory(link) {
                        println!("{link}");
                        let folder_path = format!("/{}", folder);
                        if exclude_dirs_set.contains(folder_path.as_str()) {
                            
                            continue;
                        }
                    }

                    let file_url = match Url::parse(link) {
                        Ok(url) => url.to_string(),
                        Err(_) => base_url.join(link)?.to_string(),
                    };

                    let file_name = link.split(&domain).last().unwrap();
                    let save_path = format!("{}/{}", dir_name, file_name);

                    if !file_url.contains("#") {

                    if file_url.contains(&domain) && (link.ends_with('/') || !file_name.contains('.')) {
                        subpages_to_visit.push_back(file_url.clone());
                        // Si c'est un répertoire, ajouter "/index.html" pour les liens convertis
                        if convert_links {
                            let relative_path = format!("./{}/index.html", link.trim_start_matches('/'));
                            let cleaned_path = clean_path_segments(&relative_path);
                            modified_html = replace_exact_link(&modified_html, link, &cleaned_path);
                        }
                    } else if file_url.contains(&domain) {
                        download_resource(&client, &file_url, &save_path).await?;

                        if convert_links {
                            // Eviter de doubler les chemins relatifs (comme "./index.html")
                            let relative_path = format!("./{}", file_name);
                            let cleaned_path = clean_path_segments(&relative_path);
                            modified_html = replace_exact_link(&modified_html, link, &cleaned_path);
                        }
                    }
                }
                
                }
            }

            // Gérer les balises <style> et ressources CSS
            let style_selector = Selector::parse("style").unwrap();
            for element in document.select(&style_selector) {
                let style_content = element.inner_html();
                let updated_style = handle_css_resources(&client, &base_url, &dir_name, &style_content).await?;
                modified_html = modified_html.replace( &style_content, &updated_style);
            }

            // Sauvegarder le HTML modifié avec les liens convertis
            if convert_links {
                file.write_all(modified_html.as_bytes()).await?;
            } else {
                file.write_all(html.as_bytes()).await?;
            }

            while let Some(subpage_url) = subpages_to_visit.pop_front() {
                Box::pin(mirror_recursive(&subpage_url, reject_types, exclude_dirs, convert_links, Arc::clone(&visited_urls))).await?;
            }

            println!("Mirroring completed: {}", dir_name);
            Ok(())
        } else {
            eprintln!(
                "Error: Failed to fetch the website. Status: {}",
                response.status()
            );
            Err(Box::from("Failed to download the website"))
        }
    }

    mirror_recursive(url, reject_types, exclude_dirs, convert_links, visited_urls).await
}

// Téléchargement des ressources individuelles
async fn download_resource(
    client: &Client,
    file_url: &str,
    save_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = client.get(file_url).send().await?;

    if response.status().is_success() {
        let save_path = Path::new(save_path);

        if let Some(parent) = save_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let mut file = File::create(save_path).await?;
        let content = response.bytes().await?;
        file.write_all(&content).await?;
        println!("Downloaded: {}", save_path.display());
    } else {
        eprintln!("Error downloading: {}", file_url);
    }

    Ok(())
}

// Gérer et convertir les ressources CSS
async fn handle_css_resources(
    client: &Client,
    base_url: &Url,
    dir_name: &str,
    css_content: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let re = Regex::new(r#"url\(\s*['"]?([^'"]+)['"]?\s*\)"#).unwrap();
    let mut updated_css = css_content.to_string();

    for cap in re.captures_iter(css_content) {
        if let Some(link) = cap.get(1) {
            let file_url = match Url::parse(link.as_str()) {
                Ok(url) => url.to_string(),
                Err(_) => base_url.join(link.as_str())?.to_string(),
            };

            let file_name = link.as_str().split('/').last().unwrap();
            let save_path = format!("{}/{}", dir_name, file_name);
            // let save_path1 = format!("./{}", file_name);

            // Télécharger les ressources CSS
            download_resource(client, &file_url, &save_path).await?;

            // Remplacer le lien dans le CSS par le chemin local
            updated_css = updated_css.replace(link.as_str(), &file_name);
        }
    }

    Ok(updated_css)
}



// Remplacer les liens de manière plus précise pour éviter la corruption de balises
fn replace_exact_link(content: &str, original: &str, replacement: &str) -> String {
    let cleaned_replacement = clean_path_segments(replacement);
    let re = Regex::new(&format!(r#"(?P<before>[="\(']){}(?P<after>[)"'])"#, regex::escape(original))).unwrap();
    let result = re.replace_all(content, |caps: &regex::Captures| {
        format!("{}{}{}", &caps["before"], cleaned_replacement, &caps["after"])
    });
    result.to_string()
}

// Nettoie les segments d'un chemin en supprimant les répétitions
fn clean_path_segments(path: &str) -> String {
    let segments: Vec<&str> = path.split('/').collect();
    let mut cleaned_segments = Vec::new();
    
    let mut i = 0;
    while i < segments.len() {
        if segments[i].is_empty() {
            i += 1;
            continue;
        }
        
        // Ignore le premier si deux segments successifs sont identiques
        if i + 1 < segments.len() && segments[i] == segments[i + 1] {
            i += 1;
        } else if i + 2 < segments.len() && segments[i] == segments[i + 2] {
            i += 2;
        } else {
            cleaned_segments.push(segments[i]);
        }
        
        i += 1;
    }

    cleaned_segments.join("/")
}

fn extract_parent_directory(path: &str) -> Option<&str> {
    let segments: Vec<&str> = path.split('/').collect();

    if segments.len() > 1 {
        return Some(segments[segments.len() - 2]);
    }

    None
}
