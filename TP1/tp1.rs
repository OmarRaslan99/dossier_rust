// Author: Omar MOSTAFA

// TP Evaluation 

// Créer un fichier main.rs 
// dans ce TP vous Créer un compte bancaire 
// avec ce Menu let options = ["Afficher solde","Retrait","Liste comptes","Quitter"];
// et les actions associés

use std::io;

fn main() {
    let options = ["Affichage du solde", "Retrait d'argent", "Liste des comptes", "Quitter"];
    println!("Menu:");
    for (i, option) in options.iter().enumerate() {
        println!("{}: {}", i + 1, option);
    }

    println!("Veuillez choisir une option...");

    let mut choix = String::new();
    io::stdin().read_line(&mut choix).expect("Erreur de lecture");
    let choix: usize = choix.trim().parse().expect("Veuillez entrer un nombre valide");

    // match choix {
    //     1 => println!("Affichage du solde..."),
    //     2 => println!("Retrait d'argent..."),
    //     3 => println!("Liste des comptes..."),
    //     4 => println!("Quitter le programme..."),
    //     _ => println!("Option invalide, veuillez réessayer."),
    // }

    // mainetnant au lieu d'afficher l'action comme texte, il faudra appliquer l'action correspondante
    // par exemple, si l'utilisateur choisit 1, afficher le solde du compte
    let solde = 1000.0; // Exemple de solde initial
    if choix == 1 {
        println!("Votre solde est de {} euros.", solde);
    } else if choix == 2 {
        println!("Veuillez renseigner le montant de retrait...");
        let mut montant_retrait = String::new();
        io::stdin().read_line(&mut montant_retrait).expect("Erreur de lecture");
        let montant_retrait: f64 = match montant_retrait.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un montant valide.");
                return;
            }
        };
        if montant_retrait <= solde {
            println!("Retrait de {} euros effectué.", montant_retrait);
        } else {
            println!("Fonds insuffisants pour le retrait de {} euros.", montant_retrait);
        }
    } else if choix == 3 {
        println!("Liste des comptes :");
        // Ici, on peut afficher une liste de comptes fictifs
        println!("1. Compte Courant - Solde: {} euros", solde);
        println!("2. Compte Épargne - Solde: {} euros", solde * 1.5);
    } else if choix == 4 {
        println!("Merci d'avoir utilisé le service. Au revoir !");
        return;
    } else {
        println!("Choix invalide. Veuillez choisir une option valide.");
    }
}
