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
- [ ] Choisir la stack technique apres comparaison des consequences.
- [ ] Definir les sources locales de detection des logiciels Windows.
- [ ] Definir les donnees affichees pour chaque logiciel.
- [ ] Definir le fonctionnement de la recherche et des categories.
- [ ] Definir une strategie prudente pour verifier les versions disponibles.
- [ ] Definir une strategie prudente pour proposer ou lancer les mises a jour.
- [ ] Definir le perimetre du terminal integre PowerShell/CMD et ses garde-fous.
- [ ] Definir les attentes de finition UI : navigation, details, etats vides,
      erreurs, chargements, accessibilite.
- [ ] Construire un premier prototype local en lecture seule.
- [ ] Valider le prototype sur la machine cible.

## Bloquants / arbitrages

- La stack n'est pas encore choisie.
- Le niveau d'automatisation des mises a jour n'est pas encore tranche.
- Le terminal integre demande un cadrage securite avant implementation.
- Le depot GitHub `https://github.com/kinowill/Commode` est relie en remote
  `origin`, verifie en lecture, et le premier commit a ete pousse.
- Les sources externes eventuelles doivent etre confirmees avant usage reseau.

## Decisions

- 2026-05-06 : Commode doit etre une vraie application Windows installable.
- 2026-05-06 : Commode doit fonctionner en privacy-first strict : inventaire
  100% local, aucune telemetrie, aucune synchro cloud, acces reseau seulement
  sur clic explicite pour verifier les mises a jour.
- 2026-05-06 : commits et push GitHub autorises, avec verification anti-leak et
  `.gitignore` prudent.
