use std::fmt::Display;


pub fn la_plus_longue<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x 
    } else{
            y
        }
}

//La façon dont vous avez à préciser les paramètres de durées //de vie dépend de ce que fait votre fonction. 
//Par exemple, si nous changions l'implémentation de la fonction la_plus_longue pour qu'elle retourne systématiquement 
//le premier paramètre plutôt que la slice de chaîne de caractères la plus longue, nous n'aurions pas besoin de renseigner 
//une durée de vie sur le paramètre y. Le code suivant se compile :


pub fn la_plus_longue1<'a>(x: &'a str, y: &str) -> &'a str {

    x 

}


//L'ajout des durées de vies dans les définitions des structures

#[derive(Debug)]
pub struct ExtraitImportant<'a>{
    pub partie: &'a str,
}


// Regardons brièvement la syntaxe pour renseigner tous les paramètres de type génériques, 
//les traits liés, et les durées de vies sur une seule fonction !

pub fn la_plus_longue_avec_annonce<'a ,T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
    where T: Display 
{
        println!("Annonce ! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
}
