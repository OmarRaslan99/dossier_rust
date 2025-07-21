// tp2
// Author: Omar MOSTAFA
// Application de la structure avec les notions de `&self`, `mut self`, `self` et `mut`


struct CompteBancaire {
    nom :String,
    solde: f64,
}

impl CompteBancaire{

    fn afficher(&self){
        println!("Compte de {}: Solde actuel: {} €", self.nom, self.solde);
        if self.solde < 0.0 {
            println!("Attention, le solde est négatif !"); 
        }
    }

    fn deposer(&mut self, montant:f64){
        // Bonus : empêcher le dépôt d'un montant négatif
        if montant < 0.0 {
            println!("Le montant doit être positif.");
            return;
        }else if montant == 0.0 {
            println!("Aucun montant déposé.");
            return;
        } else {
            self.solde += montant;
        }   
        println!("+{} € déposés:",montant);
    }

    fn retirer( &mut self, montant:f64){

        if self.solde >= montant{
            self.solde -=montant;
            println!("-{} € retirés.",montant)
        } else{
            println!("Solde insuffisant")
        }
    }

    fn fermer(self){
        println!("le compte de {} est fermé, dernier solde : {}€ ", self.nom, self.solde);
    }

    // Bonus : ajouter une méthode  renommer  qui renvoi un nouveau compte avec le nom changé
    fn renommer(self, nouveau_nom: String) -> CompteBancaire {
        CompteBancaire {
            nom: nouveau_nom,
            solde: self.solde,
        }
    }

    // self ici est consomé ici , on ne peut plus utiliser l'objet ensuite 

}

fn main() {
    let mut compte = CompteBancaire {
        nom: String::from("Alice"),
        solde: 100.0,
    };

    compte.afficher();
    compte.deposer(-50.0);
    compte.retirer(30.0);
    compte.afficher();
    compte = compte.renommer(String::from("Bob"));
    compte.afficher();
    compte.fermer();

     // Bonus : créer un Vec<CompteBancaire> pour gérer plusieurs comptes ( en utilisant .iter(), .enumerate() )
    let mut comptes: Vec<CompteBancaire> = Vec::new();
    comptes.push(CompteBancaire {
        nom: String::from("Alice"),
        solde: 100.0,
    });
    comptes.push(CompteBancaire {
        nom: String::from("Bob"),
        solde: 200.0,
    });
    comptes.push(CompteBancaire {
        nom: String::from("Charlie"),
        solde: 300.0,
    });
    for (index, compte) in comptes.iter().enumerate() {
        println!("Compte {}: {} et le montant de Solde est {} €", index + 1, compte.nom, compte.solde);
    }
}   
