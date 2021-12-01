

fn mutable(){
    let mut mutable_string = "a string".to_string();

    mutable_string.push_str(" an other string");

    println!("{}", mutable_string);
}

fn immutable(){
    let imutable_string = "a string".to_string();

    //string.push_str(" an other string") 
    // Retirer la ligne précédente en commentaire et essayer de lancer avec
    // cargo run --bin mutabiliter

    println!("{}", imutable_string);
}

fn main(){
    immutable();
    mutable();
}