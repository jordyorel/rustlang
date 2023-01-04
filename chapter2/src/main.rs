
use rand::Rng;
use std::{io, cmp::Ordering};

fn main() {
    
    println!("Devinez un nombre");

    let nombre_scret = rand::thread_rng().gen_range(1..100);
  



    loop {

        println!("Veuiller entrer un nombre entre 1 > 100");
        let mut supposition = String::new();

         io::stdin()
            .read_line(&mut supposition)
            .expect("Échec de la lecture de l'entrée utilisateur");


        let supposition: i32 = match supposition.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => continue,
        
        };

        if supposition < 1 || supposition >100 {
            println!("le nombre est entre 1 et 100");
            continue;
        }

        match supposition.cmp(&nombre_scret) {
            Ordering::Less => println!("C'est plus"),
            Ordering::Greater =>  println!("C'est moins"),
            Ordering::Equal => {
                println!("Vous avez gagné");
                break;
            }
        }
    }

    // let a = [10, 20, 30, 40, 50];

    // for i in a {
    //     println!("Le nombre à l'indice est:{}", i);
            
    // }

    // for nombre in (0..4).rev() {
    //     println!("{}", nombre);
    // }




}


pub struct Supposition {
    valeur: i32,
}

impl Supposition {
    pub fn new(valeur: i32) -> Supposition {
        if valeur < 1 || valeur > 100 {
            panic!("Supposition valeur must be between 1 and 100, got {}.", valeur);
        }

        Supposition { valeur }
    }

    pub fn valeur(&self) -> i32 {
        self.valeur
    }
}





