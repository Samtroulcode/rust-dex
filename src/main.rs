use colored::*;
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
mod gen1;

type Pokedex = HashMap<String, HashSet<String>>;

fn add_pokemon(dex: &mut Pokedex, p_type: &str, nom: &str) {
    dex.entry(p_type.to_string())
        .or_default()
        .insert(nom.to_string());
}

fn print_pokedex(dex: &Pokedex) {
    let mut types: Vec<_> = dex.keys().collect();
    types.sort();

    for t in types {
        let colorized_type = match t.as_str() {
            "feu" => t.red(),
            "eau" => t.blue(),
            "plante" => t.green(),
            "électrique" => t.yellow(),
            "dragon" => t.purple(),
            "fée" => t.magenta(),
            "combat" => t.bright_cyan(),
            "glace" => t.bright_white(),
            "insecte" => t.bright_green(),
            "poison" => t.bright_red(),
            "roche" => t.bright_yellow(),
            "psy" => t.bright_blue(),
            "spectre" => t.bright_magenta(),
            "sol" => t.bright_black(),
            _ => t.white(),
        };

        println!("{}", colorized_type.bold());

        let mut pokemons: Vec<_> = dex[t].iter().collect();
        pokemons.sort();

        for p in pokemons {
            println!("  - {}", p);
        }

        println!();
    }
}

fn add_from_struct(dex: &mut Pokedex, p: &Pokemon) {
    add_pokemon(dex, &p.p_type, &p.nom);
}

fn add_many_pokemon_from_struct(dex: &mut Pokedex, pokemons: Vec<Pokemon>) {
    for p in pokemons {
        add_from_struct(dex, &p);
    }
}

fn init_pokedex() -> Pokedex {
    let mut dex: Pokedex = HashMap::new();

    let list_pokemon = gen1::gen1_pokedex();

    add_many_pokemon_from_struct(&mut dex, list_pokemon);

    dex
}

fn add_pokemon_from_input(dex: &mut Pokedex) {
    println!("Entrez le nom du Pokémon :");
    let nom = io::stdin()
        .lines()
        .next()
        .expect("Erreur de lecture")
        .expect("Erreur de lecture")
        .trim()
        .to_string();

    println!("Entrez le type du Pokémon :");
    let p_type = io::stdin()
        .lines()
        .next()
        .expect("Erreur de lecture")
        .expect("Erreur de lecture")
        .trim()
        .to_string();

    add_pokemon(dex, &p_type, &nom);
}

#[derive(Debug)]
struct Pokemon {
    nom: String,
    p_type: String,
}

impl Pokemon {
    fn new(nom: &str, p_type: &str) -> Self {
        Self {
            nom: nom.to_string(),
            p_type: p_type.to_string(),
        }
    }
}

fn main() {
    println!("initialisation du pokedex...");
    let mut dex = init_pokedex();

    let total_pokemon: usize = dex.values().map(|set| set.len()).sum();

    println!("Pokedex initialisé avec les {total_pokemon} Pokémon de la gen 1 !");

    println!("Voulez-vous ajouter un Pokémon ? y/n");

    let add_pokemon = io::stdin()
        .lines()
        .next()
        .expect("Erreur de lecture")
        .expect("Erreur de lecture")
        .trim()
        .to_lowercase();

    if add_pokemon == "y" {
        add_pokemon_from_input(&mut dex);
        println!("Pokémon ajouté avec succès !");
    } else {
        println!("Aucun Pokémon ajouté.");
    }

    println!("Afficher les pokemons ? y/n");

    let display_pokemon = io::stdin()
        .lines()
        .next()
        .expect("Erreur de lecture")
        .expect("Erreur de lecture")
        .trim()
        .to_lowercase();

    if display_pokemon == "y" {
        println!("Affichage du Pokédex :");
        print_pokedex(&dex);
    } else {
        println!("Affichage du Pokédex annulé.");
    }

    println!("\nAppuyez sur Entrée pour quitter...");
    io::stdout().flush().unwrap(); // s'assurer que le texte s'affiche avant de bloquer
    let _ = io::stdin().read_line(&mut String::new());
}
