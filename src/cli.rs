use clap::{Arg, Command};

// Structure pour les options CLI
pub struct CliOptions {
    pub url: String,
    pub output_file: Option<String>,
    pub output_dir: Option<String>,
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
        .get_matches();

    // Utilisation de `get_one::<T>()` pour récupérer les arguments
    let url = matches.get_one::<String>("url").unwrap().to_string();
    let output_file = matches.get_one::<String>("output_file").map(|o| o.to_string());
    let output_dir = matches.get_one::<String>("output_dir").map(|p| p.to_string());

    CliOptions {
        url,
        output_file,
        output_dir,
    }
}
