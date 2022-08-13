pub fn supprimer_projet(nom : String, chemin_projet : String){
    enlever_la_ligne_du_projet_dans_fichier_information(nom);
    supprimer_fichier_projet(chemin_projet);
}

fn remplacer_fichier_information(nouveau_contenu_fichier : String){
    println!("Mise à jour du fichier d'information du Framework...");
    //1. On supprime le fichier d'information, sachant qu'on a déja récupérer son contenu
    std::fs::remove_file("projets\\projets.info").expect("Erreur durant la suppression du fichier d'information.");
    //2. On le recréer
    std::fs::File::create("projets\\projets.info").expect("Erreur durant la régénération du fichier d'information.");
    //3. On replace le contenu de l'ancien fichier (sans le projet qui a été supprimé) dans le nouveau fichier créer, comme si de rien n'était.
    std::fs::write("projets\\projets.info", nouveau_contenu_fichier).expect("Erreur durant le re-remplissage du fichier d'information.");

    println!("fini.");
}

fn supprimer_fichier_projet(chemin_projet : String){
    std::fs::remove_dir_all(chemin_projet.clone()).expect("Impossible de supprimer les fichiers du projets.");
}

fn enlever_la_ligne_du_projet_dans_fichier_information(nom : String){
    let contenu_fichier = std::fs::read_to_string("projets\\projets.info").expect("Erreur durant la lecture du fichier listant les projets du framework.");
    let mut contenu_fichier_sans_projet = String::new();
    if le_nom_du_projet_est_dans_le_fichier(nom.clone(), contenu_fichier.clone()) {
        println!("Le nom du projet est valide.");
        for ligne in std::fs::read_to_string("projets\\projets.info").unwrap().lines(){
            if !ligne.contains(&nom) {
                contenu_fichier_sans_projet = contenu_fichier_sans_projet + ligne;
            }
        }
        remplacer_fichier_information(contenu_fichier_sans_projet.clone());
    }
    else{
        panic!("Le projet est introuvable. Vous avez peut-être fait une faute de frappe ?");
    }
}
//On vérifie que le nom du projet et dans le contenu du fichier.
fn le_nom_du_projet_est_dans_le_fichier(nom_projet : String, contenu_fichier : String) -> bool{
    return contenu_fichier.contains(&nom_projet);
}