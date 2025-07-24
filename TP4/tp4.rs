// Author : Omar MOSTAFA

use tokio::{net::{TcpListener, TcpStream}, sync::Mutex};
use tokio::io::{AsyncBufReadExt, BufReader};
use std::sync::Arc;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::error::Error;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. Créer le dossier logs si nécessaire
    fs::create_dir_all("logs")?;

    // 2. Démarrer le listener TCP
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Serveur démarré sur 127.0.0.1:8080");

    // 3. Préparer le fichier de log protégé par un Mutex
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/server.log")?;
    let file = Arc::new(Mutex::new(file));

    loop {
        // 4. Accepter une connexion entrante
        let (socket, addr) = listener.accept().await?;
        println!("Nouvelle connexion depuis {}", addr);

        // 5. Cloner l'Arc pour le move dans la tâche
        let file = Arc::clone(&file);

        // 6. Lancer une tâche pour gérer ce client
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, file).await {
                eprintln!("Erreur client {}: {}", addr, e);
            }
        });
    }
}

/// Gère un client TCP: lit ligne par ligne et enregistre dans le fichier de log
async fn handle_client(
    stream: TcpStream,
    file: Arc<Mutex<std::fs::File>>
) -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        let timestamp = Utc::now().to_rfc3339();
        let entry = format!("[{}] {}\n", timestamp, line);

        // Écriture protégée
        let mut f = file.lock().await;
        f.write_all(entry.as_bytes())?;
    }

    Ok(())
}
