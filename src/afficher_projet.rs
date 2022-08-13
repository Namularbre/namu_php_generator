use std::fs::read_to_string;

pub fn afficher_projet(){
    let contenu_fichier_information = read_to_string("projets/projets.info").unwrap();

    for ligne in contenu_fichier_information.lines(){
        let nom_projet = ligne.split_whitespace().next().unwrap();
        println!("{}", nom_projet);
    }
}