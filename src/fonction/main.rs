


fn une_function_carrer(nombre : u128) -> u128 {
    return nombre * nombre;
}

fn une_fonction_qui_affiche(){
    println!("j'affiche");
}


fn main(){
    let x = une_function_carrer(50);
    println!("{}", &x);
    une_fonction_qui_affiche();


    let lambda = |i: u128| -> u128 { i + 1 };
    println!("{}", lambda(x));
}

