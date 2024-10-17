mod download;
mod mirror;
mod utils;

use chrono::Local;
use clap::{Arg, Command};
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let matches = Command::new("Rust-Wget")
        .version("1.0")
        .about("Recreate wget functionality in Rust")
        .arg(
            Arg::new("url")
                .help("URL to download")
                .required_unless_present_any(&["input", "mirror"]), // Téléchargement d'un fichier requis sauf si 'input' ou 'mirror' est présent
        )
        .arg(
            Arg::new("output")
                .short('O')
                .help("Save the file with a specific name"),
        )
        .arg(
            Arg::new("directory")
                .short('P')
                .help("Save the file in a specific directory"),
        )
        .arg(
            Arg::new("rate")
                .long("rate-limit")
                .help("Limit download speed"),
        )
        .arg(
            Arg::new("background")
                .short('B')
                .action(clap::ArgAction::SetTrue)
                .help("Download in the background"),
        )
        .arg(
            Arg::new("input")
                .short('i')
                .help("Download multiple files from a list"),
        )
        .arg(
            Arg::new("mirror")
                .long("mirror")
                .help("Mirror an entire website")
                .action(clap::ArgAction::SetTrue), // Mirror is a boolean flag
        )
        .arg(
            Arg::new("reject")
                .short('R')
                .long("reject")
                .value_name("TYPES") // Ensure this argument can accept a value
                .help("Reject specific file types (used with --mirror)"),
        )
        .arg(
            Arg::new("exclude")
                .short('X')
                .long("exclude")
                .value_name("DIRS") // Ensure this argument can accept a value
                .help("Exclude specific directories (used with --mirror)"),
        )
        .arg(
            Arg::new("convert_links")
                .long("convert-links")
                .action(clap::ArgAction::SetTrue)
                .help("Convert links for offline viewing (used with --mirror)"),
        )
        .arg(
            Arg::new("already_in_background")
                .long("already_in_background")
                .hide(true)
                .action(clap::ArgAction::SetTrue)
                .help("Indicates the program is already running in background"),
        )
        .get_matches();

    // Téléchargement de fichiers multiples
    if let Some(file_path) = matches.get_one::<String>("input") {
        if let Err(e) = download::download_multiple_files(file_path).await {
            eprintln!("Error occurred during multiple file download: {}", e);
        }
    // Mirroring d'un site complet avec gestion des flags --mirror, --reject, --exclude, --convert-links
    } else if matches.get_flag("mirror") {
        let mirror_url = matches.get_one::<String>("url").expect("URL is required for mirroring");
        let reject_types = matches.get_one::<String>("reject");
        let exclude_dirs = matches.get_one::<String>("exclude");
        let convert_links = matches.get_flag("convert_links");

        // Appel à la fonction `mirror_website` pour effectuer le mirroring
        println!("Mirroring website: {}", mirror_url);

        if let Err(e) = mirror::mirror_website(
            mirror_url,
            reject_types.map(|s| &**s),   // Handle file types to reject
            exclude_dirs.map(|s| &**s),   // Handle directories to exclude
            convert_links,                // Handle link conversion for offline viewing
        ).await {
            eprintln!("Error occurred during website mirroring: {}", e);
        }

    // Téléchargement d'un seul fichier
    } else if let Some(url) = matches.get_one::<String>("url") {
        let file_name = matches.get_one::<String>("output").map(|s| s.to_string());
        let directory = matches.get_one::<String>("directory").map(|s| s.to_string());
        let rate_limit = matches.get_one::<String>("rate").map(|s| s.to_string());
        let background = matches.get_flag("background");
        let already_in_background = matches.get_flag("already_in_background");

        if background && !already_in_background {
            println!("Output will be written to 'wget-log'.");

            // Commande pour exécuter en arrière-plan
            let mut command = std::process::Command::new(std::env::current_exe().unwrap());
            command.arg("--already_in_background").arg(url);

            if let Some(ref name) = file_name {
                command.arg("-O").arg(name);
            }
            if let Some(ref dir) = directory {
                command.arg("-P").arg(dir);
            }
            if let Some(ref rate) = rate_limit {
                command.arg("--rate-limit").arg(rate);
            }

            // Redirection des logs vers `wget-log` explicitement
            let log_file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("wget-log")
                .expect("Failed to open log file for background process");

            command.stdout(log_file.try_clone().unwrap()); // Redirection stdout vers fichier
            command.stderr(log_file); // Redirection stderr vers fichier

            command.spawn().expect("Failed to launch background process");

            return; // Libérer immédiatement le terminal
        }

        // Télécharger en mode normal ou si déjà en arrière-plan
        if let Err(e) = download::download_single_file(
            url,
            file_name.as_deref(),
            directory.as_deref(),
            rate_limit.as_deref(),
            background,
        )
        .await
        {
            eprintln!("Error occurred during single file download: {}", e);
        }
    }

    let end_time = Local::now();
    println!("Finished at: {}", end_time.format("%Y-%m-%d %H:%M:%S"));
}
