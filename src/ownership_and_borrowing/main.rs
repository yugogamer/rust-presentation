

fn main() {
    let pomme = String::from("pomme");

    prête_ma_pomme(&pomme);
    //La pomme est encore utilisable dans cette fonction

    prend_ma_pomme(pomme);
    //Pomme n'existe plus dans cette fonction
}

fn prête_ma_pomme(pomme : &String){
    println!("On me préte la {}", pomme)
}// La variable pomme est rendu a se moment

fn prend_ma_pomme(pomme : String){
    println!("Je vais manger ma {}", pomme)
}// La variable pomme est détruite ici

