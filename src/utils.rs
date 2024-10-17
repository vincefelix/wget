use std::fs::OpenOptions;
use std::io::Write;
// use std::time::SystemTime;
// use chrono::{Local, DateTime};

// Fonction pour enregistrer les logs dans un fichier
pub fn log_to_file(log_message: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("wget-log")
        .expect("Cannot open log file");

    // let now: DateTime<Local> = SystemTime::now().into();
    // let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let log_entry = format!("{}\n", log_message);

    file.write_all(log_entry.as_bytes()).expect("Unable to write log");
}
