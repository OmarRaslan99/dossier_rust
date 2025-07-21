# Synthèse du cours 1 : 21/07/2025 de 9h45 à 19h00
# Notions abordées :

---

## 1. Installation de Rust  
Rust s’installe facilement via l’outil `rustup`, qui gère le compilateur `rustc` et le gestionnaire de paquets `cargo`.

## 2. Syntaxe et conventions  
- **Typage** : Rust utilise des types précis (ex. `u32`, `i32`, `f32`) pour garantir la sécurité mémoire.  
- **Convention snake_case** : Tous les noms de variables et fonctions suivent la convention `snake_case`.  

## 3. Fonctions  
- **Déclaration** : `fn nom_fonction(param: Type) -> TypeRetour { … }` permet de définir des fonctions claires et typées.  
- **Retour implicite** : La dernière expression sans point-virgule est retournée.

## 4. Conditions  
- **`if` / `else`** : Permet de diriger le flux d’exécution selon des expressions booléennes.

## 5. Boucles  
- **`for`** : Itère sur des gammes (`1..=5`) ou des collections, avec `..` exclusif et `..=` inclusif.  

## 6. Tableaux  
- **Déclaration** : `let tab = [elem1, elem2, elem3];` définit un tableau de taille fixe.  
- **Itération** : `for x in tab { … }` ou `for (i, x) in tab.iter().enumerate() { … }` pour récupérer l’index.

## 7. Vecteurs  
- **Déclaration dynamique** : `let v = vec![val1, val2];` crée un conteneur extensible.  
- **Itération** : Même mécanisme qu’avec les tableaux, via `.iter().enumerate()`.

## 8. Mutabilité  
- **`mut`** : Rend une variable modifiable (`let mut x`).  
- **`&mut`** : Passe une référence mutable à une fonction ou structure pour en modifier le contenu.

## 9. Lecture d’entrée utilisateur  
- **`std::io`** : Utilise `io::stdin().read_line(&mut s)` pour récupérer et parser des chaînes depuis l’utilisateur.  

---

## 10. Travaux Pratiques 1
Le TP1 consiste à créer un mini‑programme de gestion de compte bancaire qui affiche un menu, lit le choix de l’utilisateur et exécute les actions :  
1. **Afficher le solde**  
2. **Retirer de l’argent**  
3. **Afficher la liste des comptes**  
4. **Quitter**  

Chaque option fait appel aux notions vues : boucles, conditions, typage, mutabilité et lecture d’entrée.

---
