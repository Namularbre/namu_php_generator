use std::process::{self};
use std::io::{self};

pub fn supprimer_projet(racine : String){
    let mut confirmation = String::new();
    println!("Vous etes sur le point de supprimer un projet. Cette opération est irréversible. Taper \"OUI\" pour valider.");
    io::stdin().read_line(&mut confirmation).unwrap();
    if confirmation.trim() != "OUI"{
        println!("Opération annulée !");
        return;
    }

    let chemin_racine_projet_supprimer = racine.clone();
    let _output = if cfg!(target_os = "windows"){ 
                    std::process::Command::new("cmd")
                    .args(["/C", &format!("rmdir {} /S /Q", chemin_racine_projet_supprimer)])
                    .output()
                    .expect("Erreur durant la suppression du projet")
                }
                else{
                    panic!("Linux n'est pas prit en charge... Pour le momment :3");
                };
    println!("Projet supprimé.");
}