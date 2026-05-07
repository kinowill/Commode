# Commode - Journal de validation

## 2026-05-06 - Initialisation documentaire

Etat :

- Repo modifie : `MASTER.md`, `ROADMAP.md` et `VALIDATION.md` crees.
- Prod alignee : non applicable.
- Validation reelle effectuee : presence et contenu de `MASTER.md`,
  `ROADMAP.md` et `VALIDATION.md` verifies localement le 2026-05-06.

Notes :

- Aucun code applicatif n'existe encore.
- Aucune stack technique n'est encore decidee.

## 2026-05-06 - Ajout d'exigence produit

Etat :

- Repo modifie : besoin de fenetre terminal integree PowerShell/CMD et exigence
  de finition UI ajoutes a `MASTER.md` et `ROADMAP.md`.
- Prod alignee : non applicable.
- Validation reelle effectuee : modification documentaire verifiee localement
  le 2026-05-06.

Notes :

- Le terminal integre est une zone sensible : il faudra definir les garde-fous
  avant toute implementation.

## 2026-05-06 - Decision application Windows installable

Etat :

- Repo modifie : decision "vraie application Windows installable" ajoutee a
  `MASTER.md` et `ROADMAP.md`.
- Prod alignee : non applicable.
- Validation reelle effectuee : modification documentaire verifiee localement
  le 2026-05-06.

Notes :

- Cette decision exclut une simple application web locale comme produit final.

## 2026-05-06 - Depot GitHub cible fourni

Etat :

- Repo modifie : URL GitHub cible ajoutee a `MASTER.md` et `ROADMAP.md`.
- Prod alignee : non applicable.
- Validation reelle effectuee : depot distant verifie en lecture le
  2026-05-06.

Notes :

- URL fournie : `https://github.com/kinowill/Commode`.
- Premiere tentative de lecture distante en acces Git echouee dans le sandbox.
- Verification distante reussie hors sandbox avec `git ls-remote`, sans sortie,
  ce qui correspond a un depot vide accessible.
- Remote `origin` ajoute localement ensuite.
- Aucun push, aucun commit, aucun secret manipule a ce stade.

## 2026-05-06 - Autorisation commit/push et privacy-first

Etat :

- Repo modifie : regles privacy-first, autorisation commit/push avec garde-fous,
  et `.gitignore` prudent ajoutes.
- Prod alignee : non applicable.
- Validation reelle effectuee : contenu verifie avant initialisation git et
  premier commit.

Notes :

- Politique validee : inventaire 100% local, aucune telemetrie, aucune synchro
  cloud, acces reseau seulement sur action explicite de verification des mises
  a jour.
- Les secrets, donnees personnelles, caches, builds, logs, exports, captures et
  bases locales sont exclus par defaut.

## 2026-05-06 - Initialisation git locale

Etat :

- Repo modifie : depot git initialise en branche `main`, remote `origin` relie
  a `https://github.com/kinowill/Commode.git`, identite Git locale configuree
  avec un email GitHub noreply.
- Prod alignee : non applicable.
- Validation reelle effectuee : `git status` et `git remote -v` verifies
  localement.

Notes :

- Aucun commit n'existe encore au moment de cette entree.
- L'email Git local configure est un noreply afin d'eviter la publication d'une
  adresse personnelle.

## 2026-05-06 - Premier commit et push GitHub

Etat :

- Repo modifie : premier commit cree et pousse sur `origin/main`.
- Prod alignee : non applicable, aucune application n'existe encore.
- Validation reelle effectuee : sortie du script de commit/push confirmee et
  branche `main` configuree pour suivre `origin/main`.

Notes :

- Commit initial : `f5f780d` (`docs: initialize project protocol`).
- Fichiers pousses : `.gitignore`, `MASTER.md`, `ROADMAP.md`, `VALIDATION.md`.
- Aucun code applicatif, secret, cache, build, export ou donnee personnelle n'a
  ete pousse.

## 2026-05-06 - Decision de stack applicative

Etat :

- Repo modifie : stack Tauri 2 + React + TypeScript + Vite + Rust inscrite dans
  `MASTER.md` et `ROADMAP.md`.
- Prod alignee : non applicable, aucune application n'existe encore.
- Validation reelle effectuee : decision utilisateur confirmee dans la session.

Notes :

- Sources consultees : documentation officielle Tauri 2 pour la creation de
  projet, Vite, les commandes Rust appelees depuis le frontend, et les
  installateurs Windows.
- Les prerequis locaux restent a verifier avant generation du squelette.

## 2026-05-06 - Verification des prerequis locaux

Etat :

- Repo modifie : non, verification environnement uniquement.
- Prod alignee : non applicable.
- Validation reelle effectuee : commandes locales de version executees.

