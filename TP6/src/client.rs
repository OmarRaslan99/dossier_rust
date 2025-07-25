// src/client.rs
// Client TCP simple utilisant le protocole personnalisé 'Echo'

use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:9000")?;
    println!("Connecté au serveur 127.0.0.1:9000");

    // Exemple de message à envoyer
    let message = "Bonjour, protocole personnalisé!".as_bytes();
    let length = message.len() as u16;

    // Construire le paquet: opcode=1, length, payload
    let mut packet = Vec::new();
    packet.push(1u8); // Echo
    packet.extend_from_slice(&length.to_be_bytes());
    packet.extend_from_slice(message);

    // Envoyer
    stream.write_all(&packet)?;
    println!("Message envoyé: {}", String::from_utf8_lossy(message));

    // Recevoir la réponse
    let mut op_buf = [0u8; 1];
    stream.read_exact(&mut op_buf)?;
    let opcode = op_buf[0];

    let mut len_buf = [0u8; 2];
    stream.read_exact(&mut len_buf)?;
    let resp_len = u16::from_be_bytes(len_buf) as usize;

    let mut resp_payload = vec![0u8; resp_len];
    stream.read_exact(&mut resp_payload)?;
    println!("Réponse reçue (opcode={}): {}", opcode, String::from_utf8_lossy(&resp_payload));

    Ok(())
}