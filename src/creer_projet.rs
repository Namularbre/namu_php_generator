use std::fs::create_dir_all;
use std::fs::File;
use std::fs::write;
use std::fs::read_to_string;

pub struct ArborescenceProjet{
    racine : String,
}

impl ArborescenceProjet{
    pub fn new(racine : String) -> Self{
        return Self{
            racine,
        }
    }

    fn avoir_racine(&self) -> String{
        return self.racine.clone();
    }

    pub fn creer_arborescence(&self) -> std::io::Result<()>{
        self.creer_dossier_arborescence_par_defaut().expect("Il y a eu une erreur durant la création des dossiers composant l'arborescence par défault");
        self.creer_fichier_par_defaut().expect("Il y a eu une erreur durant la création des fichiers par défaut du framework.");
        self.creer_fichier_routage().expect("Il y a eu une erreur durant la création des classes Route et Routeur.");
        self.creer_vue_entree().expect("Il y a eu une erreur durant la création de la vue d'entrée de l'application.");

        return Ok(());
    }

    fn creer_dossier_arborescence_par_defaut(&self) -> std::io::Result<()> {
        //Les chemins des dossiers
        let chemin_dossier_publique = self.avoir_racine() + "/publique";
        let chemin_dossier_vues = self.avoir_racine() + "/vues";
        let chemin_dossier_controleurs = self.avoir_racine() + "/controleurs";
        let chemin_dossier_modeles = self.avoir_racine() + "/modeles";
        let chemin_dossier_routage = self.avoir_racine() + "/routage";

        println!("Création de l'arborescence par défault sur {} ...", self.avoir_racine());
        //Création de l'arborescence du projet
        create_dir_all(self.avoir_racine()).expect("Il y a eu une erreur à la création du dossier racine du projet.");
        create_dir_all(chemin_dossier_publique).expect("Il y a eu une erreur durant la création du dossier publique.");
        create_dir_all(chemin_dossier_vues).expect("Il y a eu une erreur durant la création du dossier des vues.");
        create_dir_all(chemin_dossier_controleurs).expect("Il y a eu une erreur durant la création du dossier contenant les contrôleurs.");
        create_dir_all(chemin_dossier_modeles).expect("Il y a eu une erreur durant la création du dossier contenant les modèles.");
        create_dir_all(chemin_dossier_routage).expect("Il y a eu une erreur durant la création du dossier contenant les classes Routeur et Route.");
        
        println!("fini.");

        return Ok(());
    }

    fn creer_fichier_par_defaut(&self) -> std::io::Result<()>{
        //Le chemin des fichiers "par défaut" du framework.
        let chemin_fichier_index = self.avoir_racine() + "/index.php";
        let chemin_fichier_information = self.avoir_racine() + "/namu_php_framework.info";
        /*
            On créer index.php, le fichier qui contiendra les routes de l'application et fera appel au routeur ainsi que
            le fichier d'information, un fichier utiliser par le framework pour savoir dans quel fichier il se trouve, etc.
        */
        println!("Création des fichiers de base...");

        File::create(chemin_fichier_index).expect("Il y a eu une erreur durant la création du fichier d'entrée de l'application.");
        File::create(chemin_fichier_information).expect("Il y a eu une erreur durant la création du fichier contenant les données nécessaires au fonctionnement du framework");
        
        println!("fini.");

        return Ok(());
    }

    fn creer_fichier_routage(&self) -> std::io::Result<()>{
        let chemin_fichier_routeur = self.avoir_racine() + "/routage/Routeur.php";
        let chemin_fichier_route = self.avoir_racine() + "/routage/Route.php";

        let chemin_fichier_route_clone = chemin_fichier_route.clone();
        let chemin_fichier_routeur_clone = chemin_fichier_routeur.clone();
        //On remplis le dossier "routage" avec une classe pour les routes et une pour le routeur.
        println!("Création des fichiers utiliser pour le routage...");
        File::create(chemin_fichier_routeur).expect("Il y a eu une erreur durant la création du fichier contenant la classe du routeur");
        File::create(chemin_fichier_route).expect("Il y a eu une erreur durant la création du fichier contenant la classe des routes");
        
        let contenu_fichier_routeur = read_to_string("samples\\routeur.php.sample").expect("Il y a eu une erreur durant la lecture du contenu du fichier sample du routeur");
        let contenu_fichier_route = read_to_string("samples\\route.php.sample").expect("Il y  a eu une erreur durant la lecture du contenu du fichier sample de la classe des routes");

        write(chemin_fichier_route_clone, contenu_fichier_route).expect("Il y a eu une erreur durant le remplissage du fichier de la classe des routes");
        write(chemin_fichier_routeur_clone, contenu_fichier_routeur).expect("Il y a eu une erreur durant le remplisage du fichier de la classe du routeur");

        println!("fini.");

        return Ok(());
    }

    fn creer_vue_entree(&self) -> std::io::Result<()>{
        let chemin_fichier_vue_entree = self.avoir_racine() + "/vues/accueil.php";
        let chemin_fichier_vue_entree_clone = chemin_fichier_vue_entree.clone();
        //On creer ensuite une vues sur laquelle on sera rediriger, car index.php n'affiche rien.
        println!("Création de la vue d'entrée");

        File::create(chemin_fichier_vue_entree).expect("Il y a eu une erreur durant la création de la vue d'entrée de l'application (accueil.php)");

        let contenu_vue_entree = read_to_string("samples\\accueil.php.sample").expect("Il y a eu une erreur durant la lecture du sample du code d'accueil.php");

        write(chemin_fichier_vue_entree_clone, contenu_vue_entree).expect("Il y a eu une erreur durant l'écriture du contenu de la vue d'entrée.");

        println!("fini.");
        return Ok(());
    }
}