pub struct Action{
    nom : String,
    nom_reduit : String,
    parametres : Vec<String>,
}

impl Action{
    pub fn new(nom : String, nom_reduit : String, parametres : Vec<String>) -> Self{
        return Self{
            nom,
            nom_reduit,
            parametres,
        };
    }

    pub fn avoir_nom(&self) -> String{
        return self.nom.clone();
    }

    pub fn avoir_nom_reduit(&self) -> String{
        return self.nom_reduit.clone();
    }

    pub fn avoir_parametres(&self) -> Vec<String>{
        return self.parametres.clone();
    }
}