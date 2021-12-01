# rust-presentation

## Installer Rust

### Linux et MacOs
```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows
Installer Visual Studio avec tout les paquet c++ (on ne parle pas de Visual Studio Code attention).

Rendez vous sur cette page : https://www.rust-lang.org/fr/tools/install

Telecharger l'installeur et lancer le.

## Utiliser Rust
Choisisez votre IDE ou éditeur de text préférer.
Installer les plugin Rust
 - Intelij Idea : 
   - https://plugins.jetbrains.com/plugin/8182-rust
 - Visual Studio Code : 
   - https://marketplace.visualstudio.com/items?itemName=rust-lang.rust 
   - https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer

Affin de crée un projet, crée un fichier, allez dedans avec votre terminal et taper :
```bash
    cargo init
```

Vous devriers vous retrouver avec l'arborecente suivante
```folder
├── Cargo.lock
├── Cargo.toml
└── src
    └── main.rs
```

Cargo.lock est a ignorer
Cargo.toml contient les information de l'application du projet, sa version, les auteur,
les dépendance utiliser.

src contientdra votre future code, pour le moment il ne posséde que le fichier main.rs .

Pour lancher les différent exemple executer :
```bash
    cargo run --bin <noms du fichier>
```
n'hésiter pas a les modifier et tester avec le comportement des variables.