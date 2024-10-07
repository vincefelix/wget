mod cli;
mod downloader;
mod utils;

fn main() {
    let cli_options = cli::parse_cli();  // Parse les options CLI

    if let Err(e) = downloader::download_file(&cli_options.url, cli_options.output_file, cli_options.output_dir) {
        eprintln!("Failed to download the file: {}", e);
    }
}
