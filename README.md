# Synthèse du cours 1 : 21/07/2025 de 9h45 à 19h00
# Notions abordées :

---

## 1. Installation de Rust  
Rust s’installe facilement via l’outil `rustup`, qui gère le compilateur `rustc` et le gestionnaire de paquets `cargo`.  

## 2. Syntaxe et conventions  
- **Typage** : Rust utilise des types précis (ex. `u32`, `i32`, `f32`) pour garantir la sécurité mémoire.  
- **Convention snake_case** : Variables et fonctions utilisent le format `snake_case`.  

## 3. Fonctions  
- **Déclaration** : `fn nom_fonction(param: Type) -> TypeRetour { … }`.  
- **Retour implicite** : La dernière expression sans point-virgule est retournée.  

## 4. Conditions  
- **`if` / `else`** : Dirige le flux selon une expression booléenne.  

## 5. Boucles  
- **`for`** : Itère sur une plage (`1..5`) ou une collection (`for x in coll`).  
- **`loop`** : Crée une boucle infinie, quittée avec `break`.  
- **`while`** : Répète tant que la condition est vraie.  

## 6. Tableaux  
- **Déclaration** : `let tab: [T; N] = [v1, v2, …]`.  
- **Itération** : `for x in tab.iter()` ou `for (i, x) in tab.iter().enumerate()`.  

## 7. Vecteurs  
- **Déclaration dynamique** : `let v = vec![v1, v2, …];`.  
- **Itération** : Identique aux tableaux via `.iter().enumerate()`.  

## 8. Mutabilité  
- **`mut`** : Rend une variable modifiable (`let mut x`).  
- **`&mut`** : Passe une référence mutable pour modifier un contenu sans déplacer la valeur.  

## 9. Lecture d’entrée utilisateur  
- **`std::io`** : `io::stdin().read_line(&mut s)` lit une ligne depuis l’utilisateur, à parser ensuite.  

---

## 10. `match`  
Le `match` permet le pattern‑matching sur des valeurs ou des enums, remplaçant souvent plusieurs `if/else` :  
```rust
match valeur {
    0 => println!("zéro"),
    1..=5 => println!("entre 1 et 5"),
    _ => println!("autre"),
}
```

---

## `struct`
Les `struct` définissent vos propres types en regroupant des champs nommés :
```rs
struct Personne {
    nom: String,
    age: u32,
}
```

## 12. `impl` et méthodes
On utilise `impl` pour associer des fonctions (méthodes) à une struct :
```rs
impl Personne {
    fn afficher(&self) {
        println!("Je suis {} et j'ai {} ans", self.nom, self.age);
    }
}
```

## 13. Notion de self
- `&self` : Méthode en lecture seule, emprunte une référence immuable.
- `&mut self` : Méthode en écriture, emprunte une référence mutable.
- `self` : Consomme l’objet, qui n’est plus utilisable après l’appel.

---

## 14. Travaux Pratiques 1
Création d’un menu interactif de gestion de comptes bancaires :
1. Afficher le solde
2. Retirer de l’argent
3. Afficher la liste des comptes
4. Quitter
Chaque option mobilise boucles, conditions, typage, mutabilité et lecture d’entrée.

---

## 15. Travaux Pratiques 2
Application des structs et méthodes avec un CompteBancaire :
- `struct CompteBancaire { nom: String, solde: f64 }`
- Méthodes via `impl` :
    - `afficher(&self)` — affiche solde et avertit si négatif
    - `deposer(&mut self, montant: f64)` — ajoute un montant positif
    - `retirer(&mut self, montant: f64)` — soustrait si fonds suffisants
    - `fermer(self)` — consomme le compte et affiche le dernier solde
    - `renommer(self, nouveau_nom: String) -> CompteBancaire` — retourne un nouveau compte renommé

Bonus : gestion de plusieurs comptes dans un `Vec<CompteBancaire>` et itération `.iter().enumerate()`.