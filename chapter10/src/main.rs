use crate::trait_types::{Resumable, Resumable1};


mod plus_grand;
mod trait_types;
mod life_time;

fn main() {
    let list = vec![10, 20, 25, 30, 35, 40, 45, 60];
    let liste_de_caracteres = vec!['y', 'm', 'a', 'q'];


    // println!("{}", plus_grand::le_plus_grand_nombre(&list));
    // println!("{}", plus_grand::le_plus_grand_char(&liste_de_caracteres));
    // println!("{}", plus_grand::le_plus_grand(&liste_de_caracteres));
    // println!("{}", plus_grand::le_plus_grand(&list));
    

    //Dans la définition des structures
    let entier = plus_grand::Point{x: 5, y:8}; //les deux valeurs doivent être du même type
    let flotant = plus_grand::Point{x: 5.5, y:8.8};

    let deux_entiers = plus_grand::Point1 { x: 5, y: 10 };
    let deux_flottants = plus_grand::Point1 { x: 1.0, y: 4.0 };
    let un_entier_et_un_flottant = plus_grand::Point1 { x: 5, y: 4.0 };

    //Dans la définition des méthodes
    // let point  = plus_grand::Coordonne{x:5, y:10};
    // println!("{}", point.x());

    // let point1  = plus_grand::Coordonne{x:7.5, y:10.3};

    // println!("{}", point1.distance_depuis_origine());

    let p1 = plus_grand::Coordonne1 { x: 5, y: 10.4 };
    let p2 = plus_grand::Coordonne1 { x: "Hello", y: 'c' };

    let p3 = p1.melanger(p2);

    // println!("p3.x = {}, p3.y = {}", p3.x, p3.y);


    //--------------Définir des comportements partagés avec les traits---------//

    let tweet = trait_types::Tweet {
        nom_utilisateur: String::from("Hierat"),
        contenu: String::from("Ceci est est tweet de jordy"),
        reponse: false,
        retweet: false,
    };



    // println!("Un nouveau tweeet: {}", tweet.resumer());

    let article = trait_types::ArticleDePresse {
        titre: String::from("Les Penguins ont remporté la Coupe Stanley !"),
        lieu: String::from("Pittsburgh, PA, USA"),
        auteur: String::from("Iceburgh"),
        contenu: String::from(
            "Les Penguins de Pittsburgh sont une nouvelle fois la meilleure \
            équipe de hockey de la LNH.",
        ),
    };
    // println!("Nouvel article disponible ! {}", article.resumer());

    // println!("{}", tweet.resumer1());

    pub fn notifier(element: &impl Resumable1) {
        // println!(" Flash infos : {}", element.resumer1());
    }

    //Eviter les références pendouillantes avec les durées de vie

    // Le code suivant ne compilera pas car x ne vit pas aussi lontemps que r
    // {
    //     let r;
    //     {
    //         let x = 5;
    //         r = &x;
    //     }
    //     println!("r: {}", r);
    // }

// ceci est la version correct du code précédent
{
    let r;
    
    let x = 5;
    r = &x;
   
    // println!("r: {}", r);
}


    //Les durées de vies génériques dans les fonctions
    let x = String::from("Je suis une string");
    let y = "abc";

    let plus_long = life_time::la_plus_longue(x.as_str(), y);
    // println!("le plus long est: {}", plus_long);


    // ceci ne fonctionnera pas car string1 vit plus longtemps que string2
    let string1 = String::from("une longue chaîne est longue");

    {
        let string2 = String::from("xyz");
        let resultat = life_time::la_plus_longue(string1.as_str(), string2.as_str());
        // println!("La chaîne la plus longue est {}", resultat);
    }


    let plus_long1 = life_time::la_plus_longue1(x.as_str(), y);

    // println!("{}",plus_long1);

    //L'ajout des durées de vies dans les définitions des structures
    let roman = String::from("Ceci est une extrait du livre de Cheuck Anta Diop");
    let premiere_phrase = roman.split('.').next().expect("Impossible de trouver '.' ");
    let i = life_time::ExtraitImportant {
        partie: premiere_phrase
    };

    println!("{:#?}",i );






}



