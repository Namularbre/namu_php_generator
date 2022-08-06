use structopt::StructOpt;

//import des modules
mod creer_projet;

#[derive(StructOpt)]
struct Options{
    action : String,
    parametres : Vec<String>,
}

fn main() {
    let options = Options::from_args();

    if options.action == "nouveau" || options.action == "n" {
        let arborescence = creer_projet::ArborescenceProjet::new(options.parametres[0].clone());
        arborescence.creer_arborescence().expect("Il y a eu une erreur durant la création du nouveau projet.");
        println!("Votre projet est prêt ! Il vous attends :3");
    }
}
