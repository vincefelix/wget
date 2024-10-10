
// Fonction pour formater la taille du fichier en KB, MB ou GB
pub fn format_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    if bytes as f64 >= GB {
        format!("{:.2} GB", bytes as f64 / GB)
    } else if bytes as f64 >= MB {
        format!("{:.2} MB", bytes as f64 / MB)
    } else if bytes as f64 >= KB {
        format!("{:.2} KB", bytes as f64 / KB)
    } else {
        format!("{} bytes", bytes)
    }
}

// Fonction pour extraire le nom du fichier à partir de l'URL
pub fn get_file_name(url: &str) -> &str {
    url.split('/').last().unwrap_or("downloaded_file")
}

// Fonction pour convertir le rate-limit en octets/s
pub fn parse_rate_limit(rate_limit: &str) -> Result<u64, String> {
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
