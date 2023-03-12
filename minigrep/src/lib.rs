use std::error::Error;
use std::{fs, env};


pub struct Config {
    // definition d'une structure qui nous
    // servira de definir les variable qui
    // contiendront le text à rechercher
    // et la fichier dans lequel rechercher
    pub recherche: String,
    pub nom_fichier: String,
    pub sensible_casse: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        //args contient la list des arguments récupérés dans la 
        //ligne de commande nous les stockons dans les deux 
        //variables recherche et mon_fichier 
        // ensuite nous les renvoyons sous forme d'un struc
        if args.len() < 3 {
            return Err("Il n'y a pas assez d'arguments")
        }
        let recherche = &args[1]; // j'ai enlevé clone()
        let nom_fichier = &args[2]; //j'ai enlevé clone()
        let sensible_casse = env::var("MINIGREP_INSENSIBLE_CASSE").is_err();

        Ok(Config{recherche:recherche.to_string(), 
            nom_fichier: nom_fichier.to_string(), 
            sensible_casse
        })
    }
    
}


pub fn run(config:Config) -> Result<(), Box<dyn Error>> {
    // cette fonction lie le fichier récupéré depuis la ligne de commande 
    // et stocké dans la variale mon_fichier, ensuite la skocke to 
    // la variable contenu
    let contenu = fs::read_to_string(config.nom_fichier)?;
    let resultats = if config.sensible_casse {
        rechercher(&config.recherche, &contenu)
    } else {
        rechercher_insensible_casse(&config.recherche, &contenu)
    };
    for line in resultats{
        println!("{}", line);
    }
    Ok(())
}




pub fn rechercher<'a>(recherche: &str, contenu: &'a str) -> Vec<&'a str> {
    //cette fonction recherche le mot dans la fichier indiqué
    let mut resultats = Vec::new();
    for lines in contenu.lines() {
        if lines.contains(recherche) {
            resultats.push(lines);
        }
    }
    resultats
    // vec![]
}


pub fn rechercher_insensible_casse<'a>(
    recherche: &str, 
    contenu: &'a str
) -> Vec<&'a str> {
    let recherche = recherche.to_lowercase();

    let mut resultats = Vec::new();
    for lines in contenu.lines() {
        if lines.to_lowercase().contains(&recherche) {
            resultats.push(lines);
        }
    }
    resultats
}










//Tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn sensible_casse() {
        let recherche = "duct";
        let contenu = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.";

        assert_eq!(vec!["sécurité, rapidité, productivité."], rechercher(recherche, contenu))
    }


     #[test]
    fn insensible_casse() {
        let recherche = "rUsT";
        let contenu = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.
C'est pas rustique.";

        assert_eq!(
            vec!["Rust:", "C'est pas rustique."],
            rechercher_insensible_casse(recherche, contenu)
        );
    }


}



