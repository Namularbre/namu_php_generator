use std::fs::create_dir_all;
use std::path::Path;
use std::fs::read_to_string;
use std::fs::write;
use std::fs::File;

pub fn creer_modele(nom_modele : String, chemin_projet : String){
    verifier_que_dossier_modeles_existe(chemin_projet.clone());
    if creer_fichier_modele(chemin_projet.clone(), nom_modele.clone()).is_ok(){
        if remplir_fichier_classe_bdd(chemin_projet, nom_modele).is_ok() {
            println!("Opération terminé");
        }
        else{
            panic!("Impossible de remplir le fichier du modèle. Peut être un problème de droit d'accès ?");
        }
    }
    else{
        panic!("Impossible de créer le fichier du modèle. Peut-être un problème de droit d'accès ?");
    }
}
//Cette fonction recreer le dossier "modeles" s'il n'existe pas.
fn verifier_que_dossier_modeles_existe(chemin_projet : String){
    println!("Vérification que le dossier des modèles existe... (Sinon, on le crée)");
    if !Path::new(&format!("{}", chemin_projet.clone() + "/modeles")).is_dir(){
        create_dir_all(chemin_projet.clone() + "/modeles").expect("Impossible de créer le dossier des modeles (qui a été supprimé avant cette opération)");
        println!("Le dossier \"modeles\" est introuvable, nous l'avons recréé");
    }
}

fn avoir_contenu_modele(nom_modele : String) -> String{
    println!("Récupération et préparation du contenu du modèle...");
    let contenu_sample_modele = read_to_string("samples/modele.php.sample").expect("Erreur durant la lecture du sample des modeles.");
    return str::replace(&contenu_sample_modele, "MODELE", &nom_modele);
}

fn avoir_chemin_modele(chemin_projet : String, nom_modele : String) -> String{
    return chemin_projet + "/modeles/" + &nom_modele + ".php";
}

fn creer_fichier_modele(chemin_projet : String, nom_modele : String) -> std::io::Result<()>{
    let chemin_modele = avoir_chemin_modele(chemin_projet, nom_modele);
    println!("Création du fichier du modèle...");
    File::create(chemin_modele.clone()).expect("Impossible de créer le fichier du modele.");
    return Ok(());
}

fn remplir_fichier_classe_bdd(chemin_projet : String, nom_modele : String) -> std::io::Result<()>{
    let chemin_modele = avoir_chemin_modele(chemin_projet, nom_modele.clone());
    println!("Remplissage du fichier...");
    write(chemin_modele.clone(), avoir_contenu_modele(nom_modele)).expect("Impossible de remplir le fichier avec le sample des modeles");

    return Ok(());
}