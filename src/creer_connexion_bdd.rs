use std::fs::create_dir_all;
use std::fs::File;
use std::fs::write;

pub fn creer_connexion_bdd(chemin_projet : String, hote : String, nom_bdd : String, utilisateur : String, code : String){
    if creer_fichier_connexion_bdd(chemin_projet.clone()).is_ok(){
        let contenu_fichier_bdd = preparer_contenu_classe_bdd(hote, nom_bdd, utilisateur, code);
        let chemin_fichier_classe_bdd = avoir_chemin_fichier_classe_bdd(chemin_projet);
        if remplir_fichier_classe_bdd(contenu_fichier_bdd, chemin_fichier_classe_bdd).is_ok(){
            println!("Opération terminée !");
        }
        else{
            panic!("Il y a eu une erreur durant le ramplissage du fichier de la classe bdd (Bdd.php)");
        }
    }
}

fn creer_fichier_connexion_bdd(chemin_projet : String) -> std::io::Result<()>{
    println!("Création des fichiers et dossier nécessaire...");
    let chemin_dossier_connexion_bdd = chemin_projet.clone() + "/connexion_bdd";
    create_dir_all(chemin_dossier_connexion_bdd.clone()).expect("Erreur durant la création du dossier qui contiendra le système de connexion à la base de données.");
    
    let chemin_fichier_classe_bdd = chemin_dossier_connexion_bdd.clone() + "/Bdd.php";
    File::create(chemin_fichier_classe_bdd.clone()).expect("Erreur durant la création du fichier contenant la classe Bdd");
    return Ok(());
}

fn avoir_chemin_fichier_classe_bdd(chemin_projet : String) -> String{
    return (chemin_projet.clone() + "/connexion_bdd/Bdd.php").clone();
}

fn preparer_contenu_classe_bdd(hote : String, nom_bdd : String, utilisateur : String, code : String) -> String{
    println!("Préparation du contenu de la classe Bdd avec les informations que vous avez donné.");
    let mut contenu_sample_fichier_bdd = std::fs::read_to_string("samples/bdd.php.sample").unwrap();
    contenu_sample_fichier_bdd = str::replace(&contenu_sample_fichier_bdd, "HOTE", &hote);
    contenu_sample_fichier_bdd = str::replace(&contenu_sample_fichier_bdd, "NOM_BDD", &nom_bdd);
    contenu_sample_fichier_bdd = str::replace(&contenu_sample_fichier_bdd, "UTILISATEUR", &utilisateur);
    contenu_sample_fichier_bdd = str::replace(&contenu_sample_fichier_bdd, "MDP", &code);

    return contenu_sample_fichier_bdd;
}

fn remplir_fichier_classe_bdd(contenu_fichier_bdd : String, chemin_fichier_classe_bdd : String) -> std::io::Result<()>{
    println!("Remplissage du fichier connexion_bdd/Bdd.php");
    write(chemin_fichier_classe_bdd, contenu_fichier_bdd).expect("Erreur durant le remplissage du fichier de la classe BDD.");
    return Ok(());
}