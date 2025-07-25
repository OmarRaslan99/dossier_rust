// src/client.rs
// Client WebSocket basique en Rust avec tokio and tokio-tungstenite

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use url::Url;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    // 1. URL du serveur
    let url = Url::parse("ws://127.0.0.1:9001").unwrap();
    println!("Connexion à {}", url);

    // 2. Connexion WebSocket
    let (mut ws_stream, _) = connect_async(url)
        .await
        .expect("Échec de la connexion WebSocket");
    println!("Connecté au serveur WebSocket");

    // 3. Envoyer un message texte
    let texte = "Bonjour depuis le client WebSocket!";
    ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(texte.into()))
        .await
        .expect("Erreur envoi message");
    println!("Message envoyé: {}", texte);

    // 4. Attendre la réponse (echo)
    if let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(msg) => {
                if msg.is_text() {
                    println!("Réponse reçue: {}", msg.to_text().unwrap());
                }
            }
            Err(e) => println!("Erreur réception message: {}", e),
        }
    }

    // 5. Fermer proprement
    ws_stream.close(None).await.expect("Erreur fermeture WebSocket");
    println!("Client fermé");

    Ok(())
}