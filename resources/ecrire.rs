use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut file = File::create("test.txt")?;
    file.write_all(b"Bonjour a tous, fichier cree!\n")?;
    println!("Fichier créé avec succès !");
    
    // let compte = CompteBancaire {
    //     nom: String::from("Alice"),
    //     solde: 100.0,
    // };

    // compte.afficher();
    // compte.deposer(50.0);
    // compte.retirer(30.0);
    // compte.afficher();

    // // Écrire dans le fichier
    // writeln!(file, "Compte de {}: Solde actuel: {} €", compte.nom, compte.solde)?;
    
    Ok(());       
    Err(e)   
}
