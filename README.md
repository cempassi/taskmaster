# Taskmaster

## Vendredi 29/10

Commencer par la mise en place du serveur
Priorite : mise en place des data structures pour les differents elements du projet

    - Enum status de job (KILLED, RUNNING, STOPPED...)
    - Structure pour chaque job avec command, statut etc
    - Enum commande (Start, Stop...)

Il faudra voir aussi comment gerer la config meme du server avec une structure par exemple (avec comme elements le fichier de config et les jobs lances)
  -> savoir aussi dans quel type de structure stockes les jobs (une hashmap ou encore une liste vectorielle)


Sources liste signaux: https://www.tutorialspoint.com/unix/unix-signals-traps.htm