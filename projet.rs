use std::io::{self, Write};

//structure livre
#[derive(Clone)]
struct Livre {
    titre: String,
    auteur: String,
    annee: u32,
    disponible: bool,
}

fn lire_ligne() -> String {
    let mut entree = String::new();
    io::stdin().read_line(&mut entree).unwrap();
    entree.trim().to_string()
}

//Ajouter livres
fn ajouter_livre(biblio: &mut Vec<Livre>) {
    println!("Titre du livre :");
    let titre = lire_ligne();

    for livre in biblio.iter() {
        if livre.titre == titre {
            println!("Un livre avec ce titre existe déjà");
            return;
        }
    }

    println!("Auteur :");
    let auteur = lire_ligne();

    println!("Année :");
    let annee_str = lire_ligne();
    let annee = annee_str.parse::<u32>().unwrap_or(0);

    let nouveau = Livre {
        titre,
        auteur,
        annee,
        disponible: true,
    };

    biblio.push(nouveau);
    println!("Livre ajouté !");
}

// emprunter livre
fn emprunter_livre(biblio: &mut Vec<Livre>) {
    println!("Titre du livre à emprunter :");
    let titre = lire_ligne();

    for livre in biblio.iter_mut() {
        if livre.titre == titre {
            if livre.disponible {
                livre.disponible = false;
                println!("Livre emprunté");
            } else {
                println!("Ce livre est déjà emprunté");
            }
            return;
        }
    }

    println!("Livre introuvable");
}

//retour du livre
fn retourner_livre(biblio: &mut Vec<Livre>) {
    println!("Titre du livre à retourner :");
    let titre = lire_ligne();

    for livre in biblio.iter_mut() {
        if livre.titre == titre {
            if !livre.disponible {
                livre.disponible = true;
                println!("Livre retourné");
            } else {
                println!("Ce livre n'était pas emprunté");
            }
            return;
        }
    }
    println!("Livre introuvable");
}

//affichage des livres
fn afficher_livres(biblio: &Vec<Livre>) {
    println!("Liste des livres :");
    for livre in biblio {
        let statut = if livre.disponible { "Disponible" } else { "Emprunté" };
        println!("- {} | {} | {} | {}",livre.titre, livre.auteur, livre.annee, statut
        );
    }
}

// affichage des livres disponibles
fn afficher_livres_disponibles(biblio: &Vec<Livre>) {
    println!("Livres disponibles :");
    for livre in biblio {
        if livre.disponible {
            println!("- {} | {} | {}",livre.titre, livre.auteur, livre.annee);
        }
    }
}

fn main() {
    let mut biblio: Vec<Livre> = Vec::new();

    println!("\n---- Bibliothèque ----");
    println!("1. Ajouter un livre");
    println!("2. Emprunter un livre");
    println!("3. Retourner un livre");
    println!("4. Afficher tous les livres");
    println!("5. Afficher les livres disponibles");
    println!("6. Quitter");

    loop {
        print!("\nVotre choix (1-6): ");
        io::stdout().flush().unwrap();//pour éviter de faire entré après avvoir terminé une action
        let choix = lire_ligne();

        match choix.as_str() {
            "1" => {println!("votre choix est 1"); 
            ajouter_livre(&mut biblio);},
            "2" => {println!("votre choix est 2"); 
            emprunter_livre(&mut biblio);},
            "3" => {println!("votre choix est 3"); 
            retourner_livre(&mut biblio);},
            "4" => {println!("votre choix est 4"); 
            afficher_livres(&biblio);},
            "5" => {println!("votre choix est 5"); 
            afficher_livres_disponibles(&biblio);},
            "6" => {break;}
            _ => println!("Choix invalide"),
        }
    }
}
