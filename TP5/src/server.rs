// Fichier: src/server.rs
// Serveur DNS simple sur UDP, réponse pour quelques domaines prédéfinis

use std::net::UdpSocket;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    // 1. Crée et lie la socket UDP (port 5354 pour tests non-root)
    let socket = UdpSocket::bind("0.0.0.0:5354")?;
    println!("Serveur DNS démarré sur 0.0.0.0:5354");

    // 2. Table DNS statique
    let mut table: HashMap<&str, [u8; 4]> = HashMap::new();
    table.insert("example.com", [93, 184, 216, 34]);
    table.insert("localhost", [127, 0, 0, 1]);

    let mut buf = [0u8; 512];
    loop {
        // 3. Recevoir une requête
        let (len, src) = socket.recv_from(&mut buf)?;
        println!("Requête de {} ({} octets)", src, len);

        // 4. Parser l'entête DNS (transaction ID) et la question
        let tx_id = &buf[0..2];
        let _flags = &buf[2..4];
        let qdcount = u16::from_be_bytes([buf[4], buf[5]]);
        if qdcount != 1 {
            continue; // on gère uniquement une question
        }
        // Lecture du nom de domaine dans la question
        let mut pos = 12;
        let mut labels = Vec::new();
        loop {
            let len_label = buf[pos] as usize;
            if len_label == 0 { pos += 1; break; }
            pos += 1;
            let label = String::from_utf8_lossy(&buf[pos..pos + len_label]);
            labels.push(label.to_string());
            pos += len_label;
        }
        let domain = labels.join(".");
        println!("Question: {}", domain);

        // Type et class (on suppose type A, class IN)
        let _qtype = u16::from_be_bytes([buf[pos], buf[pos+1]]);
        let _qclass = u16::from_be_bytes([buf[pos+2], buf[pos+3]]);

        // 5. Construire la réponse dans un nouveau buffer
        let mut res = Vec::new();
        // Transaction ID
        res.extend_from_slice(tx_id);
        // Flags: réponse, récursif impossible, pas d'erreur
        res.extend_from_slice(&0x8180u16.to_be_bytes());
        // QDCOUNT = 1, ANCOUNT = 1 si trouvé sinon 0
        res.extend_from_slice(&1u16.to_be_bytes());
        let found = table.contains_key(domain.as_str());
        res.extend_from_slice(&(if found {1u16} else {0u16}).to_be_bytes());
        // NSCOUNT + ARCOUNT = 0
        res.extend_from_slice(&0u16.to_be_bytes());
        res.extend_from_slice(&0u16.to_be_bytes());
        // Copier la question originale
        res.extend_from_slice(&buf[12..pos+4]);

        // 6. Si domaine connu, ajouter la réponse
        if let Some(ip) = table.get(domain.as_str()) {
            // Pointer vers le nom (offset 0xC = 12)
            res.extend_from_slice(&0xC00Cu16.to_be_bytes());
            // Type A, classe IN
            res.extend_from_slice(&1u16.to_be_bytes());
            res.extend_from_slice(&1u16.to_be_bytes());
            // TTL (32 bits)
            res.extend_from_slice(&300u32.to_be_bytes());
            // Longueur du RDATA = 4
            res.push(4);
            // Adresse IP
            res.extend_from_slice(ip);
        }

        // 7. Envoyer la réponse
        socket.send_to(&res, src)?;
        println!("Réponse envoyée à {}", src);
    }
}