
use structure::Utilisateur;
use  structure::Rectangle;
mod structure;

fn main() {
    
   let _utilisateur: Utilisateur = structure
   ::creer_utilisateur("jordy@.com".to_string()
   , "seven".to_string());

    let mut utilisateur1: Utilisateur = Utilisateur {
        email: String::from("quelquun@example.com"),
        pseudo: String::from("pseudoquelconque123"),
        actif: true,
        nombre_de_connexions: 1,
    };

   let utilisateur2: Utilisateur = Utilisateur{
     email: String::from("quelquundautre@example.com"),
     ..utilisateur1

   };

  //  let dimension: (u32, u32) = (53, 43);

  //  let aire:u32 = structure::aire(dimension);
  //  println!("{}", aire);


   //ici nous utilison le type struc pour faire la même chose que la fonction précédente 
   let rect1:Rectangle = Rectangle{
      hauteur: 53,
      largeur: 43,
   };

   let aire1: u32 = structure::aire1(&rect1);
   println!("{}", aire1);
  //  dbg!(rect1);

   let rect2:Rectangle = Rectangle {
    hauteur:1,
    largeur:2,
   };

   // on fait appel à la methode aire implémentée par Rectangle
   println!("{} {}",rect2.aire(), rect2.largeur());  



    let rect3: Rectangle = Rectangle {
        largeur: 4,
        hauteur: 5,
    };


    println!("{}",rect2.peut_contenir(&rect3));



}
