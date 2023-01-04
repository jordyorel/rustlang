

pub fn prendre_possession(texte: String) { // texte rentre dans la portée.
  println!("{}", texte);
} // Ici, texte sort de la portée et `drop` est appelé. La mémoire est libérée.

pub fn creer_copie(entier: i32) { // entier rentre dans la portée.
  println!("{}", entier);
} 

pub fn donne_possession() -> String {      // donne_possession va déplacer sa
                                       // valeur de retour dans la
                                       // fonction qui l'appelle.
  let texte = String::from("yours not mine");   // texte rentre dans la portée.

  texte                                // texte est retournée et
                                       // est déplacée vers le code qui
                                       // l'appelle.
}

// Cette fonction va prendre une String et en retourne aussi une.
pub fn prend_et_rend(texte: String) -> String { // texte rentre dans la portée.

  texte  // texte est retournée et déplacée vers le code qui l'appelle.
} 