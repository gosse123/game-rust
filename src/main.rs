use std::io;
use rand::Rng;
use std::cmp::Ordering;



fn main() {
    let secret_number = rand::thread_rng().gen_range(1..100);

    let mut suppose = String::new();

    loop {
        println!("entrez un nombre");
        io::stdin()
            .read_line(&mut suppose)
            .expect("erreur de lecture");

        let suppose : u32 = suppose
            .trim()
            .parse()
            .expect("entrez un nombre");

        match suppose.cmp(&secret_number) {
            Ordering::Less=>println!("moins"),
            Ordering::Greater=>println!("plus"),
            Ordering::Equal=>println!("bravo")
        }
    }
    
   
}