Notes :

- Node : `v22.22.2`.
- npm : `11.12.0`.
- rustc : `1.94.1`.
- cargo : `1.94.1`.
- Les prerequis de base pour Tauri + React + Rust sont presents.

## 2026-05-06 - Prototype Tauri local

Etat :

- Repo modifie : squelette Tauri 2 + React + TypeScript + Vite remplace par une
  premiere application Commode.
- Prod alignee : non applicable, application non distribuee ni installee.
- Validation reelle effectuee : validations automatiques et builds locaux
  effectues.

Notes :

- Interface ajoutee : tableau de bord, recherche, categories, liste logiciels,
  fiche detaillee, panneau terminal.
- Backend Rust ajoute : scan des logiciels Windows via registre Uninstall
  HKLM/HKCU, categorisation simple, commande terminal PowerShell/CMD explicite
  avec timeout de 30 secondes.
- Commandes validees :
  - `npm run typecheck`
  - `cargo fmt --manifest-path src-tauri\Cargo.toml -- --check`
  - `cargo check --manifest-path src-tauri\Cargo.toml`
  - `cargo test --manifest-path src-tauri\Cargo.toml`
  - `npm audit --audit-level=high`
  - `npm run build`
  - `npm run tauri build`
- Tests Rust ajoutes : 5 tests unitaires sur la categorisation et le formatage
  des dates registre.
- `npm run build` a d'abord echoue dans le sandbox sur `spawn EPERM` pour le
  binaire esbuild, puis a reussi hors sandbox sans modification de code.
- `npm run tauri build` a genere localement :
  - `src-tauri\target\release\commode.exe`
  - `src-tauri\target\release\bundle\nsis\Commode_0.1.0_x64-setup.exe`
  - `src-tauri\target\release\bundle\msi\Commode_0.1.0_x64_en-US.msi`
- Les artefacts `dist`, `node_modules`, `src-tauri\target` et les installateurs
  sont ignores par Git et ne doivent pas etre pousses.
- Controle anti-leak local effectue avant commit : aucun token, cle privee,
  mot de passe ou secret detecte dans les fichiers non ignores.
- Test manuel dans une fenetre Tauri : non effectue a ce stade.

## 2026-05-06 - Push du prototype applicatif

Etat :

- Repo modifie : prototype applicatif commite et pousse sur `origin/main`.
- Prod alignee : non applicable, application non distribuee ni installee.
- Validation reelle effectuee : sortie du script de commit/push confirmee.

Notes :

- Commit pousse : `49519f5` (`feat: scaffold commode desktop prototype`).
- Les artefacts de build et installateurs sont restes locaux et ignores.
- Cette entree documentaire aligne la trace projet avec le push du prototype.

## 2026-05-06 - Correction images en mode non installe

Etat :

- Repo modifie : pictogrammes SVG inline par categorie ajoutes a l'interface.
- Prod alignee : non applicable.
- Validation reelle effectuee : `npm run typecheck`, `npm run build` et
  `npm run tauri build` reussis le 2026-05-06.

Notes :

- Cause constatee : le prototype n'affichait pas de vraies images logiciel,
  seulement des badges texte. En mode non installe, il ne fallait pas pointer
  vers des fichiers externes ou installes ailleurs.
- Correction choisie : pictogrammes embarques dans le bundle React, sans acces
  disque et sans reseau.
- Executable portable regenere :
  `src-tauri\target\release\commode.exe`.
- Prochaine evolution : extraction de vraies icones depuis les executables
  Windows et cache local controle.

## 2026-05-06 - Nettoyage UX mono-page

Etat :

- Repo modifie : suppression des faux liens de navigation, terminal deplace en
  panneau plein largeur a la demande, wording mises a jour simplifie.
- Prod alignee : non applicable.
- Validation reelle effectuee : `npm run typecheck`, `npm run build` et
  `npm run tauri build` reussis le 2026-05-06.

Notes :

- Cause constatee : les liens vers des sections etaient inutiles dans une seule
  page, le terminal occupait une colonne permanente sans valeur continue, et le
  message de mises a jour exposait une phrase technique non utile.
- Correction choisie : etat lateral statique, bouton d'ouverture du terminal,
  terminal plein largeur, et statut visible "Mises a jour non configurees".
- Premiere tentative de `npm run tauri build` echouee car Windows refusait de
  remplacer `src-tauri\target\release\commode.exe`, probablement verrouille par
  l'app lancee en mode portable. Aucun processus `commode` actif n'a ensuite
  ete detecte, puis la relance a reussi.
- Executable portable regenere :
  `src-tauri\target\release\commode.exe`.

## 2026-05-06 - Retour filtre Toutes et actions logiciel notees

