use std::fmt::format;




pub trait Resumable {
    fn resumer(&self) -> String{
        String::from("(En savoir plus...)")
    } 
}

pub struct ArticleDePresse {
    pub titre: String,
    pub lieu: String,
    pub auteur: String,
    pub contenu: String,
}

impl Resumable for ArticleDePresse {
    fn resumer(&self) -> String {
         format!("{}, par {} ({})", self.titre, self.auteur, self.lieu)
    }
}

pub struct Tweet {
    pub nom_utilisateur: String,
    pub contenu: String,
    pub reponse: bool,
    pub retweet: bool,
}

impl Resumable for Tweet {
    fn resumer(&self) -> String {
        format!("{} : {}", self.nom_utilisateur, self.contenu)
    }
}

//Implémentations par défaut

pub trait Resumable1 {
    fn resumer_auteur(&self) -> String;

    fn resumer1(&self) -> String {
        format!("(Lire plus d'éléments de {} ...)", self.resumer_auteur())
    }
}

impl Resumable1 for Tweet {
     fn resumer_auteur(&self) -> String {
        format!("@{}", self.nom_utilisateur)
    }
}




