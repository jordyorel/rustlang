  
  
pub enum AdresseIp {
        V4(u8, u8, u8, u8),
        V6(String),
}

#[derive(Debug)]

pub enum Message {
    Quitter,
    Deplacer { x: i32, y: i32 },
    Ecrire(String),
    ChangerCouleur(i32, i32, i32),
}

impl Message {
    pub fn appeler(&self) {
        println!("Un message");
    }
}