# Commode - Roadmap

## Objectif courant

Definir proprement le produit avant implementation : application Windows
installable qui liste les logiciels installes, les classe par categorie, affiche des
informations utiles, permet la recherche, puis prepare la verification et les
mises a jour. L'application devra aussi prevoir une fenetre terminal integree
PowerShell/CMD et une interface avec un niveau de finition eleve.

## Prochaines taches

- [x] Clarifier le type d'application souhaite : desktop native, web locale,
      ou hybride.
- [x] Definir la politique privacy-first et l'usage reseau.
- [x] Initialiser git avec un `.gitignore` prudent avant le premier commit.
- [x] Relier le remote GitHub et pousser la documentation initiale.
- [x] Choisir la stack technique apres comparaison des consequences.
- [x] Verifier les prerequis locaux Node/npm/Rust/Cargo.
- [x] Generer le squelette Tauri 2 + React + TypeScript + Vite.
- [x] Definir et implementer une premiere source locale de detection Windows :
      registre Uninstall HKLM/HKCU.
- [x] Definir et afficher les premieres donnees logiciel : nom, editeur,
      version, date, taille estimee, emplacement, source, commande de
      desinstallation.
- [x] Definir et implementer le premier fonctionnement recherche/categories.
- [x] Definir et implementer une premiere strategie prudente pour verifier les
      versions disponibles via `winget upgrade`, lancee uniquement sur action
      explicite.
- [ ] Definir une strategie prudente pour proposer ou lancer les mises a jour.
- [ ] Durcir les garde-fous du terminal integre PowerShell/CMD.
- [x] Ajouter un premier garde-fou terminal : detection de commandes sensibles,
      confirmation avant execution, historique local effacable et commandes
      rapides.
- [x] Definir et implementer les actions controlees sur un logiciel : ouvrir
      l'emplacement dans l'Explorateur, lancer l'application, modifier les
      metadonnees locales, masquer/retirer de l'affichage, et desinstaller
      seulement avec confirmation explicite.
- [x] Persister des metadonnees locales par logiciel : categorie personnalisee,
      note, favori et masquage.
- [x] Extraire et mettre en cache une premiere version des vraies icones
      logiciel depuis les chemins Windows detectes.
- [x] Corriger les retours de validation utilisateur : lancement Windows avec
      elevation requise, commandes rapides PowerShell, et acces clair aux
      logiciels masques.
- [x] Corriger les retours de validation utilisateur : icones absentes a cause
      de la CSP Tauri et responsive desktop trop rigide.
- [x] Stabiliser le responsive desktop apres retour utilisateur : abandon de la
      largeur minimale 720 px, retour a une largeur minimale Tauri 1040 px, et
      simplification des breakpoints CSS.
- [x] Corriger les filtres du catalogue : `Categorie`, bouton `Toutes` et
      `Affichage` ne doivent plus se chevaucher ni casser au minimum desktop.
- [x] Implementer une premiere finition UI : navigation, details, etats vides,
      erreurs, chargements, accessibilite.
- [x] Ajouter des pictogrammes embarques qui fonctionnent en mode non installe.
- [x] Supprimer les faux liens de navigation d'une interface mono-page.
- [x] Deplacer le terminal en panneau plein largeur ouvert a la demande.
- [x] Nettoyer le wording des mises a jour tant que la fonction n'est pas
      configuree.
- [x] Ajouter un bouton de filtre "Toutes" visible pour revenir au catalogue
      complet sans passer par la liste deroulante.
- [x] Construire un premier prototype local.
- [x] Pousser le prototype applicatif sur GitHub.
- [ ] Definir le perimetre d'une recherche locale de documents : dossiers,
      types de fichiers, scan a la demande ou index, et garde-fous de
      confidentialite/performance.
- [ ] Valider manuellement le prototype dans une fenetre Tauri sur la machine
      cible.

## Bloquants / arbitrages

- Le niveau d'automatisation des mises a jour n'est pas encore tranche :
  verification via `winget` implementee, proposition/lancement de mise a jour
  non implementes.
- Le terminal integre fonctionne en commande explicite avec premiers
  garde-fous, mais ses garde-fous avances restent a durcir.
- Les pictogrammes embarques restent le fallback ; une premiere extraction/cache
  des vraies icones Windows est implementee localement. La CSP Tauri autorise
  maintenant les icones base64, mais le rendu reste a valider manuellement sur
  la machine cible.
- Les actions logiciel sont implementees localement par niveau de risque, avec
  confirmation pour la desinstallation, mais elles restent a valider
  manuellement dans une fenetre Tauri.
- Le responsive desktop a ete stabilise localement autour d'une largeur
  minimale Tauri de 1040 px, mais il reste a valider visuellement dans la
  fenetre Tauri.
- Les filtres catalogue ont ete repris localement, mais le rendu doit etre
  revalide manuellement dans la fenetre Tauri au minimum de largeur.
- La recherche de documents est une idee produit notee, non implementee. Elle
  ne doit pas etre melangee aux corrections actions/CSS en cours.
- Le depot GitHub `https://github.com/kinowill/Commode` est relie en remote
  `origin`, verifie en lecture, et le prototype applicatif a ete pousse. Les
  derniers changements locaux actions/CSS ne sont pas encore commites ni
  pousses.
- Les sources externes de mises a jour doivent etre confirmees avant usage
  reseau.

## Decisions

- 2026-05-06 : Commode doit etre une vraie application Windows installable.
- 2026-05-06 : Commode doit fonctionner en privacy-first strict : inventaire
  100% local, aucune telemetrie, aucune synchro cloud, acces reseau seulement
  sur clic explicite pour verifier les mises a jour.
- 2026-05-06 : commits et push GitHub autorises, avec verification anti-leak et
  `.gitignore` prudent.
- 2026-05-06 : stack retenue : Tauri 2 + React + TypeScript + Vite + Rust.
- 2026-05-06 : premiere detection locale implementee via le registre Windows
  Uninstall HKLM/HKCU.
