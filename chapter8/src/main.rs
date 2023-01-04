use std::fmt::format;



fn main() {
  
    let mut v = Vec::new();
        v.push(1); 
        v.push(2); 
        v.push(4);
    // comment acceder à une valeur contenue dans un vecteur

    //methode 1 via reférence
    let troisieme = &v[2];

    // methode 2 via la methode get()
    // let deuxieme = v.get(1);

    // ou bien
    //  match v.get(9) {
    //      Some(existe) => println!("{}", existe),
    //      None => println!("Il n'y a pas de nombre a cet index")
    //  };
    
   
    let mut vecteur = vec![1, 2, 3, 4, 5, 6];
    let un = &vecteur[0];

     //tentative d'ajout d'un element dans notre vecteur
    //alors que nous utilisons une référence à un element dans notre vecteur
    // vecteur.push(4);

    //Itérer sur les valeurs d'un vecteur

    //afficher les elemetent ddu vecteur en utilisant une boucle fo
    for i in &v {
        // println!("{}", i);
    }


    //modifier les element dans le vecteur
    for i in &mut v {
        *i += 10;
    }



    //comme un vecteur ne peut stocker que des donné du même type
    //si nous voulons stoker plusieurs types dans un vecteur nous pouvons utiliser un enum
    
    // Utiliser une énumération pour stocker différents types dans un vecteur
    #[derive(Debug)]
    enum Cellule {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let ligne = vec![
        Cellule::Int(4),
        Cellule::Float(4.5),
        Cellule::Text(String::from("voici un text"))
    ];

    for i in ligne  {
        // println!("{:?}", i);
    }


//----------------Stocker du texte encodé en UTF-8 avec les Strings-------------------//

        //--------------------diffrentes façons de créer une string-----------------//

// aveec la fonction new() 
let s = String::new(); //string vide


//convertir une literale de chaine en String
let data = "a pice of data";
data.to_string();

// ou on pouvait aussi faire:
let another_data = "another pice of data".to_string();

// utiliser la fonction String::from()
let h = String::from("This is a tring");

            //-----------------Modififier une string---------------------//

//Une String peut s'agrandir et son contenu peut changer,
// exactement comme le contenu d'un Vec<T>, si on y ajoute des données. 
//De plus, vous pouvez aisément utiliser l'opérateur + ou la macro format! 
//pour concaténer des valeurs String.

//Ajouter du texte à une chaîne avec push_str et push
//La méthode push_str prend une slice de chaîne de caractères 
//car nous ne souhaitons pas forcément prendre possession du paramètre.
let mut s = String::from("Hello, ");
s.push_str("World");

//Utilisation d'une slice de chaîne de caractères 
//après avoir ajouté son contenu dans une String

let mut s = String::from("Je m'appelle ");
let s1 = "hierat seven";
s.push_str(s1);
// println!("{}", s);

//La méthode push prend un seul caractère en paramètre et l'ajoute à la String
let mut t = String::from("lol");
t.push('l');

//Concaténation avec l'opérateur + ou la macro format!
let t1 = String::from("Hello");
let t2 = String::from("world");
let t3 = t1 + &t2; // notez que s1 a été déplacé ici
                       // et ne pourra plus être utilisé


let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3; // ceci peut etre simplifié en tilisant la macro format!

let h1 = String::from("tic");
let h2 = String::from("tac");
let h3 = String::from("toe");

let h4 = format!("{}-{}-{}", h1, h2, h3);
// println!("{}",h4);


//L'indexation des Strings
let bonjour = "Здравствуйте";
let reponse = &bonjour[0..2];

// println!("{}", reponse);

//Les méthodes pour parcourir les chaînes de caractères

//la methode char() separe et retourne chaque valeur
for c in "Здравствуйте".chars() {
    // println!("{}", c);
}

//Aussi, la méthode bytes() va retourner chaque octet brut (Ascci values), 
//ce qui sera peut-être plus utile selon ce que vous voulez faire 

for b in "नमस्ते".bytes() {
    // println!("{}", b);
}


//-----------Stocker des clés associées à des valeurs dans des tables de hachage------//

use std::collections::HashMap;

//cree une table de hashge
let mut score = HashMap::new();
score.insert(String::from("A"), 10);
score.insert(String::from("B"), 50);

// creer une table de hashage a partir de deux vecteur grace a zip() et collect()

let equipes = vec!["A".to_string(), "B".to_string()];
let scores_initiaux = vec![10, 50];

let mut scores :HashMap<_, _> = equipes.into_iter().zip(scores_initiaux.into_iter()).collect();
scores.insert(String::from("C"), 25);


for (k,v) in scores{
    // println!("{} {}", k,v);
}

//Les tables de hachage et la possession

let mon_champ = "couleur".to_string();
let valeur_champ = "bleu".to_string();

let mut ma_table = HashMap::new();
ma_table.insert(mon_champ, valeur_champ); // nom_champ et valeur_champ ne sont plus en vigueur à partir d'ici,

// println!("{:?}", ma_table);

for (k,v) in ma_table{
    // println!("{} {}", k,v);
}

//Accéder aux valeurs dans une table de hachage
//Nous pouvons obtenir une valeur d'une table de hachage en passant sa clé à la méthode get
let mut resultat = HashMap::new();
resultat.insert("Bleu".to_string(), 10);
resultat.insert("Jaune".to_string(), 20);

let mon_equipe = "Bleu".to_string();
let score = resultat.get(&mon_equipe);

// match score {
//     Some(a) => println!("Votre score est de: {}", a),
//     None => println!("Rien a afficher")
// };

//Modifier une table de hachage

//Réécrire une valeur
let mut couleurs = HashMap::new();
couleurs.insert(String::from("Noir"), 7);
couleurs.insert(String::from("Vert"), 28);
couleurs.insert(String::from("Noir"), 7); //ne sera pas inserée

// println!("{:?}", couleurs);

//Ajouter une valeur seulement si la clé n'a pas déjà de valeur

couleurs.entry(String::from("Jaune")).or_insert(50); //cette valeur sera inserée a la table de hashage
couleurs.entry(String::from("Noir")).or_insert(45); // ne serat pas inséree

// println!("{:?}", couleurs);

//Modifier une valeur en fonction de l'ancienne valeur

let mot = "Bonjour tout le monde quelle bonne journée pour écrire un peu du code, le code c'est magnifique".to_string();
let mut table = HashMap::new();

for mot in mot.split_whitespace() {
    let compteur = table.entry(mot).or_insert(0);
    *compteur += 1;
}

println!("{:?}", table);

}



