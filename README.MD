## Files structure

    wget/
        │
        ├── src/
        │   ├── main.rs                // Fichier principal du programme
        │   ├── lib.rs                 // Fichier de la bibliothèque (pour séparer la logique)
        │   ├── downloader.rs          // Module pour gérer les téléchargements de fichiers
        │   ├── progress.rs            // Module pour gérer l'affichage de la progression
        │   ├── cli.rs                 // Module pour gérer les options en ligne de commande
        │   ├── rate_limiter.rs        // Module pour limiter la vitesse de téléchargement
        │   ├── mirror.rs              // Module pour implémenter le mirroring de site web
        │   └── utils.rs               // Module pour des fonctions utilitaires (gestion des fichiers, etc.)
        │
        ├── tests/                     // Tests pour le projet
        │   ├── integration_test.rs    // Tests d'intégration (pour tester les fonctionnalités globales)
        │
        ├── Cargo.toml                 // Fichier de configuration des dépendances et du projet
        ├── Cargo.lock                 // Fichier de verrouillage des dépendances
        ├── README.md                  // Documentation du projet
        └── .gitignore                 // Fichier pour ignorer certains fichiers dans Git


## usage

    cargo run -- --option "url"