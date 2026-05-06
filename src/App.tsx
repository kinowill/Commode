import { useCallback, useEffect, useMemo, useRef, useState } from "react";
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
  iconSource?: string;
  executablePath?: string;
  source: string;
  category: string;
  updateHint: string;
};

type SoftwareInventory = {
  scannedAt: string;
  total: number;
  apps: InstalledApp[];
};

type LocalMetadata = {
  customCategory?: string | null;
  note?: string | null;
  favorite?: boolean;
  hidden?: boolean;
};

type LocalMetadataMap = Record<string, LocalMetadata>;

type TerminalShell = "powershell" | "cmd";
type VisibilityFilter = "visible" | "all" | "hidden";

type TerminalResult = {
  shell: TerminalShell;
  command: string;
  success: boolean;
  exitCode?: number;
  stdout: string;
  stderr: string;
  durationMs: number;
  risky: boolean;
  timestamp: number;
};

type HistoryEntry = {
  shell: TerminalShell;
  command: string;
  success: boolean;
  exitCode?: number;
  stdout: string;
  stderr: string;
  durationMs: number;
  timestamp: number;
};

type WingetUpgrade = {
  name: string;
  id: string;
  currentVersion: string;
  availableVersion: string;
  source: string;
};

type WingetReport = {
  success: boolean;
  checkedAt: number;
  upgrades: WingetUpgrade[];
  rawOutput: string;
  message?: string | null;
};

type ConfirmRequest = {
  title: string;
  detail: string;
  confirmLabel: string;
  cancelLabel?: string;
  tone?: "danger" | "warning";
  onConfirm: () => void | Promise<void>;
};

type QuickCommand = {
  label: string;
  command: string;
  shell: TerminalShell;
  risky: boolean;
  description: string;
};

const QUICK_COMMANDS: QuickCommand[] = [
  {
    label: "Version winget",
    command: "winget --version",
    shell: "powershell",
    risky: false,
    description: "Affiche la version installee de winget.",
  },
  {
    label: "Liste services actifs",
    command:
      "Microsoft.PowerShell.Management\\Get-Service | Microsoft.PowerShell.Core\\Where-Object Status -eq 'Running' | Microsoft.PowerShell.Utility\\Select-Object -First 20 Name,Status,DisplayName",
    shell: "powershell",
    risky: false,
    description: "Liste les 20 premiers services en cours.",
  },
  {
    label: "Top processus",
    command:
      "Microsoft.PowerShell.Management\\Get-Process | Microsoft.PowerShell.Utility\\Sort-Object -Property CPU -Descending | Microsoft.PowerShell.Utility\\Select-Object -First 10 Name,Id,CPU",
    shell: "powershell",
    risky: false,
    description: "Top 10 processus par CPU.",
  },
  {
    label: "Espace disque",
    command:
      "Microsoft.PowerShell.Management\\Get-PSDrive -PSProvider FileSystem | Microsoft.PowerShell.Utility\\Select-Object Name,Used,Free",
    shell: "powershell",
    risky: false,
    description: "Espace utilise/libre par lecteur.",
  },
  {
    label: "Verification SFC",
    command: "sfc /scannow",
    shell: "cmd",
    risky: true,
    description: "Verifie l'integrite des fichiers systeme (long, sensible).",
  },
  {
    label: "Vider DNS",
    command: "ipconfig /flushdns",
    shell: "cmd",
    risky: false,
    description: "Vide le cache DNS local.",
  },
];

const numberFormatter = new Intl.NumberFormat("fr-FR");

