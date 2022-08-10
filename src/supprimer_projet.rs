pub fn supprimer_projet(nom : String){
    let contenu_fichier = std::fs::read_to_string("projets\\projets.info").expect("Erreur durant la lecture du fichier listant les projets du framework.");
    let contenu_fichier_sans_projet = enlever_la_ligne_du_projet_dans_fichier_information(nom);
    
    if !contenu_fichier_sans_projet.is_empty(){
        remplacer_fichier_information(contenu_fichier_sans_projet.clone());
    }
    else{
        panic!("Le nom de projet que vous avez fournis n'est pas reconnu. Vérifier l'orthographe de votre saisie");
    }
}

fn remplacer_fichier_information(nouveau_contenu_fichier : String){
    println!("Mise à jour du fichier d'information du Framework...");

    std::fs::remove_file("projets\\projets.info").expect("Erreur durant la suppression du fichier d'information.");
    std::fs::File::create("projets\\projets.info").expect("Erreur durant la régénération du fichier d'information.");
    std::fs::write("projets\\projets.info", nouveau_contenu_fichier).expect("Erreur durant le re-remplissage du fichier d'information.");

    println!("fini.");
}

fn supprimer_fichier_projet(chemin_projet : String){
    std::fs::remove_dir_all(chemin_projet.clone()).expect("Impossible de supprimer les fichiers du projets.");
}

fn enlever_la_ligne_du_projet_dans_fichier_information(nom : String) -> String{
    let contenu_fichier = std::fs::read_to_string("projets\\projets.info").expect("Erreur durant la lecture du fichier listant les projets du framework.");
    let mut contenu_fichier_sans_projet = String::new();
    println!(".info : {}", contenu_fichier.clone());
    if contenu_fichier.contains(&nom) {
        for ligne in std::fs::read_to_string("projets\\projets.info").unwrap().lines(){
            if !ligne.contains(&nom) {
                contenu_fichier_sans_projet = contenu_fichier_sans_projet + ligne;
            }
            else{
                let mut iteration = 0;
                let mut chemin_projet = String::new();
                for champ in ligne.split_whitespace(){
                    if iteration == 1{
                        chemin_projet = chemin_projet + champ;
                        supprimer_fichier_projet(chemin_projet.clone());
                    }
                    iteration += 1;
                }
            }
        }
        return contenu_fichier_sans_projet;
    }
    else{
        return String::new();
    }
}