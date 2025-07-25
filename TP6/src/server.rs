// src/server.rs
// Serveur TCP simple implémentant un protocole personnalisé de type 'Echo'
// Format du message client -> Serveur :
// [u8 opcode][u16 length][payload]
// opcode 1 = Echo : renvoie exactement le payload au client

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    loop {
        // 1. Lire l'opcode (1 octet)
        let mut op_buf = [0u8; 1];
        if stream.read_exact(&mut op_buf).is_err() {
            println!("Client déconnecté");
            break;
        }
        let opcode = op_buf[0];

        // 2. Lire la longueur (2 octets big-endian)
        let mut len_buf = [0u8; 2];
        if stream.read_exact(&mut len_buf).is_err() {
            println!("Erreur lecture longueur");
            break;
        }
        let length = u16::from_be_bytes(len_buf) as usize;

        // 3. Lire le payload
        let mut payload = vec![0u8; length];
        if stream.read_exact(&mut payload).is_err() {
            println!("Erreur lecture payload");
            break;
        }

        // 4. Traiter selon opcode
        match opcode {
            1 => {
                // Echo
                let mut response = Vec::new();
                response.push(1u8); // même opcode
                response.extend_from_slice(&(length as u16).to_be_bytes());
                response.extend_from_slice(&payload);
                if stream.write_all(&response).is_err() {
                    println!("Erreur écriture réponse");
                    break;
                }
            }
            _ => {
                println!("Opcode inconnu: {}", opcode);
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9000")?;
    println!("Serveur démarré sur 127.0.0.1:9000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Nouvelle connexion: {}", stream.peer_addr()?);
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => eprintln!("Erreur connexion: {}", e),
        }
    }
    Ok(())
}