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
- [ ] Definir une strategie prudente pour verifier les versions disponibles.
- [ ] Definir une strategie prudente pour proposer ou lancer les mises a jour.
- [ ] Durcir les garde-fous du terminal integre PowerShell/CMD.
- [ ] Definir et implementer les actions controlees sur un logiciel : ouvrir
      l'emplacement dans l'Explorateur, lancer l'application, modifier les
      metadonnees locales, masquer/retirer de l'affichage, et desinstaller
      seulement avec confirmation explicite.
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
- [ ] Valider manuellement le prototype dans une fenetre Tauri sur la machine
      cible.

## Bloquants / arbitrages

- Le niveau d'automatisation des mises a jour n'est pas encore tranche.
- Le terminal integre fonctionne en commande explicite, mais ses garde-fous
  avances restent a durcir.
- Les pictogrammes actuels sont embarques par categorie ; les vraies icones
  extraites depuis les executables restent a implementer.
- Les mises a jour sont affichees comme non configurees tant que la strategie
  produit/technique n'est pas tranchee.
- Les actions logiciel doivent etre separees par niveau de risque : ouvrir ou
  lancer est peu risque, modifier des metadonnees reste local, desinstaller ou
  supprimer doit demander une confirmation explicite.
- Le depot GitHub `https://github.com/kinowill/Commode` est relie en remote
  `origin`, verifie en lecture, et le prototype applicatif a ete pousse.
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
