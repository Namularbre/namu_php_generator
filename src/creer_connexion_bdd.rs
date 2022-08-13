use std::fs::create_dir_all;
use std::fs::File;
use std::fs::write;

pub fn creer_connexion_bdd(chemin_projet : String, hote : String, nom_bdd : String, utilisateur : String, code : String){
    println!("Création du dossier pour la connexion...");
    let chemin_dossier_connexion_bdd = chemin_projet.clone() + "/connexion_bdd";
    create_dir_all(chemin_dossier_connexion_bdd.clone()).expect("Erreur durant la création du dossier qui contiendra le système de connexion à la base de données.");
    println!("fin");

    let chemin_fichier_classe_bdd = chemin_dossier_connexion_bdd.clone() + "/Bdd.php";
    println!("Création du fichier qui contiendra la classe de la BDD...");
    File::create(chemin_fichier_classe_bdd.clone()).expect("Erreur durant la création du fichier contenant la classe Bdd");
    let mut contenu_sample_fichier_bdd = std::fs::read_to_string("samples\\bdd.php.sample").unwrap();
    contenu_sample_fichier_bdd = str::replace(&contenu_sample_fichier_bdd, "HOTE", &hote);
    contenu_sample_fichier_bdd = str::replace(&contenu_sample_fichier_bdd, "NOM_BDD", &nom_bdd);
    contenu_sample_fichier_bdd = str::replace(&contenu_sample_fichier_bdd, "UTILISATEUR", &utilisateur);
    contenu_sample_fichier_bdd = str::replace(&contenu_sample_fichier_bdd, "MDP", &code);
    
    write(chemin_fichier_classe_bdd, contenu_sample_fichier_bdd).expect("Erreur durant le remplissage du fichier de la classe BDD.");
    println!("Opération terminée !");
}