Etat :

- Repo modifie : bouton de filtre "Toutes" ajoute dans la barre de categories,
  categorie active conservee dans les filtres visibles, et demande d'actions
  logiciel inscrite dans `MASTER.md` et `ROADMAP.md`.
- Prod alignee : non applicable.
- Validation reelle effectuee : `npm run typecheck`, `git diff --check`,
  `npm run build` et `npm run tauri build` reussis le 2026-05-06.

Notes :

- Le retour au catalogue complet ne depend plus de la liste deroulante
  categorie.
- Les actions logiciel demandees sont notees comme chantier controle :
  ouvrir dans l'Explorateur, lancer l'application, modifier les metadonnees
  locales, masquer ou retirer de l'affichage, et desinstaller uniquement avec
  confirmation explicite.
- Executable portable regenere :
  `src-tauri\target\release\commode.exe`.

## 2026-05-06 - Chantier local actions logiciel, CSS et validations

Etat :

- Repo modifie : changements locaux non encore commites ajoutant actions
  logiciel, metadonnees locales persistantes, historique terminal, premiers
  garde-fous de commandes sensibles, verification `winget upgrade`, extraction
  et cache d'icones Windows, et CSS associe. `cargo fmt` a ete applique apres
  un premier echec de controle de formatage.
- Prod alignee : non applicable, application non distribuee ni installee.
  GitHub `origin/main` n'est pas encore aligne avec ces changements locaux.
- Validation reelle effectuee : validations automatiques et build installable
  reussis le 2026-05-06.

Commandes validees :

- `npm run typecheck`
- `npm run build`
- `cargo fmt --manifest-path src-tauri\Cargo.toml -- --check`
- `cargo test --manifest-path src-tauri\Cargo.toml` : 21 tests reussis.
- `git diff --check`
- `npm audit --audit-level=high` : 0 vulnerabilite haute detectee.
- `npm run tauri build`

Artefacts regeneres :

- `src-tauri\target\release\commode.exe`
- `src-tauri\target\release\bundle\nsis\Commode_0.1.0_x64-setup.exe`
- `src-tauri\target\release\bundle\msi\Commode_0.1.0_x64_en-US.msi`

Notes :

- Test manuel dans une fenetre Tauri : non effectue a ce stade.
- Les fichiers Rust ajoutes doivent etre inclus dans le prochain commit, sinon
  `src-tauri/src/lib.rs` referencera des modules absents dans le depot distant.

## 2026-05-06 - Corrections apres validation utilisateur partielle

Etat :

- Repo modifie : correction du lancement Windows pour laisser ShellExecute
  gerer les applications qui demandent une elevation/UAC, durcissement du
  terminal PowerShell avec commande encodee UTF-16 et import des modules
  standards, commandes rapides PowerShell qualifiees, filtre visible `Visibles
  / Tous / Masques`, bascule automatique vers les logiciels masques apres
  masquage, et ajout de l'idee produit "recherche locale de documents" dans la
  roadmap.
- Prod alignee : non applicable, application non distribuee ni installee.
  GitHub `origin/main` n'est pas encore aligne avec ces changements locaux.
- Validation reelle effectuee : validations automatiques et build installable
  reussis le 2026-05-06 apres correction.

Cause constatee :

- `L'operation demandee necessite une elevation (os error 740)` venait du
  lancement direct par `CreateProcess` de certains executables Windows.
- `Top processus` echouait parce qu'un fichier local
  `C:\Windows\System32\Get-Process` pouvait etre resolu avant le cmdlet
  PowerShell attendu quand les modules standards n'etaient pas importes.
- Le masquage fonctionnait techniquement, mais l'interface n'expliquait pas
  clairement ou retrouver les logiciels masques.

Commandes validees :

- `npm run typecheck`
- `npm run build`
- `cargo fmt --manifest-path src-tauri\Cargo.toml -- --check`
- `cargo test --manifest-path src-tauri\Cargo.toml` : 24 tests reussis.
- `git diff --check`
- `npm audit --audit-level=high` : 0 vulnerabilite haute detectee.
- `npm run tauri build`

Notes :

- Retest manuel dans une fenetre Tauri : a refaire par l'utilisateur sur les
  points concernes, notamment `Top processus`, masquage/demasquage, lancement
  d'un logiciel qui demande l'elevation, et annulation d'une confirmation UAC.

## 2026-05-06 - Corrections icones et responsive

Etat :

- Repo modifie : la CSP Tauri autorise maintenant `data:` pour afficher les
  icones PNG base64 extraites depuis Windows, le fallback d'icone est plus
  visible. La tentative de largeur minimale Tauri a 720 px a ensuite ete
  retiree apres retour utilisateur : l'application revient a une largeur
  minimale desktop de 1040 px, avec des breakpoints CSS simplifies pour ne pas
  casser l'interface dense.
