use futures::future::join_all;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tokio::task::spawn;

pub async fn process_input_file(input_file: String, cli_options: &crate::cli::CliOptions) -> Result<(), Box<dyn std::error::Error + Send>> {
    let file = File::open(&input_file)?;
    let reader = BufReader::new(file);

    let mut tasks = Vec::new();
    for line in reader.lines() {
        let url = line?;
        let cli_options_clone = cli_options.clone();

        let task = spawn(async move {
            // Télécharger chaque fichier ici de manière asynchrone
            crate::downloader::download_file_async(&url, &cli_options_clone).await
        });

        tasks.push(task);
    }

    let results = join_all(tasks).await;

    for result in results {
        if let Err(e) = result {
            eprintln!("Error downloading file: {:?}", e);
        }
    }

    Ok(())
}
