## Étape 1 : Initialisation du projet et parsing des arguments

    Tâche :
        Initialiser le projet Rust (cargo init).
        Configurer le parsing des arguments en utilisant clap.
        Arguments à gérer : URL, -O (nom du fichier), -P (répertoire), --rate-limit, etc.
    Test :
        Lancer le programme avec différents arguments pour s'assurer que les arguments sont correctement parsés et affichés.

## Étape 2 : Téléchargement de base

    Tâche :
        Implémenter un téléchargement simple avec reqwest à partir d'une URL donnée.
        Afficher l'heure de début, l'état HTTP et la taille du fichier.
    Test :
        Lancer le programme avec une URL pour vérifier que le fichier est téléchargé et que les informations sont correctement affichées.

## Étape 3 : Sauvegarde sous un autre nom et dans un répertoire spécifique

    Tâche :
        Gérer l'option -O pour permettre de spécifier un autre nom pour le fichier.
        Gérer l'option -P pour permettre de spécifier un répertoire de sauvegarde.
    Test :
        Tester avec -O et -P pour vérifier que le fichier est enregistré sous le bon nom et dans le bon répertoire.

## Étape 4 : Affichage de la barre de progression

    Tâche :
        Implémenter une barre de progression qui affiche le pourcentage téléchargé, la taille téléchargée et le temps restant.
    Test :
        Lancer le téléchargement d'un gros fichier pour vérifier que la barre de progression fonctionne.

## Étape 5 : Limitation de la vitesse de téléchargement (--rate-limit)

    Tâche :
        Implémenter la limitation de vitesse avec --rate-limit, en prenant en charge les unités k et M.
    Test :
        Tester avec différentes valeurs de --rate-limit et vérifier que la vitesse est respectée.

## Étape 6 : Téléchargement en arrière-plan (-B)

    Tâche :
        Implémenter le téléchargement en arrière-plan, avec redirection de la sortie vers un fichier log (wget-log).
    Test :
        Lancer le programme avec -B et vérifier que le fichier est téléchargé en arrière-plan et que le log est correct.

## Étape 7 : Téléchargement de plusieurs fichiers en parallèle (-i)

    Tâche :
        Gérer l'option -i pour lire une liste d'URLs depuis un fichier et télécharger les fichiers de manière asynchrone.
    Test :
        Tester avec un fichier contenant plusieurs URLs et vérifier que les téléchargements se font en parallèle.

## Étape 8 : Mirroring d'un site web (--mirror)

    Tâche :
        Implémenter la fonctionnalité de mirroring, en récupérant tous les fichiers nécessaires (HTML, CSS, images) et en les sauvegardant dans un répertoire spécifique.
    Test :
        Lancer le programme avec --mirror pour un site et vérifier que le site est correctement téléchargé.

## Étape 9 : Exclusion de types de fichiers et répertoires (-R, -X)

    Tâche :
        Gérer les options -R et -X pour exclure certains fichiers et répertoires lors du mirroring.
    Test :
        Tester avec des fichiers à exclure et vérifier que les fichiers exclus ne sont pas téléchargés.

## Étape 10 : Conversion des liens pour consultation hors ligne (--convert-links)

    Tâche :
        Implémenter la conversion des liens HTML pour qu'ils pointent vers les fichiers locaux.
    Test :
        Lancer le programme avec --mirror --convert-links et vérifier que les liens sont modifiés pour un usage hors ligne.