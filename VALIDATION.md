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
