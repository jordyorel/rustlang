pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn exploration() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }

    // #[test]
    // fn un_autre() {
    //     panic!("Fait échouer ce test");
    // }

    // #[test]
    // fn un_grand_peut_contenir_un_petit() {
    //     let le_grand = Rectangle {largeur: 8, hauteur: 7};
    //     let le_petit = Rectangle{largeur: 5, hauteur: 1};
    //     assert!(le_grand.peut_contenir(&le_petit));

    // }

    //  #[test]
    // fn un_petit_ne_peut_pas_contenir_un_plus_grand() {
    //     let le_grand = Rectangle {
    //         largeur: 8,
    //         hauteur: 7,
    //     };
    //     let le_petit = Rectangle {
    //         largeur: 5,
    //         hauteur: 1,
    //     };

    //     assert!(!le_petit.peut_contenir(&le_grand));
    // }


    // #[test]
    // fn cela_ajote_deux(){
    //     assert_eq!(4, ajouter_deux(2));
    // }

    // #[test]
    // fn acceuil_contient_nom() {
    //     let resultat = acceuil("Hierat");
    //     assert!(resultat.contains("Hierat"), 
    //     "Le resultat d'acceuil ne contient pas le nom, il vaut '{}'", resultat);
    // }


    // #[test]
    //  #[should_panic(expected = "La supposition doit être plus petite ou égale à 100")]
    // fn plus_grand_que_100(){
    //     Supposition::new(200);
    // }






    #[test]
    fn it_works()-> Result<(), String>{
        if 2 + 2 == 4 {
            Ok(())
        }else {
            Err(String::from("deux et deux ne vallent pas quatre"))
        }
    }

}



// #[derive(Debug)]
// struct Rectangle {
//     largeur: u32,
//     hauteur: u32,
// }

// impl Rectangle {
//     fn peut_contenir(self, other: &Rectangle) -> bool {
//         self.largeur > other.largeur && self.hauteur > other.hauteur
//     }
// }



// Tester l'égalité avec les macros assert_eq! et assert_ne!

    // pub fn ajouter_deux(a: i32) -> i32 {
    //     a + 2
    // }

//Ajouter des messages d'échec personnalisés

// pub fn acceuil(nom: &str) -> String {
//     // format!("Salut {}", nom)
//     String::from("salut") //le test ne fonctionnera pas
// }


//Vérifier le fonctionnement des paniques avec should_panic

pub struct Supposition {
    valeur: i32,
}

impl Supposition {
    pub fn new(valeur: i32) -> Supposition {
        if valeur < 1 || valeur > 100 {
            panic!("La valeur doit être comprise entre 1 et 100 vous avez entré '{}' ", valeur);
        }
        Supposition { valeur }
    }
}


