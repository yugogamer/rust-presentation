use std::rc::Rc;


struct UnStruct{
    nombre : Rc<i32>
}

fn main(){
    let reference = Rc::new(1);

    let premier = UnStruct{nombre: reference.clone()};
    let deuxieme = UnStruct{nombre: reference.clone()};

    // Pour modifier les valeurs il faut utiliser RefCell
    println!("{}", &*premier.nombre);
    println!("{}", &*deuxieme.nombre);
}

