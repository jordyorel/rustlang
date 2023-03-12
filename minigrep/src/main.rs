use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problème rencontré lors de l'interprétation des arguments : {}", err);
        process::exit(1)
    });

    println!("On cherche: {:?}", config.recherche);
    println!("Dans le fichier: {:?}", config.nom_fichier);

    if let Err(e) = run(config) {
        eprintln!("Erreur applicative: {}", e);

        process::exit(1);
    };
    
}


