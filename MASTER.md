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
- Recherche locale de documents a definir : retrouver rapidement des fichiers
  utiles sans remplacer tout l'Explorateur Windows.
- Fenetre terminal integree pour PowerShell et CMD.
- Actions controlees sur un logiciel : ouvrir l'emplacement dans
  l'Explorateur, lancer l'application quand le chemin est connu, modifier les
  metadonnees locales, masquer ou retirer une entree locale, et proposer une
  desinstallation via les commandes Windows avec confirmation explicite.
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
  et terminal PowerShell/CMD integre. Le chantier local non encore commite
  ajoute une premiere version des actions logiciel (lancer, ouvrir,
  Explorateur, copie de chemin, desinstallation avec confirmation), des
  metadonnees locales persistantes (categorie personnalisee, note, favori,
  masque), l'historique terminal local, la detection de commandes sensibles, la
  verification `winget upgrade` sur action explicite, l'extraction/cache
  d'icones Windows et les styles CSS associes. Apres validation utilisateur
  partielle, le lancement Windows a ete corrige pour laisser Windows gerer
  l'elevation/UAC, les commandes PowerShell rapides ont ete durcies contre les
  collisions de noms comme `Get-Process`, et les logiciels masques sont
  retrouvables via un filtre visible `Masques`. Apres nouveau retour
  utilisateur, les icones base64 sont autorisees par la CSP Tauri (`data:`), le
  fallback d'icone est rendu plus visible. La tentative de largeur minimale a
  720 px a ete jugee trop instable pour cette interface desktop dense ; la
  largeur minimale Tauri est revenue a 1040 px avec des breakpoints CSS
  simplifies. Le dernier correctif local stabilise specifiquement les filtres
  du catalogue : recherche sur une ligne complete, categorie et affichage sur
  une ligne controlee, boutons d'affichage flexibles, et bande `Toutes` /
  categories en defilement horizontal stable.
- Prod alignee : non applicable, application non distribuee ni installee.
  GitHub `origin/main` est aligne avec les changements locaux via les commits
  `265d05e` et `9744597`. Une prerelease GitHub `v0.1.0` existe avec les
  installateurs Windows NSIS et MSI :
  `https://github.com/kinowill/Commode/releases/tag/v0.1.0`.
- Validation reelle effectuee : typecheck frontend, formatage Rust, check Rust,
  tests Rust, audit npm, build frontend, build Tauri installable et push GitHub
  effectues le 2026-05-06. Le 2026-05-06, apres le chantier local actions/CSS,
  `npm run typecheck`, `npm run build`, `cargo fmt --manifest-path
  src-tauri\Cargo.toml -- --check`, `cargo test --manifest-path
  src-tauri\Cargo.toml`, `git diff --check`, `npm audit --audit-level=high` et
  `npm run tauri build` reussissent. Apres correction elevation/masques/terminal,
  les memes validations reussissent avec 24 tests Rust. Apres correction
  icones/CSP et stabilisation responsive desktop, `npm run typecheck`, `npm run
  build`, `cargo fmt --manifest-path src-tauri\Cargo.toml -- --check`, `cargo test
  --manifest-path src-tauri\Cargo.toml`, `git diff --check`, `npm audit
  --audit-level=high` et `npm run tauri build` reussissent. Ces validations
  ont ete reexecutees apres correction des filtres `Categorie` / `Toutes` /
  `Affichage`. Commits et push GitHub effectues ensuite. Retest manuel complet
  confirme par l'utilisateur le 2026-05-07 ("ca fonctionne"). Prerelease
  `v0.1.0` creee et verifiee sur GitHub le 2026-05-07.

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
- Perimetre d'une eventuelle recherche de documents locale : dossiers inclus,
  types de fichiers, index ou scan a la demande, performances et confidentialite.
- Niveau d'automatisation autorise pour les mises a jour.
- Garde-fous avances du terminal integre PowerShell/CMD.
- UX definitive de la verification et de la mise a jour des logiciels apres la
  premiere verification `winget`.
- Extraction/cache des vraies icones Windows a surveiller sur davantage de
  logiciels au fil des tests.
- Responsive desktop valide manuellement pour le prototype, a surveiller lors
  des futurs ajouts d'interface.
- UX definitive des actions logiciel apres la premiere implementation locale :
  lancer, ouvrir dans l'Explorateur, modifier les metadonnees locales, masquer,
  retirer de l'affichage, ou desinstaller. Les actions destructrices ne doivent
  jamais etre lancees sans confirmation claire.

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
