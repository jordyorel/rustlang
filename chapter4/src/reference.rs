

pub fn calculer_taille(s: &String) -> usize {
    s.len()
}

pub fn calculer_taille1(s: &String) ->  usize {
  let taille = s.len();
  println!("{}", taille);
   taille
}


pub fn changer(s: &mut String) -> String {
    
    s.push_str(", world");
    s.to_string()

}