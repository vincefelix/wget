use clap::{Arg, Command};

// Structure pour les options CLI
pub struct CliOptions {
    pub url: String,
    pub output_file: Option<String>,
    pub output_dir: Option<String>,
    pub rate_limit: Option<String>,
    pub background: bool,
}

// Fonction pour parser les arguments CLI
pub fn parse_cli() -> CliOptions {
    let matches = Command::new("wget-clone")
        .about("A simple wget-like file downloader")
        .arg(
            Arg::new("url")
                .help("The URL of the file to download")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output_file")
                .short('O')
                .long("output")
                .help("Save the file with a specific name")
                .value_name("FILENAME"),
        )
        .arg(
            Arg::new("output_dir")
                .short('P')
                .long("directory-prefix")
                .help("Save the file in a specific directory")
                .value_name("DIRECTORY"),
        )
        .arg(
            Arg::new("rate-limit")
                .long("rate-limit")
                .help("Limit the download speed, e.g., 400k or 2M")
                .value_name("RATE"),
        )
        .arg(
            Arg::new("background")
            .short('B')
            .long("background")
            .help("Run the download in the background")
            .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // Utilisation de `get_one::<T>()` pour récupérer les arguments
    let url = matches.get_one::<String>("url").unwrap().to_string();
    let output_file = matches.get_one::<String>("output_file").map(|o| o.to_string());
    let output_dir = matches.get_one::<String>("output_dir").map(|p| p.to_string());
    let rate_limit = matches.get_one::<String>("rate-limit").map(|r| r.to_string());
    let background = matches.get_flag("background");
    CliOptions {
        url,
        output_file,
        output_dir,
        rate_limit,
        background,
    }
}
