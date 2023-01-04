
//Les structures nous permettent de créer des types personnalisés significatifs
pub struct Utilisateur {
    pub actif: bool,
    pub pseudo: String,
    pub email: String,
    pub nombre_de_connexions: u64,
}


#[derive(Debug)]
pub struct Rectangle{
    pub largeur: u32,
    pub hauteur: u32,
}

impl Rectangle {
    pub fn aire(&self) -> u32 {
        self.hauteur * self.largeur
    }

    pub fn largeur(&self) -> bool{
        self.largeur > 0
    }

    pub fn peut_contenir(&self, autre: &Rectangle) -> bool {
        self.largeur > autre.largeur && self.hauteur > autre.hauteur
    }
}


pub fn creer_utilisateur(email: String, pseudo: String) -> Utilisateur {
    Utilisateur {
        actif: true,
        pseudo,
        email,
        nombre_de_connexions: 1,
    }
}

//on utilise un paramètre de type tuple
pub fn aire(rect:(u32, u32)) -> u32 {
    rect.0 * rect.1
}

// on utilise un paramètre de type Rectangle ici on est plus explicite
pub fn aire1(rect: &Rectangle) -> u32 {
    rect.largeur * rect.hauteur
}


