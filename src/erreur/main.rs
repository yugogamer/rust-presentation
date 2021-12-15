
fn is_true(x : bool) -> Result<String, String>{
    match x {
        true => return Ok("Error".to_owned()),
        false => return Err("No error".to_owned()),
    }
}


fn main(){ 

    let erreur = is_true(false);

    match erreur {
        Ok(value) => println!("{}", value),
        Err(err) => println!("{}", err),
    }

    let pas_une_erreur = is_true(true);

    match &pas_une_erreur {
        Ok(value) => println!("{}", value),
        Err(err) => println!("{}", err),
    }

    if pas_une_erreur.is_err() {
        
    }

    if pas_une_erreur.is_ok() {
        
    }

    let _value = pas_une_erreur.unwrap();
}