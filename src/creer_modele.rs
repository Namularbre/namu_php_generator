use std::fs::create_dir_all;
use std::path::Path;
use std::fs::read_to_string;
use std::fs::write;
use std::fs::File;

pub fn creer_modele(nom_modele : String, chemin_projet : String){
    verifier_que_dossier_modeles_existe(chemin_projet.clone());
    println!("Création du fichier...");
    let chemin_modele = chemin_projet + "/modeles/" + &nom_modele + ".php";
    let mut contenu_modele_php = read_to_string("projets/projets.info").expect("Erreur durant la lecture du sample des modeles.");
    contenu_modele_php = str::replace(&contenu_modele_php, "MODELE", &nom_modele);
    File::create(chemin_modele.clone()).expect("Impossible de créer le fichier du modele.");
    println!("Remplissage du fichier...");
    write(chemin_modele.clone(), contenu_modele_php).expect("Impossible de remplir le fichier avec le sample des modeles");
    println!("fini !");
}
//Cette fonction recreer le dossier "modeles" s'il n'existe pas.
pub fn verifier_que_dossier_modeles_existe(chemin_projet : String){
    if !Path::new(&format!("{}", chemin_projet.clone() + "/modeles")).is_dir(){
        create_dir_all(chemin_projet.clone() + "/modeles").expect("Impossible de créer le dossier des modeles (qui a été supprimé avant cette opération)");
        println!("Le dossier \"modeles\" est introuvable, nous l'avons recréé");
    }
}