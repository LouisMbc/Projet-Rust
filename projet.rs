use std::io;

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

fn main() {
    let mut biblio: Vec<Livre> = Vec::new();
    
    println!("ajout et  emprunt");
    ajouter_livre(&mut biblio);
    emprunter_livre(&mut biblio);
}
