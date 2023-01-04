
//Les types génériques, les traits et les durées de vie


// La généricité permet de remplacer des types concrets 
//ou d'autres propriétés par des paramètres abstraits appelés génériques.

// Dans la definition des fonctions

// pub fn le_plus_grand_nombre(list: &[i32]) -> i32 {
  
//     let mut plus_grand = list[0];

//     for &nombre in list.iter() {
//         if nombre > plus_grand{
//             plus_grand = nombre;
//         }
//     }
//     plus_grand
// }

// pub fn le_plus_grand_char(list: &[char]) -> char {
  
//     let mut plus_grand = list[0];

//     for &nombre in list.iter() {
//         if nombre > plus_grand{
//             plus_grand = nombre;
//         }
//     }
//     plus_grand
// }


//creons la même fonction que les deux précédentes en utilisant 
//le type générique

pub fn le_plus_grand <T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut plus_grand = list[0];

    for &nombre in list.iter() {
        if nombre > plus_grand{
            plus_grand = nombre;
        }
    }
    plus_grand
}

//Dans la définition des structures
pub struct Point <T> {
    pub x: T,
    pub y: T,
}


pub struct Point1 <T, U> {
    pub x: T,
    pub y: U,
}


//Dans les définitions d'énumérations
enum Option<T> {
    Some(T),
    None,
}

enum  Result <T, E>{
    Ok(T),
    Err(E),
}


//Dans la définition des méthodes

pub struct Coordonne <T> {
    pub x: T,
    pub y: T,
}
impl <T> Coordonne <T> {
    pub fn x(&self) -> &T {
        &self.x
    }
    
}

impl Coordonne<f32> {
    pub fn distance_depuis_origine(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


pub struct Coordonne1 <X1, Y1> {
    pub x: X1,
    pub y: Y1,
}

impl <X1, Y1>Coordonne1 <X1, Y1> {
    pub fn melanger  <X2, Y2>(self, other: Coordonne1<X2, Y2>) -> Coordonne1<X1, Y2> {
        Coordonne1 { 
            x: self.x, 
            y: other.y 
        }
    }
}






