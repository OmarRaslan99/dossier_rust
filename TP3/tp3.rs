// Author: Omar MOSTAFA

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

/// Représente un fichier et ses métadonnées simplifiées
struct Fichier {
    nom: String,
    taille: u64,
}

impl Fichier {
    /// Initialise la structure à partir du nom de fichier donné (crée si nécessaire)
    fn new(nom: String) -> io::Result<Self> {
        // Crée le fichier s'il n'existe pas encore
        if !Path::new(&nom).exists() {
            File::create(&nom)?;
        }
        let metadata = fs::metadata(&nom)?;
        Ok(Fichier {
            nom: nom.clone(),
            taille: metadata.len(),
        })
    }

    /// Lit et affiche le contenu du fichier
    fn lire(&self) -> io::Result<()> {
        let mut file = File::open(&self.nom)?;
        let mut contenu = String::new();
        file.read_to_string(&mut contenu)?;
        println!("-- Contenu de '{}' --\n{}", self.nom, contenu);
        Ok(())
    }

    /// Écrit du contenu dans le fichier (écrase le contenu existant)
    fn ecrire(&mut self, texte: &str) -> io::Result<()> {
        let mut file = File::create(&self.nom)?;
        file.write_all(texte.as_bytes())?;
        self.taille = fs::metadata(&self.nom)?.len();
        println!("Écriture terminée ({} octets).", self.taille);
        Ok(())
    }

    /// Modifie le fichier en ajoutant du contenu à la fin
    fn modifier(&mut self, texte: &str) -> io::Result<()> {
        let mut file = File::options().append(true).open(&self.nom)?;
        file.write_all(texte.as_bytes())?;
        self.taille = fs::metadata(&self.nom)?.len();
        println!("Ajout terminé (taille mise à jour : {} octets).", self.taille);
        Ok(())
    }

    /// Supprime le fichier
    fn supprimer(self) -> io::Result<()> {
        fs::remove_file(&self.nom)?;
        println!("Fichier '{}' supprimé.", self.nom);
        Ok(())
    }
}

fn main() -> io::Result<()> {
    loop {
        println!("\nMenu:");
        println!("1. Lire un fichier");
        println!("2. Créer un fichier");
        println!("3. Écrire dans un fichier");
        println!("4. Modifier un fichier (ajout)");
        println!("5. Supprimer un fichier");
        println!("6. Quitter");
        print!("Votre choix : ");
        io::Write::flush(&mut io::stdout())?;

        let mut choix = String::new();
        io::stdin().read_line(&mut choix)?;
        let choix: usize = match choix.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez saisir un numéro valide !");
                continue;
            }
        };

        if choix == 6 {
            println!("Au revoir !");
            break;
        }

        if choix < 1 || choix > 5 {
            println!("Choix hors système !");
            continue;
        }

        // Demande du nom de fichier
        print!("Nom du fichier : ");
        io::Write::flush(&mut io::stdout())?;
        let mut nom = String::new();
        io::stdin().read_line(&mut nom)?;
        let nom = nom.trim().to_string();

        match choix {
            1 => {
                // Lire : vérifie l'existence
                if !Path::new(&nom).exists() {
                    println!("Erreur : le fichier '{}' n'existe pas !", nom);
                    continue;
                }
                let fichier = Fichier::new(nom)?;
                fichier.lire()?;
            }
            2 => {
                // Créer : si existe déjà, message, sinon création
                if Path::new(&nom).exists() {
                    println!("Le fichier '{}' existe déjà.", nom);
                    continue;
                }
                let fichier = Fichier::new(nom)?;
                println!("Fichier '{}' créé ({} octets).", fichier.nom, fichier.taille);
            }
            3 => {
                // Écrire remplace
                let mut fichier = Fichier::new(nom)?;
                println!("Entrez le texte à écrire (remplace tout) :");
                let mut texte = String::new();
                io::stdin().read_line(&mut texte)?;
                fichier.ecrire(texte.trim())?;
            }
            4 => {
                // Ajouter : vérifie existence
                if !Path::new(&nom).exists() {
                    println!("Erreur : le fichier '{}' n'existe pas !", nom);
                    continue;
                }
                let mut fichier = Fichier::new(nom)?;
                println!("Entrez le texte à ajouter :");
                let mut texte = String::new();
                io::stdin().read_line(&mut texte)?;
                fichier.modifier(texte.trim())?;
            }
            5 => {
                // Supprimer : vérifie existence
                if !Path::new(&nom).exists() {
                    println!("Erreur : le fichier '{}' n'existe pas !", nom);
                    continue;
                }
                let fichier = Fichier::new(nom)?;
                fichier.supprimer()?;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}
