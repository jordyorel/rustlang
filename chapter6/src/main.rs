
mod enums;
mod struct_match;


fn main() {
    
    use enums::AdresseIp;
    let local = AdresseIp::V4(127,0,0,1);
    let rebouclage = AdresseIp::V6(String::from("::1"));
    
    use enums::Message;
    let m = Message::Ecrire(String::from("Hello"));
    m.appeler();
    
    
    //structure macth
    use struct_match::{EtatUs, PieceUs};
    crate::struct_match::valeur_en_centimes(PieceUs::Quarter(EtatUs::Alabama));
    crate::struct_match::valeur_en_centimes(PieceUs::Quarter(EtatUs::Alaska));
    crate::struct_match::valeur_en_centimes1(PieceUs::Penny);
    crate::struct_match::valeur_en_centimes2(PieceUs::Quarter(EtatUs::Alaska));
    crate::struct_match::valeur_en_centimes1(PieceUs::Dime);
    crate::struct_match::valeur_en_centimes1(PieceUs::Nickel);


    //structure match avec option<T>
    let cinq = Some(5);
    let six = struct_match::plus_un(& cinq);
    let none = struct_match::plus_un(& None);



    // Les motifs génériques et le motif _

    let jete_de_de = 9;
    match jete_de_de {
        3 => ajouter_chapeau_fantaisie(),
        7 => enleve_chapeau_fantaisie(),
        _ => relancer(),
    }

    
    let jete_de_de = 9;
    match jete_de_de {
        3 => ajouter_chapeau_fantaisie(),
        7 => enleve_chapeau_fantaisie(),
        _ => (),
    }

    fn ajouter_chapeau_fantaisie() {}
    fn enleve_chapeau_fantaisie() {}
    fn relancer() {}

    
    // if let
    let une_valeur_u8 = Some(3u8);
    match une_valeur_u8 {
        Some(max) => println!("Le maximum est réglé sur {}", max),
        _ => (),
    }

    //same as
    let une_valeur_u8 = Some(3u8);
    if let Some(max) = une_valeur_u8 {
        println!("Le maximum est réglé sur {}", max);
    }

}
