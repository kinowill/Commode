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

Stack decidee le 2026-05-06 :

- Tauri 2 pour l'application desktop Windows installable.
- React + TypeScript + Vite pour l'interface.
- Rust pour les commandes locales, l'inventaire logiciel, le terminal integre
  et les integrations systeme.
- Stockage local leger a definir pour les categories, metadonnees et
  preferences.

## Structure actuelle

Le projet contient maintenant un prototype Tauri 2 + React + Rust.

Fichiers et dossiers principaux :

- `MASTER.md` : reference principale du projet.
- `ROADMAP.md` : objectif courant et prochaines etapes.
- `VALIDATION.md` : journal des validations reelles.
- `src/` : interface React.
- `src-tauri/` : application Tauri/Rust, configuration desktop et commandes
  locales.
- `package.json` / `package-lock.json` : dependances frontend.
- `src-tauri/Cargo.toml` / `src-tauri/Cargo.lock` : dependances Rust.

## Etat courant

- Repo modifie : prototype Tauri/React/Rust cree et pousse sur `origin/main`
  avec inventaire Windows via registre, recherche/categories, fiches detaillees
  et terminal PowerShell/CMD integre. Corrections UI appliquees : pictogrammes
  embarques, suppression des faux liens, terminal a la demande, wording mises a
  jour nettoye pour le mode non installe.
- Prod alignee : non applicable, application non distribuee ni installee.
- Validation reelle effectuee : typecheck frontend, formatage Rust, check Rust,
  tests Rust, audit npm, build frontend, build Tauri installable et push GitHub
  effectues le 2026-05-06. Test manuel dans la fenetre Tauri non encore
  effectue.

## Sources de verite connues

1. `MASTER.md`
2. `ROADMAP.md`
3. `VALIDATION.md`
4. Code de l'application
5. Depot GitHub cible : `https://github.com/kinowill/Commode`
   (remote `origin` relie localement, depot distant verifie en lecture)
6. Instructions Codex actives fournies dans la session

## Decisions ouvertes

- Perimetre exact de la detection des logiciels installes.
- Sources de donnees pour les informations detaillees.
- Methode de verification des versions disponibles.
- Methode de mise a jour des logiciels.
- Niveau d'automatisation autorise pour les mises a jour.
- Garde-fous avances du terminal integre PowerShell/CMD.
- Extraction de vraies icones logiciel depuis les executables Windows.
- UX definitive de la verification et de la mise a jour des logiciels.

## Decisions prises

- L'application doit etre une vraie application Windows installable, pas une
  simple application web locale ouverte dans un navigateur.
- L'application suit une logique privacy-first stricte : inventaire logiciel
  100% local, aucune telemetrie, aucune synchronisation cloud, et acces reseau
  uniquement sur action explicite de l'utilisateur pour verifier les mises a
  jour.
- Stack retenue : Tauri 2 + React + TypeScript + Vite + Rust.

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
