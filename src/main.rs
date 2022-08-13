use structopt::StructOpt;

//import des modules
mod creer_projet;
mod creer_connexion_bdd;
mod supprimer_projet;
mod creer_modele;

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
        let nom_projet = options.parametres[4].clone();
        let chemin_projet = trouver_chemin_projet(nom_projet);
        creer_connexion_bdd::creer_connexion_bdd(chemin_projet ,hote, nom_bdd, utilisateur, mdp_uti);
    }
    else if options.action == "supprimer" || options.action == "s"{
        let nom_projet = options.parametres[0].clone();
        let chemin_projet = trouver_chemin_projet(nom_projet.clone());
        supprimer_projet::supprimer_projet(nom_projet, chemin_projet);
    }
    else if options.action == "creer_modele" || options.action == "cm"{
        let nom_modele = options.parametres[0].clone();
        let nom_projet = options.parametres[1].clone();
        let chemin_projet = trouver_chemin_projet(nom_projet);
        creer_modele::creer_modele(nom_modele, chemin_projet);
    }
    else{
        println!("L'action choisi n'est pas reconnue :/");
    }
}

fn trouver_chemin_projet(nom_projet : String) -> String{
    let lignes_fichier_info = std::fs::read_to_string("projets\\projets.info").unwrap();
    for ligne in lignes_fichier_info.lines(){
        if ligne.contains(&nom_projet){
            let mut iterateur = ligne.split_whitespace();
            iterateur.next();
            return String::from(iterateur.next().unwrap());
        }
    }
    panic!("Impossible de trouver le chemin du projet.");
}