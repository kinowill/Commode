# Commode - Document maitre

## But du projet

Commode est une application locale Windows destinee a repertorier les logiciels
installes sur le PC comme un app store personnel.

L'objectif est de donner une vue claire des applications presentes, organisees
par categorie, avec des informations utiles, une recherche propre, et a terme
des fonctions de verification et de mise a jour.

## Fonctionnalites visees

- Catalogue local des logiciels installes.
- Classement par categories.
- Fiches detaillees par logiciel.
- Recherche propre et rapide.
- Verification des versions disponibles.
- Aide a la mise a jour des logiciels.
- Fenetre terminal integree pour PowerShell et CMD.
- Interface soignee avec de bonnes finitions.

## Stack

Stack non decidee a ce stade.

Options probables a evaluer :

- Application desktop Windows installable avec interface locale.
- Backend local pour lire les informations systeme.
- Stockage local leger pour les categories, metadonnees et preferences.

## Structure actuelle

Le projet vient d'etre initialise comme dossier vide.

Fichiers de verite crees :

- `MASTER.md` : reference principale du projet.
- `ROADMAP.md` : objectif courant et prochaines etapes.
- `VALIDATION.md` : journal des validations reelles.

## Etat courant

- Repo modifie : documentation de cadrage initiale creee.
- Prod alignee : non applicable, aucune application deployee.
- Validation reelle effectuee : presence et contenu des fichiers de cadrage
  verifies localement le 2026-05-06.

## Sources de verite connues

1. `MASTER.md`
2. `ROADMAP.md`
3. `VALIDATION.md`
4. Code futur de l'application
5. Depot GitHub cible : `https://github.com/kinowill/Commode`
   (remote `origin` relie localement, depot distant verifie en lecture)
6. Instructions Codex actives fournies dans la session

## Decisions ouvertes

- Stack technique de l'application.
- Perimetre exact de la detection des logiciels installes.
- Sources de donnees pour les informations detaillees.
- Methode de verification des versions disponibles.
- Methode de mise a jour des logiciels.
- Niveau d'automatisation autorise pour les mises a jour.
- Perimetre et garde-fous du terminal integre PowerShell/CMD.

## Decisions prises

- L'application doit etre une vraie application Windows installable, pas une
  simple application web locale ouverte dans un navigateur.
- L'application suit une logique privacy-first stricte : inventaire logiciel
  100% local, aucune telemetrie, aucune synchronisation cloud, et acces reseau
  uniquement sur action explicite de l'utilisateur pour verifier les mises a
  jour.

## Securite et confidentialite

- Aucun secret, token ou identifiant ne doit etre inscrit dans la documentation
  ou dans les commandes partagees.
- Les actions GitHub doivent rester explicites : lecture, ajout de remote,
  commit ou push ne sont pas equivalents.
- Les commits et push GitHub sont autorises, a condition de proteger le depot
  avec un `.gitignore` prudent et de verifier le contenu stage avant push.
- Les donnees personnelles locales, caches, builds, journaux, exports,
  captures, bases locales et fichiers de configuration sensibles ne doivent pas
  etre versionnes.
- L'identite Git locale du repo utilise un email GitHub noreply pour eviter de
  publier une adresse personnelle dans les commits.
