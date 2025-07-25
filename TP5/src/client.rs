// Fichier: src/client.rs
// Client DNS simple qui interroge le serveur sur UDP

use std::net::UdpSocket;
use std::time::Duration;

fn build_query(domain: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    // 1. Transaction ID arbitraire
    buf.extend_from_slice(&0x1234u16.to_be_bytes());
    // Flags: requête standard
    buf.extend_from_slice(&0x0100u16.to_be_bytes());
    // QDCOUNT=1, AN=0, NS=0, AR=0
    buf.extend_from_slice(&1u16.to_be_bytes());
    buf.extend_from_slice(&0u16.to_be_bytes());
    buf.extend_from_slice(&0u16.to_be_bytes());
    buf.extend_from_slice(&0u16.to_be_bytes());
    // 2. Encodage du nom de domaine
    for label in domain.split('.') {
        buf.push(label.len() as u8);
        buf.extend_from_slice(label.as_bytes());
    }
    buf.push(0);
    // Type A, Classe IN
    buf.extend_from_slice(&1u16.to_be_bytes());
    buf.extend_from_slice(&1u16.to_be_bytes());
    buf
}

fn main() -> std::io::Result<()> {
    let server = "127.0.0.1:5354";
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_read_timeout(Some(Duration::from_secs(2)))?;

    let domain = "example.com";
    let req = build_query(domain);
    socket.send_to(&req, server)?;
    println!("Requête envoyée pour {}", domain);

    let mut buf = [0u8; 512];
    let (len, _) = socket.recv_from(&mut buf)?;
    println!("Réponse reçue ({} octets)", len);

    // On saute l'entête (12 octets) + question
    let mut pos = 12;
    while buf[pos] != 0 { pos += buf[pos] as usize + 1; }
    pos += 5; // 

    // Lecture de la réponse
    if buf[3] & 0x0F != 0 {
        println!("Code d'erreur DNS: {}", buf[3] & 0x0F);
        return Ok(());
    }
    let ancount = u16::from_be_bytes([buf[6], buf[7]]);
    if ancount == 0 {
        println!("Aucune réponse pour {}", domain);
        return Ok(());
    }
    // Champ NAME en pointeur
    pos += 2;
    let ip = &buf[pos+8..pos+12];
    println!("Adresse IP: {}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]);
    Ok(())
}
