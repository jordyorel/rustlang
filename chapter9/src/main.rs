use std::fs::File; // renvoie un Result que nous devons gérer
use std::io::ErrorKind;
use std::io::{self, Read};
use std::fs;
use std::error::Error;
use std::net::IpAddr;

fn main()  -> Result<(), Box<dyn Error>> {
    // let v = vec![2,4, 6, 8];
    // v[99];


    //------------Des erreurs récupérables avec Result-----------//


    // let file  = File::open("hello.txt");

    // let file = match file {
    //     Ok(fichier) => fichier,
    //     Err(erreur) => panic!("Le fichier n'existe pas {:?}",erreur ), 
    // };

    //-----------Gérer les différentes erreurs

    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(fichier) => { println!("Fichier ouvert avec succes"); fichier} ,
    //     Err(err) => match err.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => {println!("Fichier créer avec succes"); fc},
    //             Err(e) => panic!("Erreur de création du fichier{:?}", e),
    //         },
    //         autre_erreur => {
    //             panic!("Erreur d'ouvertur du fichier {:?}", autre_erreur);
    //         }
    //     },
    // };
//une autre manière d'écrire la même logique 
//en utilisant les fermetures et la méthode unwrap_or_else

// à revoir après le chapitre 13
// let fichier = File::open("hello.txt").unwrap_or_else(|err|{
//     if err.kind() == ErrorKind::NotFound {
//         File::create("hello.txt").unwrap_or_else(|erreur|{
//             panic!("Erreur de creation du fichier{:?}", erreur);
//         })
//     }else {
//         panic!("Erreur d'ouverture du fichier {:?}", err);
//     }
// });


//--------Raccourcis pour faire un panic lors d'une erreur : unwrap et expect

// avec unwrap
// let file = File::open("hello.txt").unwrap();

//avec expect
// let file = File::open("hello.txt").expect("Erreur d'ouverture du fichier");


//---------------Propager les erreurs
// println!("{:?}", lire_pseudo_depuis_fichier());


//Un raccourci pour propager les erreurs : l'opérateur ?
// println!("{:?}", lire_pseudo_depuis_fichier1());
// println!("{:?}", lire_pseudo_depuis_fichier2());
// println!("{:?}", lire_pseudo_depuis_fichier3());

//Où l'opérateur ? peut être utilisé

//ceci produira une erreur car nous 
//sommes autorisés à utiliser l'opérateur ? 
//uniquement dans une fonction qui retourne Result, Option, 
//ou un autre type qui implémente FromResidual
//ce qui n'est pas le cas de main qui retourne ()
//  let file  = File::open("hello.txt")?;



let s = String::from("retourne mone dernier caractere");
println!("{:?}", denier_caractere_de_la_premiere_ligne(&s));

let home:IpAddr = "127.0.0.1".parse().unwrap();
println!("{:?}", home);






Ok(())

}


fn lire_pseudo_depuis_fichier() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(fichier) => fichier,
        Err(e) => return Err(e),
    };
    
    let mut s = String::from("la string a lire");
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}


fn lire_pseudo_depuis_fichier1() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::from("La string a lire");
    f.read_to_string(&mut s)?;
    Ok(s)
}

//Nous pouvons même encore plus réduire ce code 
//en enchaînant immédiatement les appels aux méthodes après le ?

fn lire_pseudo_depuis_fichier2() -> Result<String, io::Error> {
    let mut s = String::from("La string a lire");
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

//un autre racourcis en utilisant directement fs::
fn lire_pseudo_depuis_fichier3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}


// ? peut aussi être utilisé avec des valeurs de type Option<T>
//exemple de fonction qui trouve le dernier caractère de la première 
//ligne dans le texte qu'on lui fournit 

fn denier_caractere_de_la_premiere_ligne(texte: &str) -> Option<char> {
    texte.lines().next()?.chars().last()
}