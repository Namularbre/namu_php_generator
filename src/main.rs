use structopt::StructOpt;
use std::io::{self};

//import des modules
mod creer_projet;
mod creer_connexion_bdd;
mod supprimer_projet;

#[derive(StructOpt)]
struct Options{
    action : String,
    parametres : Vec<String>,
}

fn main() {
    let options = Options::from_args();

    if options.action == "nouveau" || options.action == "n" {
        let chemin_projet = options.parametres[0].clone();
        let nom_projet = options.parametres[1].clone();
        let arborescence = creer_projet::ArborescenceProjet::new(chemin_projet, nom_projet);
        arborescence.creer_arborescence().expect("Il y a eu une erreur durant la création du nouveau projet.");
        println!("Votre projet est prêt ! Il vous attends :3");
    }
    else if options.action == "connexion_bdd" || options.action == "bdd"{
        let hote = options.parametres[0].clone();
        let nom_bdd = options.parametres[1].clone();
        let utilisateur = options.parametres[2].clone();
        let mdp_uti = options.parametres[3].clone();
        let racine = std::fs::read_to_string("C:/Users/namul/testage/namu_php_framework.info").unwrap();
        creer_connexion_bdd::creer_connexion_bdd(racine ,hote, nom_bdd, utilisateur, mdp_uti);
    }
    else if options.action == "supprimer" || options.action == "s"{
        let racine = options.parametres[0].clone();
        supprimer_projet::supprimer_projet(racine);
    }
    else{
        println!("L'action choisi n'est pas reconnue :/");
    }
}