function App() {
  const [inventory, setInventory] = useState<SoftwareInventory | null>(null);
  const [isScanning, setIsScanning] = useState(false);
  const [scanError, setScanError] = useState<string | null>(null);
  const [query, setQuery] = useState("");
  const [category, setCategory] = useState("Toutes");
  const [selectedId, setSelectedId] = useState<string | null>(null);

  const [localMetadata, setLocalMetadata] = useState<LocalMetadataMap>({});
  const [editMode, setEditMode] = useState(false);
  const [visibilityFilter, setVisibilityFilter] = useState<VisibilityFilter>("visible");

  const [iconCache, setIconCache] = useState<Record<string, string>>({});
  const iconRequestedRef = useRef<Set<string>>(new Set());

  const [isTerminalOpen, setIsTerminalOpen] = useState(false);
  const [shell, setShell] = useState<TerminalShell>("powershell");
  const [terminalCommand, setTerminalCommand] = useState("winget --version");
  const [terminalHistory, setTerminalHistory] = useState<TerminalResult[]>([]);
  const [terminalError, setTerminalError] = useState<string | null>(null);
  const [isRunningCommand, setIsRunningCommand] = useState(false);

  const [updatesReport, setUpdatesReport] = useState<WingetReport | null>(null);
  const [isCheckingUpdates, setIsCheckingUpdates] = useState(false);
  const [updatesError, setUpdatesError] = useState<string | null>(null);

  const [actionFeedback, setActionFeedback] = useState<string | null>(null);
  const [actionError, setActionError] = useState<string | null>(null);

  const [confirmRequest, setConfirmRequest] = useState<ConfirmRequest | null>(null);

  async function scanSoftware() {
    setIsScanning(true);
    setScanError(null);

    try {
      const nextInventory = await invoke<SoftwareInventory>("scan_installed_apps");
      setInventory(nextInventory);
      setSelectedId((currentId) => currentId ?? nextInventory.apps[0]?.id ?? null);
    } catch (error) {
      setScanError(toMessage(error));
    } finally {
      setIsScanning(false);
    }
  }

  useEffect(() => {
    void scanSoftware();
    void loadInitialMetadata();
    void loadInitialHistory();
  }, []);

  async function loadInitialMetadata() {
    try {
      const map = await invoke<LocalMetadataMap>("load_local_metadata");
      setLocalMetadata(map ?? {});
    } catch (error) {
      console.warn("Lecture metadonnees impossible:", error);
    }
  }

  async function loadInitialHistory() {
    try {
      const entries = await invoke<HistoryEntry[]>("load_terminal_history");
      const restored: TerminalResult[] = entries.map((entry) => ({
        shell: entry.shell,
        command: entry.command,
        success: entry.success,
        exitCode: entry.exitCode,
        stdout: entry.stdout,
        stderr: entry.stderr,
        durationMs: entry.durationMs,
        timestamp: entry.timestamp,
        risky: false,
      }));
      setTerminalHistory(restored);
    } catch (error) {
      console.warn("Lecture historique impossible:", error);
    }
  }

  const allApps = inventory?.apps ?? [];

  const hiddenCount = useMemo(() => {
    return allApps.filter((app) => localMetadata[app.id]?.hidden).length;
  }, [allApps, localMetadata]);

  const visibleApps = useMemo(() => {
    if (visibilityFilter === "all") return allApps;
    if (visibilityFilter === "hidden") {
      return allApps.filter((app) => localMetadata[app.id]?.hidden);
    }
    return allApps.filter((app) => !localMetadata[app.id]?.hidden);
  }, [allApps, localMetadata, visibilityFilter]);

  const categoryStats = useMemo(() => {
    const counts = new Map<string, number>();
    for (const app of visibleApps) {
      const effective = localMetadata[app.id]?.customCategory || app.category;
      counts.set(effective, (counts.get(effective) ?? 0) + 1);
    }
    return Array.from(counts.entries())
      .map(([name, count]) => ({ name, count }))
      .sort((left, right) => right.count - left.count);
  }, [visibleApps, localMetadata]);

  const visibleCategoryStats = useMemo(() => {
    const top = categoryStats.slice(0, 6);
    if (category === "Toutes" || top.some((entry) => entry.name === category)) {
      return top;
    }
    const selected = categoryStats.find((entry) => entry.name === category);
    return selected ? [...top.slice(0, 5), selected] : top;
  }, [category, categoryStats]);

  const categoryNames = useMemo(() => {
    const names = new Set<string>();
    for (const app of visibleApps) {
      names.add(localMetadata[app.id]?.customCategory || app.category);
    }
    return ["Toutes", ...Array.from(names).sort((left, right) => left.localeCompare(right))];
  }, [visibleApps, localMetadata]);

  useEffect(() => {
    if (category !== "Toutes" && !categoryNames.includes(category)) {
      setCategory("Toutes");
    }
  }, [category, categoryNames]);

  const filteredApps = useMemo(() => {
    const normalized = query.trim().toLowerCase();
    return visibleApps.filter((app) => {
      const meta = localMetadata[app.id];
      const effective = meta?.customCategory || app.category;
      if (category !== "Toutes" && effective !== category) return false;
      const haystack = [
        app.name,
        app.publisher,
        app.version,
        effective,
        meta?.note ?? "",
      ]
        .filter(Boolean)
        .join(" ")
        .toLowerCase();
      return haystack.includes(normalized);
    });
  }, [visibleApps, localMetadata, query, category]);

  const sortedApps = useMemo(() => {
    return [...filteredApps].sort((left, right) => {
      const favLeft = localMetadata[left.id]?.favorite ? 1 : 0;
      const favRight = localMetadata[right.id]?.favorite ? 1 : 0;
      if (favLeft !== favRight) return favRight - favLeft;
      return left.name.toLowerCase().localeCompare(right.name.toLowerCase());
    });
  }, [filteredApps, localMetadata]);

  const selectedApp = useMemo(() => {
    if (sortedApps.length === 0) return null;
    return sortedApps.find((app) => app.id === selectedId) ?? sortedApps[0];
  }, [sortedApps, selectedId]);

  useEffect(() => {
    if (editMode && (!selectedApp || selectedApp.id !== selectedId)) {
      setEditMode(false);
    }
  }, [selectedApp, selectedId, editMode]);

  const requestIcon = useCallback(
    async (app: InstalledApp) => {
      const source = app.iconSource || app.executablePath;
      if (!source) return;
      if (iconCache[app.id]) return;
      if (iconRequestedRef.current.has(app.id)) return;
      iconRequestedRef.current.add(app.id);

      try {
        const response = await invoke<{ appId: string; base64Png: string }>("get_app_icon", {
          request: { appId: app.id, sourcePath: source },
        });
        setIconCache((current) => ({ ...current, [response.appId]: response.base64Png }));
      } catch {
        // silencieux : on garde le SVG fallback
      }
    },
    [iconCache],
  );

  useEffect(() => {
    const slice = sortedApps.slice(0, 60);
    for (const app of slice) {
      void requestIcon(app);
    }
  }, [sortedApps, requestIcon]);

  useEffect(() => {
    if (selectedApp) {
      void requestIcon(selectedApp);
    }
  }, [selectedApp, requestIcon]);

  function showFeedback(message: string) {
    setActionFeedback(message);
    setActionError(null);
    window.setTimeout(() => setActionFeedback(null), 3500);
  }

  function showError(message: string) {
    setActionError(message);
    setActionFeedback(null);
    window.setTimeout(() => setActionError(null), 5000);
  }

  async function handleLaunch(app: InstalledApp) {
    const target = app.executablePath || app.iconSource;
    if (!target) {
      showError("Aucun executable detecte pour cette application.");
      return;
    }
    try {
      await invoke("launch_app", { request: { target } });
      showFeedback(`Lancement de ${app.name}.`);
    } catch (error) {
      showError(toMessage(error));
    }
  }

  async function handleOpen(app: InstalledApp) {
    const target = app.executablePath || app.iconSource;
    if (!target) {
      return handleOpenFolder(app);
    }
    try {
      await invoke("launch_app", { request: { target } });
      showFeedback(`Ouverture de ${app.name}.`);
    } catch (error) {
      showError(toMessage(error));
    }
  }

  async function handleOpenFolder(app: InstalledApp) {
    const target = app.installLocation || app.executablePath || app.iconSource;
    if (!target) {
      showError("Aucun emplacement connu pour cette application.");
      return;
    }
    try {
      await invoke("open_install_folder", { request: { target } });
      showFeedback("Ouverture dans l'Explorateur.");
    } catch (error) {
      showError(toMessage(error));
    }
  }

  async function handleCopyPath(app: InstalledApp) {
    const target = app.executablePath || app.installLocation || app.iconSource;
    if (!target) {
      showError("Aucun chemin connu pour cette application.");
      return;
    }
    try {
      await navigator.clipboard.writeText(target);
      showFeedback("Chemin copie dans le presse-papiers.");
    } catch (error) {
      showError(toMessage(error));
    }
  }

  function handleUninstallRequest(app: InstalledApp) {
    if (!app.uninstallString) {
      showError("Pas de commande de desinstallation pour cette application.");
      return;
    }
    setConfirmRequest({
      title: `Desinstaller ${app.name} ?`,
      detail: `Cette action lance la commande Windows : ${app.uninstallString}\n\nElle peut ouvrir une fenetre Windows ou desinstaller silencieusement selon le logiciel.`,
      confirmLabel: "Desinstaller",
      tone: "danger",
      onConfirm: async () => {
        try {
          await invoke("uninstall_app", {
            request: { uninstallString: app.uninstallString!, confirmed: true },
          });
          showFeedback(`Desinstallation de ${app.name} demarree.`);
        } catch (error) {
          showError(toMessage(error));
        }
      },
    });
  }

  async function persistMetadata(appId: string, next: LocalMetadata) {
    try {
      const updated = await invoke<LocalMetadataMap>("save_local_metadata", {
        request: { appId, metadata: next },
      });
      setLocalMetadata(updated);
      return true;
    } catch (error) {
      showError(toMessage(error));
      return false;
    }
  }

  async function toggleFavorite(app: InstalledApp) {
    const current = localMetadata[app.id] ?? {};
    await persistMetadata(app.id, { ...current, favorite: !current.favorite });
  }

  async function toggleHidden(app: InstalledApp) {
    const current = localMetadata[app.id] ?? {};
    const nextHidden = !current.hidden;
    if (await persistMetadata(app.id, { ...current, hidden: nextHidden })) {
      if (nextHidden) {
        setCategory("Toutes");
        setVisibilityFilter("hidden");
        showFeedback("Logiciel masque. Retrouve-le dans le filtre Masques.");
      } else {
        if (visibilityFilter === "hidden") {
          setVisibilityFilter("visible");
        }
        showFeedback("Logiciel de nouveau visible.");
      }
    }
  }

  async function commitMetadataForm(
    app: InstalledApp,
    customCategory: string,
    note: string,
  ) {
    const current = localMetadata[app.id] ?? {};
    const saved = await persistMetadata(app.id, {
      ...current,
      customCategory: customCategory.trim() ? customCategory.trim() : null,
      note: note.trim() ? note.trim() : null,
    });
    if (saved) {
      setEditMode(false);
      showFeedback("Metadonnees mises a jour.");
    }
  }

  async function runCommand(commandText: string, targetShell: TerminalShell) {
    setIsRunningCommand(true);
    setTerminalError(null);

    try {
      const result = await invoke<TerminalResult>("run_terminal_command", {
        request: { shell: targetShell, command: commandText, confirmedRisky: true },
      });
      setTerminalHistory((history) => [result, ...history].slice(0, 20));
    } catch (error) {
      setTerminalError(toMessage(error));
    } finally {
      setIsRunningCommand(false);
    }
  }

  async function handleRunCurrent() {
    const text = terminalCommand.trim();
    if (!text) return;

    let risky = false;
    try {
      const probe = await invoke<{ risky: boolean }>("check_command_risk", { command: text });
      risky = probe.risky;
    } catch {
      risky = false;
    }

    if (risky) {
      setConfirmRequest({
        title: "Commande sensible",
        detail: `Cette commande peut modifier ou supprimer des donnees :\n\n${text}\n\nConfirme pour l'executer.`,
        confirmLabel: "Executer quand meme",
        tone: "warning",
        onConfirm: () => runCommand(text, shell),
      });
    } else {
      void runCommand(text, shell);
    }
  }

  function handleQuickCommand(quick: QuickCommand) {
    setShell(quick.shell);
    setTerminalCommand(quick.command);
    setIsTerminalOpen(true);
    if (quick.risky) {
      setConfirmRequest({
        title: "Commande sensible",
        detail: `${quick.description}\n\n${quick.command}\n\nConfirme pour l'executer.`,
        confirmLabel: "Executer",
        tone: "warning",
        onConfirm: () => runCommand(quick.command, quick.shell),
      });
    } else {
      void runCommand(quick.command, quick.shell);
    }
  }

  async function handleClearHistory() {
    try {
      await invoke("clear_terminal_history");
      setTerminalHistory([]);
    } catch (error) {
      setTerminalError(toMessage(error));
    }
  }

  async function checkUpdates() {
    setIsCheckingUpdates(true);
    setUpdatesError(null);
    try {
      const report = await invoke<WingetReport>("check_software_updates");
      setUpdatesReport(report);
    } catch (error) {
      setUpdatesError(toMessage(error));
    } finally {
      setIsCheckingUpdates(false);
    }
  }

  const upgradesById = useMemo(() => {
    const map = new Map<string, WingetUpgrade>();
    if (!updatesReport) return map;
    for (const upgrade of updatesReport.upgrades) {
      map.set(upgrade.name.toLowerCase(), upgrade);
    }
    return map;
  }, [updatesReport]);

  const matchedUpgrade = (app: InstalledApp): WingetUpgrade | null => {
    const direct = upgradesById.get(app.name.toLowerCase());
    if (direct) return direct;
    if (!updatesReport) return null;
    return (
      updatesReport.upgrades.find(
        (up) =>
          app.name.toLowerCase().includes(up.name.toLowerCase()) ||
          up.name.toLowerCase().includes(app.name.toLowerCase()),
      ) ?? null
    );
  };

  return (
    <main className="app-shell">
      <aside className="sidebar" aria-label="Navigation principale">
        <div className="brand">
          <div className="brand-mark" aria-hidden="true">C</div>
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
            <span>Reseau</span>
            <strong>{updatesReport ? "Verifie" : "Desactive"}</strong>
          </div>
        </div>

        <div className="sidebar-quick">
          <p className="eyebrow">Commandes rapides</p>
          <div className="quick-list">
            {QUICK_COMMANDS.map((quick) => (
              <button
                key={quick.label}
                type="button"
                className={quick.risky ? "quick-item risky" : "quick-item"}
                onClick={() => handleQuickCommand(quick)}
                title={quick.description}
              >
                <span>{quick.label}</span>
                {quick.risky ? <em>sensible</em> : null}
              </button>
            ))}
          </div>
        </div>

        <div className="privacy-block">
          <span className="status-dot" aria-hidden="true" />
          <p>Local only. Reseau seulement sur action explicite.</p>
        </div>
      </aside>

      <section className="workspace">
        <header className="topbar">
          <div>
            <p className="eyebrow">Windows app store personnel</p>
            <h2>Logiciels installes</h2>
          </div>
          <div className="topbar-actions">
            <button
              className="secondary-button"
              type="button"
              onClick={checkUpdates}
              disabled={isCheckingUpdates}
              title="Lance winget upgrade"
            >
              {isCheckingUpdates ? "Verification..." : "Verifier mises a jour"}
            </button>
            <button
              className="secondary-button"
              type="button"
              onClick={() => setIsTerminalOpen((value) => !value)}
            >
              {isTerminalOpen ? "Masquer terminal" : "Ouvrir terminal"}
            </button>
            <button
              className="primary-button"
              type="button"
              onClick={scanSoftware}
              disabled={isScanning}
            >
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
            <strong>{numberFormatter.format(sortedApps.length)}</strong>
          </article>
          <article className="metric wide">
            <span>Dernier scan</span>
            <strong>{inventory ? formatScanTime(inventory.scannedAt) : "En attente"}</strong>
          </article>
          <article className="metric wide">
            <span>Mises a jour</span>
            <strong>
              {updatesReport
                ? `${numberFormatter.format(updatesReport.upgrades.length)} dispo`
                : "Non verifiees"}
            </strong>
          </article>
        </section>

        {actionFeedback ? <div className="notice success">{actionFeedback}</div> : null}
        {actionError ? <div className="notice error">{actionError}</div> : null}
        {updatesError ? <div className="notice error">winget : {updatesError}</div> : null}
        {updatesReport && !updatesReport.success && updatesReport.message ? (
          <div className="notice error">winget : {updatesReport.message}</div>
        ) : null}

        <section className="main-grid">
          <section className="catalogue-panel" aria-label="Catalogue logiciels">
            <div className="panel-toolbar">
              <label className="search-field">
                <span>Recherche</span>
                <input
                  value={query}
                  onChange={(event) => setQuery(event.currentTarget.value)}
                  placeholder="Nom, editeur, version, note..."
                />
              </label>

              <label className="select-field">
                <span>Categorie</span>
                <select
                  value={category}
                  onChange={(event) => setCategory(event.currentTarget.value)}
                >
                  {categoryNames.map((entry) => (
                    <option key={entry} value={entry}>
                      {entry}
                    </option>
                  ))}
                </select>
              </label>

              <div className="visibility-field">
                <span>Affichage</span>
                <div className="visibility-tabs" role="group" aria-label="Affichage des logiciels">
                  <button
                    type="button"
                    className={visibilityFilter === "visible" ? "active" : ""}
                    onClick={() => setVisibilityFilter("visible")}
                  >
                    Visibles
                  </button>
                  <button
                    type="button"
                    className={visibilityFilter === "all" ? "active" : ""}
                    onClick={() => setVisibilityFilter("all")}
                  >
                    Tous
                  </button>
                  <button
                    type="button"
                    className={visibilityFilter === "hidden" ? "active" : ""}
                    onClick={() => setVisibilityFilter("hidden")}
                    disabled={hiddenCount === 0}
                  >
                    Masques
                    {hiddenCount > 0 ? <span>{numberFormatter.format(hiddenCount)}</span> : null}
                  </button>
                </div>
              </div>
            </div>

            {scanError ? <div className="notice error">{scanError}</div> : null}

            <div className="category-strip" aria-label="Repartition par categorie">
              <button
                className={
                  category === "Toutes" ? "category-chip all-filter active" : "category-chip all-filter"
                }
                type="button"
                onClick={() => setCategory("Toutes")}
              >
                <span>Toutes</span>
                <strong>{numberFormatter.format(visibleApps.length)}</strong>
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
              {sortedApps.map((app) => {
                const meta = localMetadata[app.id];
                const upgrade = matchedUpgrade(app);
                return (
                  <button
                    className={
                      app.id === selectedApp?.id ? "software-row selected" : "software-row"
                    }
                    key={app.id}
                    type="button"
                    onClick={() => setSelectedId(app.id)}
                  >
                    <AppIcon app={app} cache={iconCache} />
                    <span className="software-main">
                      <strong>
                        {meta?.favorite ? <span className="fav-pin" aria-label="Favori">★</span> : null}
                        {app.name}
                        {meta?.hidden ? <em className="badge-hidden">masque</em> : null}
                      </strong>
                      <small>{app.publisher ?? "Editeur inconnu"}</small>
                    </span>
                    <span className="software-meta">
                      <small>{meta?.customCategory || app.category}</small>
                      <strong>
                        {app.version ?? "n/a"}
                        {upgrade ? (
                          <em className="badge-update" title={`Disponible : ${upgrade.availableVersion}`}>
                            MAJ
                          </em>
                        ) : null}
                      </strong>
                    </span>
                  </button>
                );
              })}

              {!isScanning && sortedApps.length === 0 ? (
                <div className="empty-state">
                  {visibilityFilter === "hidden"
                    ? "Aucun logiciel masque pour le moment."
                    : "Aucun logiciel ne correspond aux filtres."}
                </div>
              ) : null}
            </div>
          </section>

          <section className="details-panel" aria-label="Details logiciel">
            {selectedApp ? (
              <DetailsPanel
                app={selectedApp}
                iconCache={iconCache}
                metadata={localMetadata[selectedApp.id]}
                upgrade={matchedUpgrade(selectedApp)}
                editMode={editMode}
                onToggleEdit={() => setEditMode((value) => !value)}
                onLaunch={() => handleLaunch(selectedApp)}
                onOpen={() => handleOpen(selectedApp)}
                onOpenFolder={() => handleOpenFolder(selectedApp)}
                onCopyPath={() => handleCopyPath(selectedApp)}
                onUninstall={() => handleUninstallRequest(selectedApp)}
                onToggleFavorite={() => toggleFavorite(selectedApp)}
                onToggleHidden={() => toggleHidden(selectedApp)}
                onSubmitMetadata={(category, note) =>
                  commitMetadataForm(selectedApp, category, note)
                }
              />
            ) : (
              <div className="empty-state">Selectionne un logiciel pour afficher sa fiche.</div>
            )}
          </section>
        </section>

        {updatesReport && updatesReport.upgrades.length > 0 ? (
          <section className="updates-panel" aria-label="Mises a jour disponibles">
            <header>
              <div>
                <p className="eyebrow">winget upgrade</p>
                <h3>
                  {numberFormatter.format(updatesReport.upgrades.length)} mises a jour detectees
                </h3>
              </div>
              <span className="muted-text">
                {formatScanTime(String(updatesReport.checkedAt))}
              </span>
            </header>
            <div className="upgrade-list">
              {updatesReport.upgrades.slice(0, 10).map((upgrade) => (
                <article className="upgrade-row" key={`${upgrade.id}-${upgrade.availableVersion}`}>
                  <div>
                    <strong>{upgrade.name}</strong>
                    <small>{upgrade.id}</small>
                  </div>
                  <div className="upgrade-versions">
                    <span>{upgrade.currentVersion}</span>
                    <span aria-hidden="true">→</span>
                    <span>{upgrade.availableVersion}</span>
                  </div>
                </article>
              ))}
            </div>
          </section>
        ) : null}

        {isTerminalOpen ? (
          <section className="terminal-panel" aria-label="Terminal integre">
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
                <button
                  className={shell === "cmd" ? "active" : ""}
                  type="button"
                  onClick={() => setShell("cmd")}
                >
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
              <button
                className="primary-button"
                type="button"
                onClick={handleRunCurrent}
                disabled={isRunningCommand}
              >
                {isRunningCommand ? "Execution..." : "Executer"}
              </button>
              <button
                className="secondary-button"
                type="button"
                onClick={handleClearHistory}
              >
                Effacer historique
              </button>
            </div>

            {terminalError ? <div className="notice error">{terminalError}</div> : null}

            <div className="terminal-output" aria-live="polite">
              {terminalHistory.length === 0 ? (
                <p>Aucune commande dans l'historique local.</p>
              ) : (
                terminalHistory.map((entry, index) => (
                  <article
                    className="terminal-entry"
                    key={`${entry.timestamp}-${entry.command}-${index}`}
                  >
                    <header>
                      <strong>
                        {entry.shell} · code {entry.exitCode ?? "n/a"} · {entry.durationMs} ms
                        {entry.risky ? " · sensible" : ""}
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

      {confirmRequest ? (
        <ConfirmDialog
          request={confirmRequest}
          onClose={() => setConfirmRequest(null)}
        />
      ) : null}
    </main>
  );
}

type DetailsPanelProps = {
  app: InstalledApp;
  iconCache: Record<string, string>;
  metadata?: LocalMetadata;
  upgrade?: WingetUpgrade | null;
  editMode: boolean;
  onToggleEdit: () => void;
  onLaunch: () => void;
  onOpen: () => void;
  onOpenFolder: () => void;
  onCopyPath: () => void;
  onUninstall: () => void;
  onToggleFavorite: () => void;
  onToggleHidden: () => void;
  onSubmitMetadata: (category: string, note: string) => Promise<void> | void;
};

function DetailsPanel(props: DetailsPanelProps) {
  const {
    app,
    iconCache,
    metadata,
    upgrade,
    editMode,
    onToggleEdit,
    onLaunch,
    onOpen,
    onOpenFolder,
    onCopyPath,
    onUninstall,
    onToggleFavorite,
    onToggleHidden,
    onSubmitMetadata,
  } = props;

  const [draftCategory, setDraftCategory] = useState<string>(metadata?.customCategory ?? "");
  const [draftNote, setDraftNote] = useState<string>(metadata?.note ?? "");

  useEffect(() => {
    setDraftCategory(metadata?.customCategory ?? "");
    setDraftNote(metadata?.note ?? "");
  }, [metadata, app.id]);

  const canLaunch = Boolean(app.executablePath || app.iconSource);
  const canOpenFolder = Boolean(app.installLocation || app.executablePath || app.iconSource);
  const canCopy = Boolean(app.executablePath || app.installLocation || app.iconSource);
  const canUninstall = Boolean(app.uninstallString);

  const effectiveCategory = metadata?.customCategory || app.category;

  return (
    <>
      <div className="details-header">
        <AppIcon app={app} cache={iconCache} size="large" />
        <div>
          <p className="eyebrow">{effectiveCategory}</p>
          <h3>{app.name}</h3>
          <p>{app.publisher ?? "Editeur inconnu"}</p>
        </div>
      </div>

      <div className="details-actions">
        <button
          type="button"
          className="primary-button"
          onClick={onLaunch}
          disabled={!canLaunch}
          title={canLaunch ? "Lance l'executable detecte" : "Aucun executable connu"}
        >
          Lancer
        </button>
        <button type="button" className="secondary-button" onClick={onOpen} disabled={!canLaunch}>
          Ouvrir
        </button>
        <button
          type="button"
          className="secondary-button"
          onClick={onOpenFolder}
          disabled={!canOpenFolder}
        >
          Ouvrir dossier
        </button>
        <button
          type="button"
          className="secondary-button"
          onClick={onCopyPath}
          disabled={!canCopy}
        >
          Copier chemin
        </button>
        <button
          type="button"
          className="secondary-button danger-action"
          onClick={onUninstall}
          disabled={!canUninstall}
          title={canUninstall ? "Lance la commande de desinstallation" : "Pas de commande connue"}
        >
          Desinstaller
        </button>
      </div>

      <div className="details-actions secondary-row">
        <button
          type="button"
          className={metadata?.favorite ? "secondary-button toggle-on" : "secondary-button"}
          onClick={onToggleFavorite}
        >
          {metadata?.favorite ? "★ Favori" : "☆ Favori"}
        </button>
        <button
          type="button"
          className={metadata?.hidden ? "secondary-button toggle-on" : "secondary-button"}
          onClick={onToggleHidden}
        >
          {metadata?.hidden ? "Ne plus masquer" : "Masquer"}
        </button>
        <button
          type="button"
          className={editMode ? "secondary-button toggle-on" : "secondary-button"}
          onClick={onToggleEdit}
        >
          {editMode ? "Annuler edition" : "Modifier"}
        </button>
      </div>

      {editMode ? (
        <form
          className="metadata-form"
          onSubmit={(event) => {
            event.preventDefault();
            void onSubmitMetadata(draftCategory, draftNote);
          }}
        >
          <label className="search-field">
            <span>Categorie personnalisee</span>
            <input
              value={draftCategory}
              onChange={(event) => setDraftCategory(event.currentTarget.value)}
              placeholder={app.category}
            />
          </label>
          <label className="search-field">
            <span>Note locale</span>
            <textarea
              value={draftNote}
              onChange={(event) => setDraftNote(event.currentTarget.value)}
              rows={3}
            />
          </label>
          <div className="metadata-actions">
            <button type="submit" className="primary-button">
              Enregistrer
            </button>
            <button
              type="button"
              className="secondary-button"
              onClick={() => {
                setDraftCategory("");
                setDraftNote("");
                void onSubmitMetadata("", "");
              }}
            >
              Reinitialiser
            </button>
          </div>
        </form>
      ) : null}

      {metadata?.note ? (
        <div className="note-block">
          <p className="eyebrow">Note locale</p>
          <p>{metadata.note}</p>
        </div>
      ) : null}

      {upgrade ? (
        <div className="update-block">
          <p className="eyebrow">winget</p>
          <p>
            <strong>{upgrade.availableVersion}</strong> disponible (actuel {upgrade.currentVersion})
          </p>
          <small>{upgrade.id}</small>
        </div>
      ) : null}

      <dl className="details-list">
        <Detail label="Version" value={app.version} />
        <Detail label="Installation" value={app.installDate} />
        <Detail
          label="Taille estimee"
          value={
            app.estimatedSizeMb
              ? `${numberFormatter.format(app.estimatedSizeMb)} Mo`
              : undefined
          }
        />
        <Detail label="Source registre" value={app.source} />
        <Detail label="Emplacement" value={app.installLocation} />
        <Detail label="Executable" value={app.executablePath} />
        <Detail label="Icone (registre)" value={app.iconSource} />
        <Detail label="Desinstallation" value={app.uninstallString} />
        <Detail
          label="Mise a jour"
          value={upgrade ? `${upgrade.availableVersion} via winget` : app.updateHint}
        />
      </dl>
    </>
  );
}

type AppIconProps = {
  app: InstalledApp;
  cache: Record<string, string>;
  size?: "compact" | "large";
};

function AppIcon({ app, cache, size = "compact" }: AppIconProps) {
  const base64 = cache[app.id];
  return (
    <span className={`app-icon ${size}`} aria-hidden="true">
      {base64 ? (
        <img src={`data:image/png;base64,${base64}`} alt="" />
      ) : (
        <>
          <svg viewBox="0 0 48 48" role="img">
            <IconShape category={app.category} />
          </svg>
          <span>{app.name.slice(0, 1).toUpperCase()}</span>
        </>
      )}
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

function ConfirmDialog({ request, onClose }: { request: ConfirmRequest; onClose: () => void }) {
  return (
    <div className="confirm-overlay" role="dialog" aria-modal="true">
      <div className={`confirm-dialog ${request.tone ?? ""}`}>
        <h3>{request.title}</h3>
        <p>{request.detail}</p>
        <div className="confirm-actions">
          <button
            type="button"
            className="secondary-button"
            onClick={onClose}
          >
            {request.cancelLabel ?? "Annuler"}
          </button>
          <button
            type="button"
            className={
              request.tone === "danger" ? "primary-button danger-action" : "primary-button"
            }
            onClick={async () => {
              try {
                await request.onConfirm();
              } finally {
                onClose();
              }
            }}
          >
            {request.confirmLabel}
          </button>
        </div>
      </div>
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

function toMessage(error: unknown): string {
  if (error instanceof Error) return error.message;
  if (typeof error === "string") return error;
  return JSON.stringify(error);
}

export default App;
