
struct Rectangle{
    pub largeur : f32, // pub -> valeur publique accecible par n'importe quelle package
    pub hauteur : f32
}

//Permet d'ajouter des méthode
impl Rectangle {

    fn new(largeur : f32, hauteur : f32) -> Rectangle{ // La non présence d'un 'self' signifie que la fonction est globale, elle n'a pas besoin d'un objet pour être appeler
        // pas de ';' a la fin signifie qu'il s'agit d'un return, cependant le mot return peut tout de même être utiliser
        Rectangle{
            largeur : largeur,
            hauteur : hauteur
        }
    }

    // Le self posséde les même proprieter que si une variable était passer,
    // Dans le cas présent il est borrower et n'est pas mutable, on ne peut que lire les valeur
    fn get_air(&self) -> f32{
        self.hauteur * self.largeur
    }
}


fn main(){
    let rectangle = Rectangle::new(4.0, 2.0);
    let air = rectangle.get_air();
    println!("L'air pour le rectangle de largeur {} et de hauteur {} est {}", rectangle.largeur, rectangle.hauteur, air);
}

