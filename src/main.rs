use structopt::StructOpt;

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
    //Pourquoi ça marche pas ?
    else if options.action == "connexion_bdd" || options.action == "bdd"{
        let hote = options.parametres[0].clone();
        let nom_bdd = options.parametres[1].clone();
        let utilisateur = options.parametres[2].clone();
        let mdp_uti = options.parametres[3].clone();
        let nom_projet = options.parametres[4].clone();
        let racine = trouver_chemin_projet(nom_projet);
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

fn trouver_chemin_projet(nom_projet : String) -> String{
    let lignes_fichier_info = std::fs::read_to_string("projets\\projets.info").unwrap();
    for ligne in lignes_fichier_info.lines(){
        if ligne.contains(&nom_projet){
            let mut iteration = 0;
            for champs in ligne.split_whitespace(){
                if iteration == 1{
                    return champs.to_owned();
                }
                iteration += 1;
            }
        }
    }
    panic!("Impossible de trouver le chemin du projet.");
}