- Prod alignee : non applicable, application non distribuee ni installee.
  GitHub `origin/main` n'est pas encore aligne avec ces changements locaux.
- Validation reelle effectuee : validations automatiques et build installable
  reussis le 2026-05-06 apres correction.

Cause constatee :

- Les icones reelles etaient rendues en `data:image/png;base64,...`, mais la
  CSP Tauri n'autorisait pas `data:` dans `img-src`.
- Le responsive a d'abord ete trop ouvert par `minWidth: 720`, alors que
  l'interface actuelle est une application desktop dense. Cette largeur
  declenchait des empilements et controles trop serres.

Commandes validees :

- `npm run typecheck`
- `npm run build`
- `cargo fmt --manifest-path src-tauri\Cargo.toml -- --check`
- `cargo test --manifest-path src-tauri\Cargo.toml` : 24 tests reussis.
- `git diff --check`
- `npm audit --audit-level=high` : 0 vulnerabilite haute detectee.
- `npm run tauri build`

Notes :

- Retest manuel dans une fenetre Tauri : a refaire par l'utilisateur sur
  l'affichage des icones et le comportement a la largeur minimale 1040 px.

## 2026-05-06 - Correction filtres catalogue

Etat :

- Repo modifie : reprise ciblee de la toolbar catalogue. La recherche occupe
  maintenant une ligne complete, `Categorie` et `Affichage` partagent une ligne
  plus stable, les boutons d'affichage peuvent se repartir sans debordement, et
  les boutons `Toutes` / categories restent en defilement horizontal controle.
- Prod alignee : non applicable, application non distribuee ni installee.
  GitHub `origin/main` n'est pas encore aligne avec ces changements locaux.
- Validation reelle effectuee : validations automatiques et build installable
  reussis le 2026-05-06 apres correction.

Cause constatee :

- La toolbar avait trois colonnes dans le panneau catalogue alors que ce panneau
  devient etroit quand la fiche detail reste a droite. Les colonnes `Categorie`
  et `Affichage` entraient en concurrence avec la bande de filtres.

Commandes validees :

- `npm run typecheck`
- `npm run build`
- `cargo fmt --manifest-path src-tauri\Cargo.toml -- --check`
- `cargo test --manifest-path src-tauri\Cargo.toml` : 24 tests reussis.
- `git diff --check`
- `npm audit --audit-level=high` : 0 vulnerabilite haute detectee.
- `npm run tauri build`

Notes :

- Retest manuel dans une fenetre Tauri : verifier au minimum de largeur que
  `Categorie`, `Toutes` et `Affichage` restent lisibles et cliquables.

## 2026-05-06 - Commits et push actions/CSS

Etat :

- Repo modifie : deux commits crees puis pousses sur `origin/main`.
- Prod alignee : non applicable, application non distribuee ni installee.
  Depot GitHub aligne avec le repo local.
- Validation reelle effectuee : push GitHub reussi le 2026-05-06.

Commits pousses :

- `265d05e` (`feat: add software controls and local state`)
- `9744597` (`docs: record software controls validation`)

Notes :

- Les validations automatiques notees dans les entrees precedentes restent les
  validations de reference pour ce push.
- Retest manuel complet dans la fenetre Tauri : encore a confirmer par
  l'utilisateur avant de considerer le chantier fonctionnellement termine.

## 2026-05-07 - Prerelease GitHub v0.1.0

Etat :

- Repo modifie : non au moment de la creation de release ; la release cible le
  commit `011219f` sur `main`.
- Prod alignee : non applicable, application non distribuee ni installee comme
  prod. Une prerelease GitHub publique existe.
- Validation reelle effectuee : creation et verification GitHub de la
  prerelease `v0.1.0` reussies le 2026-05-07.

Release :

- URL : `https://github.com/kinowill/Commode/releases/tag/v0.1.0`
- Statut GitHub : prerelease.
- Titre : `Commode v0.1.0 - Prototype Windows`.

Artefacts verifies :

- `Commode_0.1.0_x64-setup.exe`
  - Taille : 2 093 113 octets.
  - SHA256 :
    `d303426de67d6f4093c4997bce6e3f740360dea79e8c7b7848dbad78e87958b1`.
- `Commode_0.1.0_x64_en-US.msi`
  - Taille : 3 219 456 octets.
  - SHA256 :
    `d00bb79080d72c87dad13b9c4d068f20bab91aedab7782e58b8bc4cf553f71b1`.

Notes :

- Cette publication est volontairement une prerelease, pas une version stable,
  car le retest manuel complet dans la fenetre Tauri reste a confirmer.
