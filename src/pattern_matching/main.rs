
struct EntrerClassique{
    pub user_id : String 
}

struct EntrerAdmin{
    pub clé_securité : u8
}

enum Connection{
    User(EntrerClassique),
    Admin(EntrerAdmin)
}

fn main(){
    let mut queue_connection: Vec<Connection> = Vec::new();
    let admin = EntrerAdmin{ clé_securité : 42};
    let user = EntrerClassique{user_id : "5754adza".to_owned()};

    queue_connection.push(Connection::Admin(admin));
    queue_connection.push(Connection::User(user));

    for value in &queue_connection{
        match value {
            Connection::User(user) => println!("Un utilisateur se connecte, sont id est {}", user.user_id),
            Connection::Admin(admin) => println!("Un admin se connecte, sa clé est {}", admin.clé_securité),
        }
    }


    for value in queue_connection{
        match value {
            Connection::User(_user) => {
                //fonction classique, tout est autoriser
            },
            Connection::Admin(admin) => {
                let computed_key = admin.clé_securité + 55;
                println!("Un admin se connecte, sa clé calculer est {}", &computed_key);

                match computed_key {
                    1 => println!("pourquoi"),
                    42 | 97 => println!("super admin"), // les pipe servent a indiquer plusieur valeur
                    _ => println!("Une clé classique") // quand aucun patern n'est trouver, pas forcément obligatoire mais conseiller
                }
            },
        }
    }

}