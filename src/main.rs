use std::io;
use rand::Rng;

fn main() {
    let nombre_secret = rand::thread_rng().gen_range(1..100);

    let mut number_text = String::new();
    println!("entrez un nombre");
    io::stdin()
        .read_line(&mut number_text)
        .expect("Erreur lors de la lecture");
    
    let number_user = number_text
        .trim()
        .parse()
        .expect("valeur incorrect");
   
}
