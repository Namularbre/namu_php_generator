pub fn supprimer_projet(nom : String){
    enlever_la_ligne_du_projet_dans_fichier_information(nom);
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

fn enlever_la_ligne_du_projet_dans_fichier_information(nom : String){
    let contenu_fichier = std::fs::read_to_string("projets\\projets.info").expect("Erreur durant la lecture du fichier listant les projets du framework.");
    let mut contenu_fichier_sans_projet = String::new();
    println!(".info : {}", contenu_fichier.clone());
    if contenu_fichier.contains(&nom) {
        println!("le projet existe !");
        for ligne in std::fs::read_to_string("projets\\projets.info").unwrap().lines(){
            println!("ligne:{}", ligne.clone());
            if !ligne.contains(&nom) {
                contenu_fichier_sans_projet = contenu_fichier_sans_projet + ligne;
                println!("cfsp :{}", contenu_fichier_sans_projet.clone());
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
        remplacer_fichier_information(contenu_fichier_sans_projet.clone());
    }
    else{
        panic!("Le projet est introuvable");
    }
}