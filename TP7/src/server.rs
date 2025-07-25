// src/server.rs
// Serveur WebSocket basique en Rust avec tokio et tokio-tungstenite

use futures_util::{StreamExt, SinkExt};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    // 1. Démarrer un listener TCP sur le port 9001
    let addr = "127.0.0.1:9001";
    let listener = TcpListener::bind(addr).await?;
    println!("Serveur WebSocket démarré sur ws://{}", addr);

    // 2. Accepter les connexions entrantes
    while let Ok((stream, peer)) = listener.accept().await {
        println!("Nouvelle connexion de {}", peer);

        // 3. Accepter le handshake WebSocket
        tokio::spawn(async move {
            let ws_stream = accept_async(stream)
                .await
                .expect("Erreur handshake WebSocket");
            println!("WebSocket handshake réussi avec {}", peer);

            let (mut write, mut read) = ws_stream.split();

            // 4. Boucle de réception et renvoi (echo)
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(msg) => {
                        if msg.is_text() || msg.is_binary() {
                            let received = msg.clone();
                            // Afficher au serveur
                            println!("Reçu de {}: {}", peer, msg.to_text().unwrap_or("[binary]"));
                            // Renvoi au client
                            if write.send(received).await.is_err() {
                                println!("Erreur envoi vers {}", peer);
                                break;
                            }
                        } else if msg.is_close() {
                            println!("Client {} ferme la connexion", peer);
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Erreur lecture message: {}", e);
                        break;
                    }
                }
            }
            println!("Connexion {} terminée", peer);
        });
    }

    Ok(())
}
