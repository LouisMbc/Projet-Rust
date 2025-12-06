use std::io;

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

fn main() {
    let mut _biblio: Vec<Livre> = Vec::new();
    println!("livre");
}
