


pub enum PieceUs {
    Penny,
    Nickel,
    Dime,
    Quarter(EtatUs),
}

#[derive(Debug)]
pub enum EtatUs {
    Alabama,
    Alaska,
}

pub fn valeur_en_centimes(piece: PieceUs) -> u8 {
    match piece {
        PieceUs::Penny => 1,
        PieceUs::Nickel => 5,
        PieceUs::Dime => 10,
        PieceUs::Quarter(etat) => {
            println!("Il s'agit d'un quarter de l'État de {:?} !", etat);
            25
        },
    }
}

pub fn valeur_en_centimes1(piece: PieceUs)  {
    let mut compteur = 0;
    if let PieceUs::Quarter(etat) = piece {
        println!("Il s'agit d'un quarter de l'État de {:?} !", etat);
    } else {
        compteur += 1;
    }
}

pub fn valeur_en_centimes2(piece: PieceUs) {
    let mut compteur = 0;
    match piece {
        PieceUs::Quarter(etat) => println!("Il s'agit d'un quarter de l'État de {:?} !", etat),
        _ => compteur += 1,
    }
}




 pub fn plus_un(x: & Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
    }
}
