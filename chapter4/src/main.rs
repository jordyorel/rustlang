
mod reference;
mod possession;
mod slice;

fn main() {

    //Possession

    // let s = String::from("Hello possession");
    // possession::prendre_possession(s);
    // println!("{:?}", s); //erreur

    let x = 7;
    // possession::creer_copie(x);  //similaire à la fonctio prend_possession sauf que ceci marche parce que les srings et les nombre ne sont stoker dans la memoire de la même façon en rus
    // println!("{}",x); // pas d'erreur

    // let recuper_possession = possession::donne_possession();
    // println!("{}", recuper_possession);

    // let s1 = String::from("prend_et rend Hello");
    // println!("{}",possession::prend_et_rend(s1));
    // println!("{:?}", s1); //erreur car s1 est libérée de la mémoire
    

  //________________________________________________________________//
    //Référence

    // let mut s2 = String::from("Hello");
    // println!("{}",reference::calculer_taille(&s2));  
    
    // reference::calculer_taille1(&s2);

    // println!("{}",reference::changer(&mut s2));

  //________________________________________________________________//
  //slice
  
  let mut texte = String::from("une string");

  println!("{}", slice::premier_mot(&texte));




  texte.clear();  // ceci liber texte de la mémoire
  // println!("{}", texte);  // va afficher un vide comme quoi texte n'existe plus


}

