use colored::*;
use std::collections::{HashMap, HashSet};
use std::io::{self};

pub mod gen1;
pub mod pokemon;

pub use crate::pokemon::Pokemon;

type Pokedex = HashMap<String, HashSet<String>>;

fn add_pokemon(dex: &mut Pokedex, p_type: &str, nom: &str, p_id: &usize) {
    dex.entry(p_type.to_string())
        .or_default()
        .insert(format!("{} (ID: {})", nom, p_id));
}

fn print_pokedex_by_type(dex: &Pokedex) {
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
    add_pokemon(dex, &p.p_type, &p.nom, &p.p_id);
}

fn add_many_pokemon_from_struct(dex: &mut Pokedex, pokemons: Vec<Pokemon>) {
    for p in pokemons {
        add_from_struct(dex, &p);
    }
}

fn init_pokedex() -> Pokedex {
    println!("initialisation du pokedex...");
    let mut dex: Pokedex = HashMap::new();

    let list_pokemon = gen1::gen1_pokedex();

    add_many_pokemon_from_struct(&mut dex, list_pokemon);

    let total_pokemon: usize = dex.values().map(|set| set.len()).sum();
    println!("Pokedex initialisé avec {total_pokemon} Pokémon de la gen 1 !");
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
        .to_lowercase()
        .to_string();

    let p_id: usize = dex.values().map(|set| set.len()).sum::<usize>() + 1;
    add_pokemon(dex, &p_type, &nom, &p_id);
}

fn display_welcome_message() {
    println!(
        "{}",
        "Bienvenue dans le Pokedex !"
            .bold()
            .underline()
            .bright_green()
    );
    println!("Ce programme vous permet de gérer un Pokedex de la première génération.");
    println!(
        "Vous pouvez afficher les Pokémon par ID ou par type, et ajouter de nouveaux Pokémon."
    );
}

fn main() {
    display_welcome_message();
    let mut dex = init_pokedex();

    loop {
        println!(
            "\n1 : Afficher le pokedex  2: Ajouter un pokemon  3: Réinitialisé le pokedex  4: Quitter le programme\n\nVotre choix :"
        );

        let usr_choice = io::stdin()
            .lines()
            .next()
            .expect("Erreur de lecture")
            .expect("Erreur de lecture");

        match usr_choice.trim().parse::<u8>() {
            Ok(1) => {
                // println!("1: Par id  2: Par type  3: Quitter");
                // let choice = io::stdin()
                // .lines()
                // .next()
                // .expect("Erreur de lecture")
                // .expect("Erreur de lecture");
                // match choice.trim().parse::<u8>() {
                // Ok(1) => {
                // println!("\nAffichage du pokedex par ID :");
                // print_pokedex_by_id(&dex);
                // }
                // Ok(2) => {
                println!("\nAffichage du pokedex par type :");
                print_pokedex_by_type(&dex);
                // }
                // Ok(3) => {
                // println!("Retour au menu...");
                // continue;
                // }
                // _ => println!("\nChoix invalide, veuillez réessayer."),
                // }
            }
            Ok(2) => {
                add_pokemon_from_input(&mut dex);
                println!("\nPokémon ajouté avec succès !");
            }
            Ok(3) => {
                dex = init_pokedex();
                println!("\nPokedex réinitialisé avec succès !");
            }
            Ok(4) => {
                break;
            }
            _ => println!("\nChoix invalide, veuillez réessayer."),
        }
    }
    println!("Fermeture du pokedex...");
}
