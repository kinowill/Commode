# Commode

Commode est une application Windows installable qui catalogue localement les
logiciels presents sur le PC.

## Stack

- Tauri 2
- React
- TypeScript
- Vite
- Rust

## Principes

- Inventaire logiciel 100% local.
- Aucune telemetrie.
- Aucune synchronisation cloud.
- Acces reseau seulement sur action explicite de l'utilisateur.
- Terminal integre PowerShell/CMD uniquement sur commande utilisateur.

## Commandes utiles

```powershell
npm install
npm run dev
npm run tauri dev
npm run build
cargo check --manifest-path src-tauri/Cargo.toml
```
