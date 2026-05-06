import { useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type InstalledApp = {
  id: string;
  name: string;
  version?: string;
  publisher?: string;
  installLocation?: string;
  installDate?: string;
  estimatedSizeMb?: number;
  uninstallString?: string;
  source: string;
  category: string;
  updateHint: string;
};

type AppIconProps = {
  app: InstalledApp;
  size?: "compact" | "large";
};

type SoftwareInventory = {
  scannedAt: string;
  total: number;
  apps: InstalledApp[];
};

type TerminalShell = "powershell" | "cmd";

type TerminalResult = {
  shell: TerminalShell;
  command: string;
  success: boolean;
  exitCode?: number;
  stdout: string;
  stderr: string;
  durationMs: number;
};

const numberFormatter = new Intl.NumberFormat("fr-FR");

function App() {
  const [inventory, setInventory] = useState<SoftwareInventory | null>(null);
  const [isScanning, setIsScanning] = useState(false);
  const [scanError, setScanError] = useState<string | null>(null);
  const [query, setQuery] = useState("");
  const [category, setCategory] = useState("Toutes");
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [isTerminalOpen, setIsTerminalOpen] = useState(false);
  const [shell, setShell] = useState<TerminalShell>("powershell");
  const [terminalCommand, setTerminalCommand] = useState("winget --version");
  const [terminalHistory, setTerminalHistory] = useState<TerminalResult[]>([]);
  const [terminalError, setTerminalError] = useState<string | null>(null);
  const [isRunningCommand, setIsRunningCommand] = useState(false);

  async function scanSoftware() {
    setIsScanning(true);
    setScanError(null);

    try {
      const nextInventory = await invoke<SoftwareInventory>("scan_installed_apps");
      setInventory(nextInventory);
      setSelectedId((currentId) => currentId ?? nextInventory.apps[0]?.id ?? null);
    } catch (error) {
      setScanError(error instanceof Error ? error.message : String(error));
    } finally {
      setIsScanning(false);
    }
  }

  useEffect(() => {
    void scanSoftware();
  }, []);

  const categories = useMemo(() => {
    const counts = new Map<string, number>();
    for (const app of inventory?.apps ?? []) {
      counts.set(app.category, (counts.get(app.category) ?? 0) + 1);
    }

    return ["Toutes", ...Array.from(counts.keys()).sort((left, right) => left.localeCompare(right))];
  }, [inventory]);

  const filteredApps = useMemo(() => {
    const normalizedQuery = query.trim().toLowerCase();

    return (inventory?.apps ?? []).filter((app) => {
      const matchesCategory = category === "Toutes" || app.category === category;
      const searchable = [app.name, app.publisher, app.version, app.category]
        .filter(Boolean)
        .join(" ")
        .toLowerCase();
      return matchesCategory && searchable.includes(normalizedQuery);
    });
  }, [category, inventory, query]);

  const selectedApp = useMemo(() => {
    if (filteredApps.length === 0) {
      return null;
    }

    return filteredApps.find((app) => app.id === selectedId) ?? filteredApps[0];
  }, [filteredApps, selectedId]);

  const categoryStats = useMemo(() => {
    const counts = new Map<string, number>();
    for (const app of inventory?.apps ?? []) {
      counts.set(app.category, (counts.get(app.category) ?? 0) + 1);
    }

    return Array.from(counts.entries())
      .map(([name, count]) => ({ name, count }))
      .sort((left, right) => right.count - left.count);
  }, [inventory]);

  const visibleCategoryStats = useMemo(() => {
    const topCategories = categoryStats.slice(0, 6);

    if (category === "Toutes" || topCategories.some((entry) => entry.name === category)) {
      return topCategories;
    }

    const selectedCategory = categoryStats.find((entry) => entry.name === category);
    return selectedCategory ? [...topCategories.slice(0, 5), selectedCategory] : topCategories;
  }, [category, categoryStats]);

  async function runCommand() {
    setIsRunningCommand(true);
    setTerminalError(null);

    try {
      const result = await invoke<TerminalResult>("run_terminal_command", {
        request: { shell, command: terminalCommand },
      });
      setTerminalHistory((history) => [result, ...history].slice(0, 8));
    } catch (error) {
      setTerminalError(error instanceof Error ? error.message : String(error));
    } finally {
      setIsRunningCommand(false);
    }
  }

  return (
    <main className="app-shell">
      <aside className="sidebar" aria-label="Navigation principale">
        <div className="brand">
          <div className="brand-mark" aria-hidden="true">
            C
          </div>
          <div>
            <p className="eyebrow">Inventaire local</p>
            <h1>Commode</h1>
          </div>
        </div>

        <div className="scope-stack" aria-label="Etat de l'application">
          <div className="scope-item">
            <span>Vue</span>
            <strong>Catalogue</strong>
          </div>
          <div className="scope-item">
            <span>Mode</span>
            <strong>Portable</strong>
          </div>
          <div className="scope-item">
            <span>Réseau</span>
            <strong>Désactivé</strong>
          </div>
        </div>

        <div className="privacy-block">
          <span className="status-dot" aria-hidden="true" />
          <p>Local only. Réseau seulement sur action explicite.</p>
        </div>
      </aside>

      <section className="workspace">
        <header className="topbar">
          <div>
            <p className="eyebrow">Windows app store personnel</p>
            <h2>Logiciels installés</h2>
          </div>
          <div className="topbar-actions">
            <button className="secondary-button muted-action" type="button" disabled>
              Mises à jour non configurées
            </button>
            <button className="secondary-button" type="button" onClick={() => setIsTerminalOpen((value) => !value)}>
              {isTerminalOpen ? "Masquer terminal" : "Ouvrir terminal"}
            </button>
            <button className="primary-button" type="button" onClick={scanSoftware} disabled={isScanning}>
              {isScanning ? "Scan..." : "Rescanner"}
            </button>
          </div>
        </header>

        <section className="metric-grid" aria-label="Synthese inventaire">
          <article className="metric">
            <span>Total</span>
            <strong>{numberFormatter.format(inventory?.total ?? 0)}</strong>
          </article>
          <article className="metric">
            <span>Affiches</span>
            <strong>{numberFormatter.format(filteredApps.length)}</strong>
          </article>
          <article className="metric wide">
            <span>Dernier scan</span>
            <strong>{inventory ? formatScanTime(inventory.scannedAt) : "En attente"}</strong>
          </article>
          <article className="metric wide">
            <span>Mises à jour</span>
            <strong>Non configurées</strong>
          </article>
        </section>

        <section className="main-grid">
          <section className="catalogue-panel" id="catalogue" aria-label="Catalogue logiciels">
            <div className="panel-toolbar">
              <label className="search-field">
                <span>Recherche</span>
                <input
                  value={query}
                  onChange={(event) => setQuery(event.currentTarget.value)}
                  placeholder="Nom, editeur, version..."
                />
              </label>

              <label className="select-field">
                <span>Categorie</span>
                <select value={category} onChange={(event) => setCategory(event.currentTarget.value)}>
                  {categories.map((entry) => (
                    <option key={entry} value={entry}>
                      {entry}
                    </option>
                  ))}
                </select>
              </label>
            </div>

            {scanError ? <div className="notice error">{scanError}</div> : null}

            <div className="category-strip" aria-label="Repartition par categorie">
              <button
                className={category === "Toutes" ? "category-chip all-filter active" : "category-chip all-filter"}
                type="button"
                onClick={() => setCategory("Toutes")}
              >
                <span>Toutes</span>
                <strong>{numberFormatter.format(inventory?.total ?? 0)}</strong>
              </button>
              {visibleCategoryStats.map((entry) => (
                <button
                  className={entry.name === category ? "category-chip active" : "category-chip"}
                  key={entry.name}
                  type="button"
                  onClick={() => setCategory(entry.name)}
                >
                  <span>{entry.name}</span>
                  <strong>{numberFormatter.format(entry.count)}</strong>
                </button>
              ))}
            </div>

            <div className="software-list" role="list" aria-label="Logiciels detectes">
              {filteredApps.map((app) => (
                <button
                  className={app.id === selectedApp?.id ? "software-row selected" : "software-row"}
                  key={app.id}
                  type="button"
                  onClick={() => setSelectedId(app.id)}
                >
                  <AppIcon app={app} />
                  <span className="software-main">
                    <strong>{app.name}</strong>
                    <small>{app.publisher ?? "Editeur inconnu"}</small>
                  </span>
                  <span className="software-meta">
                    <small>{app.category}</small>
                    <strong>{app.version ?? "n/a"}</strong>
                  </span>
                </button>
              ))}

              {!isScanning && filteredApps.length === 0 ? (
                <div className="empty-state">Aucun logiciel ne correspond aux filtres.</div>
              ) : null}
            </div>
          </section>

          <section className="details-panel" id="details" aria-label="Details logiciel">
            {selectedApp ? (
              <>
                <div className="details-header">
                  <AppIcon app={selectedApp} size="large" />
                  <div>
                    <p className="eyebrow">{selectedApp.category}</p>
                    <h3>{selectedApp.name}</h3>
                    <p>{selectedApp.publisher ?? "Editeur inconnu"}</p>
                  </div>
                </div>

                <dl className="details-list">
                  <Detail label="Version" value={selectedApp.version} />
                  <Detail label="Installation" value={selectedApp.installDate} />
                  <Detail
                    label="Taille estimee"
                    value={
                      selectedApp.estimatedSizeMb
                        ? `${numberFormatter.format(selectedApp.estimatedSizeMb)} Mo`
                        : undefined
                    }
                  />
                  <Detail label="Source registre" value={selectedApp.source} />
                  <Detail label="Emplacement" value={selectedApp.installLocation} />
                  <Detail label="Desinstallation" value={selectedApp.uninstallString} />
                  <Detail label="Mise a jour" value={selectedApp.updateHint} />
                </dl>
              </>
            ) : (
              <div className="empty-state">Selectionne un logiciel pour afficher sa fiche.</div>
            )}
          </section>
        </section>

        {isTerminalOpen ? (
          <section className="terminal-panel" id="terminal" aria-label="Terminal integre">
            <div className="terminal-header">
              <div>
                <p className="eyebrow">Commande explicite</p>
                <h3>Terminal</h3>
              </div>
              <div className="segmented-control" aria-label="Choix du shell">
                <button
                  className={shell === "powershell" ? "active" : ""}
                  type="button"
                  onClick={() => setShell("powershell")}
                >
                  PS
                </button>
                <button className={shell === "cmd" ? "active" : ""} type="button" onClick={() => setShell("cmd")}>
                  CMD
                </button>
              </div>
            </div>

            <label className="command-field">
              <span>Commande</span>
              <textarea
                value={terminalCommand}
                onChange={(event) => setTerminalCommand(event.currentTarget.value)}
                spellCheck={false}
              />
            </label>

            <div className="terminal-actions">
              <button className="primary-button" type="button" onClick={runCommand} disabled={isRunningCommand}>
                {isRunningCommand ? "Execution..." : "Executer"}
              </button>
              <button className="secondary-button" type="button" onClick={() => setTerminalHistory([])}>
                Effacer
              </button>
            </div>

            {terminalError ? <div className="notice error">{terminalError}</div> : null}

            <div className="terminal-output" aria-live="polite">
              {terminalHistory.length === 0 ? (
                <p>Aucune commande executee dans cette session.</p>
              ) : (
                terminalHistory.map((entry) => (
                  <article className="terminal-entry" key={`${entry.command}-${entry.durationMs}-${entry.exitCode}`}>
                    <header>
                      <strong>
                        {entry.shell} · code {entry.exitCode ?? "n/a"} · {entry.durationMs} ms
                      </strong>
                      <span>{entry.success ? "OK" : "Erreur"}</span>
                    </header>
                    <code>{entry.command}</code>
                    {entry.stdout ? <pre>{entry.stdout}</pre> : null}
                    {entry.stderr ? <pre className="stderr">{entry.stderr}</pre> : null}
                  </article>
                ))
              )}
            </div>
          </section>
        ) : null}
      </section>
    </main>
  );
}

function AppIcon({ app, size = "compact" }: AppIconProps) {
  return (
    <span className={`app-icon ${size}`} aria-hidden="true">
      <svg viewBox="0 0 48 48" role="img">
        <IconShape category={app.category} />
      </svg>
      <span>{app.name.slice(0, 1).toUpperCase()}</span>
    </span>
  );
}

function IconShape({ category }: { category: string }) {
  switch (category) {
    case "Navigateurs":
      return (
        <>
          <circle cx="24" cy="24" r="15" />
          <path d="M10 22h28M24 9c6 7 6 23 0 30M24 9c-6 7-6 23 0 30" />
        </>
      );
    case "Developpement":
      return (
        <>
          <path d="M18 15 9 24l9 9M30 15l9 9-9 9" />
          <path d="M27 11 21 37" />
        </>
      );
    case "Creation":
      return (
        <>
          <path d="M14 34 32 16l4 4-18 18h-4v-4Z" />
          <path d="M29 19 33 23" />
        </>
      );
    case "Communication":
      return (
        <>
          <path d="M11 14h26v18H21l-7 6v-6h-3V14Z" />
          <path d="M17 22h14M17 27h9" />
        </>
      );
    case "Jeux":
      return (
        <>
          <path d="M15 20h18l5 12-4 4-6-5h-8l-6 5-4-4 5-12Z" />
          <path d="M17 26h8M21 22v8M31 25h.1M35 29h.1" />
        </>
      );
    case "Securite":
      return (
        <>
          <path d="M24 9 37 14v10c0 8-5 13-13 16-8-3-13-8-13-16V14l13-5Z" />
          <path d="m18 24 4 4 8-9" />
        </>
      );
    case "Productivite":
      return (
        <>
          <path d="M15 10h18v28H15z" />
          <path d="M19 17h10M19 24h10M19 31h7" />
        </>
      );
    case "Systeme":
      return (
        <>
          <path d="M24 13v-4M24 39v-4M35 24h4M9 24h4M32 16l3-3M13 35l3-3M32 32l3 3M13 13l3 3" />
          <circle cx="24" cy="24" r="8" />
        </>
      );
    default:
      return (
        <>
          <rect x="13" y="11" width="22" height="26" rx="3" />
          <path d="M18 18h12M18 24h12M18 30h8" />
        </>
      );
  }
}

function Detail({ label, value }: { label: string; value?: string }) {
  return (
    <div>
      <dt>{label}</dt>
      <dd>{value?.trim() || "Non renseigne"}</dd>
    </div>
  );
}

function formatScanTime(value: string) {
  const epoch = Number(value);
  if (!Number.isFinite(epoch) || epoch <= 0) {
    return "Inconnu";
  }

  return new Intl.DateTimeFormat("fr-FR", {
    dateStyle: "short",
    timeStyle: "medium",
  }).format(new Date(epoch * 1000));
}

export default App;
