# givn-Change `run-command`

Vollstaendige Umsetzungsbeschreibung fuer einen Change im Projekt `~/projects/givn`
(v0.4.0, Rust edition 2024, dogfooded mit sich selbst).

Dieser Change ist **Change 0** der kiekste-Roadmap
(siehe [`kiekste-plan-v3.md`](kiekste-plan-v3.md), Abschnitt 10). Er ist
Voraussetzung fuer kiekste-Change 10 und liegt zeitlich davor. Er ist kein
kiekste-Change: er wird in givn implementiert, released, und danach wird der
givn-Pin in kiekste angehoben.

---

## 0. Auftrag in einem Satz

`givn/commands.yaml` erhaelt einen `run:`-Block, der beschreibt, **wie das
gebaute Produkt gestartet wird**. givn scaffoldet ihn, schreibt ihn in
bestehende Projekte zurueck, validiert ihn, rendert ihn in die README und
blockiert `givn archive`, solange er unkonfiguriert ist.

givn startet das Produkt **nicht**. Der `run:`-Block ist eine Deklaration, kein
Ausfuehrungspfad. Ausgefuehrt wird er von kiekste.

---

## 1. Korrektur einer Annahme aus der Planungsdiskussion

Waehrend der Planung galt die Annahme, ein unbekannter Top-Level-Key `run:` in
`givn/commands.yaml` lasse das Parsen wegen `#[serde(deny_unknown_fields)]`
hart fehlschlagen, und kiekste duerfe den Block deshalb erst nach dem
givn-Release schreiben. **Die Annahme ist falsch.** Sie steht nicht in
`kiekste-plan-v3.md`, hat aber dessen Reihenfolge-Argumentation gepraegt.

Nachgeprueft:

- `src/config/manifest.rs:5` traegt `#[serde(deny_unknown_fields)]` auf
  `Manifest`. `Manifest` wird aber nur fuer das **eingebettete Default-Manifest**
  deserialisiert.
- `givn/config.yaml` und `givn/commands.yaml` werden in
  `src/config/overlay.rs:16` (`apply`) als `OverlayDoc` geparst.
  `OverlayDoc` (`src/config/overlay.rs:295-310`) traegt nur
  `#[derive(Debug, serde::Deserialize)]` — **kein** `deny_unknown_fields`.
  Serde ignoriert unbekannte Felder per Default.

Konsequenz: Ein `run:`-Block in `givn/commands.yaml` wird von givn 0.4.0
**still ignoriert**, nicht abgelehnt.

Das entschaerft die Reihenfolge-Kopplung zwischen den Projekten:
kiekste kann seinen `run:`-Block schon vor dem givn-Release in
`givn/commands.yaml` schreiben, ohne das Projekt zu brechen. Er hat bis zum
Pin-Bump nur keine Wirkung. Der givn-Change bleibt trotzdem vorher faellig,
weil ohne ihn nichts den Block validiert oder erzwingt.

---

## 2. Fachliche Anforderung

kiekste muss nach einem gruenen Change wissen:

1. Ist das Produkt ein Server, ein CLI-Werkzeug oder eine Bibliothek?
2. Mit welchem Kommando startet man es?
3. Wenn Server: auf welchem Port, und woran erkennt man, dass es bereit ist?

Diese Information kann nur aus dem Repository kommen — kiekste darf sie nicht
raten und nicht aus dem Agenten herausfragen. Sie gehoert in dieselbe Datei wie
`verify.command`, aus demselben Grund: sie ist projekteigen, versioniert, und
sie ist Teil dessen, was ein Change gruen macht.

---

## 3. Datenmodell

Neu in `src/config/manifest.rs`:

```rust
/// Product start configuration (`givn/commands.yaml`, key `run`).
///
/// givn never executes `command`. It validates the block, renders it into
/// the README, and blocks the archive gate while it is unconfigured.
/// Executing it is the caller's job.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Run {
    /// Command that starts the product. Default is the sentinel.
    #[serde(default = "Run::default_command")]
    pub command: String,

    /// `server`, `cli`, or `library`. `None` means unconfigured.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_type: Option<ProductType>,

    /// Fixed listen port. `product_type: server` only.
    /// Mutually exclusive with `port_env`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,

    /// Name of the environment variable through which the caller passes
    /// the port. `product_type: server` only. Mutually exclusive with `port`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port_env: Option<String>,

    /// HTTP path that answers 2xx once the server is ready to serve.
    /// `product_type: server` only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready_path: Option<String>,

    /// Seconds the caller waits for readiness before giving up.
    #[serde(default = "Run::default_ready_timeout_s")]
    pub ready_timeout_s: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ProductType {
    Server,
    Cli,
    Library,
}

impl Default for Run {
    fn default() -> Self {
        Self {
            command: Self::default_command(),
            product_type: None,
            port: None,
            port_env: None,
            ready_path: None,
            ready_timeout_s: Self::default_ready_timeout_s(),
        }
    }
}

impl Run {
    /// Sentinel: the block exists but was never filled in.
    pub const SENTINEL: &'static str = "givn missing-run-command";

    fn default_command() -> String {
        Self::SENTINEL.to_string()
    }

    fn default_ready_timeout_s() -> u64 {
        60
    }

    /// True while the block is unconfigured.
    pub fn is_unconfigured(&self) -> bool {
        self.command.trim() == Self::SENTINEL || self.product_type.is_none()
    }
}
```

Feld auf `Manifest` (`src/config/manifest.rs:6-33`, neben `verify`):

```rust
    /// Product start configuration.
    #[serde(default)]
    pub run: Run,
```

Feld auf `OverlayDoc` (`src/config/overlay.rs:296`):

```rust
    run: Option<Run>,
```

Merge in `apply` (`src/config/overlay.rs`, direkt hinter dem verify-Merge bei
Zeile 24):

```rust
    // Merge run block.
    if let Some(r) = overlay.run {
        result.run = r;
    }
```

**Merge-Semantik bewusst identisch zu `verify`:** der Overlay ersetzt den
kompletten Block, es gibt kein Feld-fuer-Feld-Merge. Wer im Projekt nur
`run.command` setzt, bekommt fuer alle anderen Felder die Defaults. Das ist
konsistent mit dem bestehenden Verhalten und wird in der Template-Datei
kommentiert.

---

## 4. Betroffene Dateien

| Datei | Aenderung |
|---|---|
| `src/config/manifest.rs` | `Run`, `ProductType`, `Default`-Impl, Feld `run` auf `Manifest` |
| `src/config/overlay.rs` | Feld `run` auf `OverlayDoc`, Merge in `apply` |
| `assets/commands.yaml.tmpl` | dokumentierter `run:`-Block mit Sentinel |
| `assets/addons/readme/README.md.tmpl` | `{{RUN_BLOCK}}` unter `## Quick start`, Todo-Text angepasst |
| `src/cli/init.rs` | keine Aenderung noetig (Template wird als Ganzes geschrieben, Zeile 71) |
| `src/cli/run_block.rs` | **neu** — Validierung und Findings |
| `src/cli/mod.rs` | `pub mod run_block;` |
| `src/cli/missing_run_command.rs` | **neu** — Sentinel-Kommando |
| `src/main.rs` | verstecktes Subkommando `missing-run-command` (Muster: Zeile 138/318) |
| `src/cli/readme.rs` | Marker-Konstanten, Render, Refresh, Damage-Check fuer den run-Block |
| `src/cli/check.rs` | Validierung im `"review"`-Arm (Zeile 109) |
| `src/cli/archive.rs` | Gate vor der README-Gate (Zeile 91) |
| `src/cli/migration.rs` | `givn/commands.yaml` in den Managed-Upgrade-Pfad aufnehmen |
| `src/cli/upgrade.rs` | `backfill_commands` — `run:`-Scaffold anhaengen, wenn Key fehlt |
| `givn/changes/run-command/specs/run-command.feature` | Delta-Specs |
| `tests/steps/run_command_steps.rs`, `tests/steps/run_command_e2e_steps.rs` | Step-Definitions |
| `tests/steps/mod.rs` | Registrierung |
| `docs/arc42/…`, `docs/adr/…` | ADR + arc42-Kapitel (arc42-Addon ist in givn aktiv) |

---

## 5. Verhalten je Eintrittspunkt

### 5.1 `givn init`

`src/cli/init.rs:71` schreibt `assets/commands.yaml.tmpl` unveraendert. Der
`run:`-Block kommt also automatisch mit, sobald das Template ihn enthaelt.
Keine Code-Aenderung in `init.rs`.

Anzuhaengen an `assets/commands.yaml.tmpl`:

```yaml
# --- Product start ------------------------------------------------------------
# How the built product is started. givn NEVER runs this command — it validates
# the block, documents it in the README, and blocks `givn archive` while it is
# unconfigured. Automation around givn (CI, a preview host) reads it and starts
# the product.
#
#   product_type: server   long-running process that listens on a port
#   product_type: cli      one-shot command-line tool
#   product_type: library  consumed as a dependency; nothing to start
#
# For `server`, set exactly one of `port` (fixed) or `port_env` (the caller
# passes the port through this environment variable), plus `ready_path`.
#
# Examples:
#   run:
#     command: "./run.sh"
#     product_type: server
#     port_env: "PORT"
#     ready_path: "/healthz"
#     ready_timeout_s: 60
#
#   run:
#     command: "cargo run --release -- --help"
#     product_type: cli
#
# Overlay semantics match `verify:` — this block replaces the default as a
# whole, so list every field you need.
run:
  command: "givn missing-run-command"   # replace with the real start command
  # product_type: server
  # port_env: "PORT"
  # ready_path: "/healthz"
  # ready_timeout_s: 60
```

### 5.2 `givn upgrade` — Backfill in bestehende Projekte

Heute fasst `givn upgrade` ausschliesslich `givn/config.yaml` und die
generierten Agent-Dateien an (`src/cli/upgrade.rs:14`,
`src/cli/migration.rs:324`). `givn/commands.yaml` wird nie angefasst.

Erweiterung, minimal und im bestehenden Trichter:

1. Neue Funktion in `src/cli/upgrade.rs`:

```rust
/// Append the `run:` scaffold to an existing commands file when the key is
/// absent. Returns `None` when the file already has an active `run:` key
/// (idempotent), so a repeated upgrade produces no diff.
pub fn backfill_commands(existing: &str) -> Option<String> {
    let has_run = existing.lines().any(|line| {
        !line.starts_with(' ')
            && !line.starts_with('\t')
            && line.split(':').next().unwrap_or(line).trim() == "run"
    });
    if has_run {
        return None;
    }
    let mut out = existing.to_string();
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out.push('\n');
    out.push_str(RUN_SCAFFOLD);
    Some(out)
}
```

`RUN_SCAFFOLD` ist derselbe Text wie in Abschnitt 5.1, ausgelagert nach
`assets/run.yaml.tmpl` und per `include_str!` eingebunden, damit Template und
Backfill nicht auseinanderlaufen. `assets/commands.yaml.tmpl` bindet ihn
nicht ein (Assets sind statisch); stattdessen prueft ein Unit-Test, dass
`commands.yaml.tmpl` den Inhalt von `run.yaml.tmpl` woertlich enthaelt.

2. In `src/cli/migration.rs::prepare_managed_upgrade` (Zeile 324):
   - `commands_path` (`givn/commands.yaml`) in `FileSnapshot::capture`
     aufnehmen (Zeile 335), damit ein fehlgeschlagener Upgrade zurueckrollt.
   - In `apply_managed_upgrade` (Zeile 357) nach dem Schreiben der
     `config.yaml` den Backfill anwenden, falls die Datei existiert.
   - In `commit_managed_upgrade` (Zeile 373) `commands_path` an `paths`
     anhaengen. `stage_paths` filtert nicht existierende Pfade bereits selbst.
   - `has_staged_changes` entscheidet weiterhin, ob ein Commit entsteht. Ein
     Projekt mit bereits vorhandenem `run:` erzeugt keinen leeren Commit.

3. Ausgabe: wenn der Backfill gegriffen hat, zusaetzlich zur bestehenden
   Zeile ausgeben:

```
  wrote    run: scaffold to givn/commands.yaml — fill it in; `givn archive` is blocked until you do
```

**Kein Migrations-Bundle.** Der Katalog in `src/cli/migration.rs:26` ist fuer
Aenderungen gedacht, die ein Agent im Projekt nacharbeiten muss. Hier reicht
der Managed-Refresh: das Feld hat ein `#[serde(default)]`, der Overlay ist
tolerant, nichts an bestehenden Projekten bricht. Der Zwang entsteht erst am
Archive-Gate, und dort mit einer Meldung, die genau sagt, was zu tun ist.

### 5.3 `givn check`

Neues Modul `src/cli/run_block.rs`:

```rust
use crate::config::manifest::{Manifest, ProductType, Run};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunFindingKind {
    Unconfigured,
    MissingField,
    ForbiddenField,
    NoOpCommand,
}

pub struct RunFinding {
    pub kind: RunFindingKind,
    pub message: String,
}

/// Validate the `run:` block. An empty result means the block is complete
/// and internally consistent.
pub fn check_run(manifest: &Manifest) -> Vec<RunFinding> { /* Abschnitt 6 */ }

pub fn label(kind: RunFindingKind) -> &'static str { /* "UNCF ", "MISS ", "FORB ", "NOOP " */ }
```

Einhaengen in `src/cli/check.rs`, im `"review"`-Arm (Zeile 109), direkt hinter
`run_integrity_hook` (Zeile 115):

```rust
            let run_findings = crate::cli::run_block::check_run(manifest);
            if !run_findings.is_empty() {
                for f in &run_findings {
                    println!("  [{}] {}", crate::cli::run_block::label(f.kind), f.message);
                }
                anyhow::bail!(
                    "run block incomplete — configure `run:` in givn/commands.yaml"
                );
            }
            println!("  ✓ run block passed");
```

Begruendung fuer `review` und nicht `tasks`: `tasks` prueft, ob der Code tut,
was die Specs sagen. `review` prueft, ob der Change abgabefertig ist. Ein
fehlender Startbefehl ist ein Abgabemangel, kein Testfehler.

### 5.4 README

In `src/cli/readme.rs` analog zum Development-Block (Zeile 36 ff.):

```rust
pub const RUN_BLOCK_BEGIN: &str = "<!-- givn:begin:run -->";
pub const RUN_BLOCK_END: &str = "<!-- givn:end:run -->";
```

`render_run_block()` (Muster: `render_development_block`, Zeile 210):

- Block unkonfiguriert → Rumpf ist **genau eine** Todo-Zeile:

```markdown
<!-- givn:todo: set run.command and run.product_type in givn/commands.yaml -->
```

  Damit greift der bestehende `OpenPlaceholder`-Pfad in `check_readme`
  (Zeile 60-75) ohne neuen Code, und `givn archive` blockiert bereits ueber
  das README-Gate.

- `product_type: server`:

```markdown
Start the server:

```sh
<command>
```

It listens on <port | $port_env> and reports readiness at `<ready_path>`
within <ready_timeout_s> seconds.
```

- `product_type: cli`:

```markdown
Run the tool:

```sh
<command>
```
```

- `product_type: library`: Rumpf ist eine Zeile —
  `This project is a library; there is nothing to start.`

`refresh_run_block()` spiegelt `refresh_development_block` (Zeile 268):
fehlt das Markerpaar, passiert nichts; ist genau ein Marker da, meldet
`check_readme` `DamagedBlock` (Muster: Zeile 129-139).

Template `assets/addons/readme/README.md.tmpl`:

```markdown
## Quick start

<!-- givn:todo: Exact commands to clone and build the project. -->

{{RUN_BLOCK}}
```

Der bestehende Todo-Text wird von `Exact commands to clone, build, and run the
project.` auf `Exact commands to clone and build the project.` gekuerzt — das
Starten steht jetzt im Managed-Block. Das Praefix `Exact commands to clone`
bleibt erhalten, damit das bestehende Szenario in
`givn/specs/readme-enforcement/readme-enforcement.feature`
(`stdout mentions "Exact commands to clone"`) gruen bleibt.

`scaffold_readme` (Zeile 185) ersetzt `{{RUN_BLOCK}}` zusaetzlich zu
`{{DEVELOPMENT_BLOCK}}`.

### 5.5 `givn archive`

In `src/cli/archive.rs` **vor** dem README-Gate (Zeile 91) einsetzen, also vor
jeder Mutation, damit kein Rollback noetig ist:

```rust
    // Run-block gate: the product must document how it is started. Runs
    // independently of the readme addon, so a project without a README is
    // still held to it.
    let run_findings = crate::cli::run_block::check_run(&manifest);
    if !run_findings.is_empty() {
        println!("archive blocked: run block check found {} finding(s):", run_findings.len());
        for f in &run_findings {
            println!("  [{}] {}", crate::cli::run_block::label(f.kind), f.message);
        }
        bail!(
            "archive blocked: configure the `run:` block in givn/commands.yaml — \
             see the run block report above"
        );
    }
    println!("✓ Run block gate passed");
```

Bewusst **nicht** vom `readme`-Addon abhaengig: das README-Gate ist die
Dokumentationspflicht, das Run-Gate die Deklarationspflicht. Ein Projekt mit
abgeschaltetem readme-Addon muss den Startbefehl trotzdem deklarieren.

### 5.6 Sentinel-Kommando `givn missing-run-command`

`src/cli/missing_run_command.rs`, Muster: `src/cli/missing_testrunner.rs`.
Gibt die Konfigurationshilfe auf stderr aus und beendet sich mit Exit-Code 1.

Registrierung in `src/main.rs` nach dem Muster Zeile 138/318:

```rust
    #[command(name = "missing-run-command", hide = true)]
    MissingRunCommand,
```

Begruendung, obwohl givn den Befehl nie selbst ausfuehrt: der Sentinel-String
landet im README-Block und in `commands.yaml`. Jemand kopiert ihn und fuehrt
ihn aus. Dann soll er eine Anleitung sehen und keinen `command not found`.

---

## 6. Validierungsregeln

`check_run` liefert Findings nach diesen Regeln. Alle Pruefungen sind
deterministische String- und Options-Pruefungen, keine Heuristik.

| Nr | Bedingung | Kind | Meldung (Kern) |
|---|---|---|---|
| R1 | `command.trim()` ist leer oder gleich `Run::SENTINEL` | `Unconfigured` | `run.command is unset — set the command that starts the product` |
| R2 | `product_type` ist `None` | `Unconfigured` | `run.product_type is unset — one of server, cli, library` |
| R3 | `product_type: server` und `ready_path` ist `None` | `MissingField` | `product_type server requires run.ready_path` |
| R4 | `product_type: server` und weder `port` noch `port_env` gesetzt | `MissingField` | `product_type server requires exactly one of run.port or run.port_env` |
| R5 | `product_type: server` und **beide** gesetzt | `ForbiddenField` | `run.port and run.port_env are mutually exclusive` |
| R6 | `product_type: cli` oder `library` und eines von `port`, `port_env`, `ready_path` gesetzt | `ForbiddenField` | `run.<field> applies to product_type server only` |
| R7 | `product_type: server` und `ready_timeout_s == 0` | `MissingField` | `run.ready_timeout_s must be greater than zero` |
| R8 | `product_type: server` und `ready_path` beginnt nicht mit `/` | `MissingField` | `run.ready_path must be an absolute path, e.g. /healthz` |
| R9 | erstes Token von `command` steht auf der No-Op-Liste | `NoOpCommand` | `run.command '<cmd>' does nothing — it cannot start the product` |
| R10 | `product_type: library` und `command` ist gesetzt (nicht Sentinel) | — | **kein** Finding; erlaubt, wird in der README gerendert |

Bei R2 werden R3-R8 uebersprungen (ohne Typ keine typabhaengige Pruefung),
R9 laeuft trotzdem.

Bei `product_type: library` gilt R1 **nicht**: eine Bibliothek darf den
Sentinel behalten. Praezisierung von R1: R1 feuert nur, wenn
`product_type` `None`, `Server` oder `Cli` ist.

No-Op-Liste fuer R9 (Vergleich auf dem ersten whitespace-getrennten Token des
getrimmten Kommandos):

```rust
const NO_OP_COMMANDS: &[&str] = &["true", ":", "echo", "printf", "sleep", "cat", "test", "["];
```

Zusaetzlich feuert R9 bei `command.trim() == "exit 0"`.

---

## 7. Anti-Gaming

Der einzige realistische Umgehungsweg ist, `product_type` auf `library` zu
setzen und damit alle Serveranforderungen loszuwerden. Dagegen gibt es in givn
**bewusst keine** Gegenmassnahme:

- givn kennt das Produkt nicht und kann nicht entscheiden, ob es ein Server ist.
- Eine Heuristik (Port im Code gefunden, also Server) waere falschpositiv und
  wuerde ein blockierendes Gate auf Raten stellen.

Stattdessen greifen zwei Kontrollen ausserhalb von givn:

1. Der `run:`-Block steht versioniert im Repository und wird in der README
   gerendert. Eine falsche Deklaration ist im Diff sichtbar.
2. kiekste macht einen Change mit `product_type: server` erst dann fertig, wenn
   eine Project Preview bereit ist (`kiekste-plan-v3.md`, Abschnitt 4.5/4.6).
   Wer auf `library` faelscht, umgeht das Preview-Gate und produziert damit
   einen offensichtlich falschen Eintrag, keinen gruenen Fehlstart.

Diese Abgrenzung folgt dem Vorbild von `run_verify_e2e_hook`
(`src/hooks.rs:132`): dort prueft givn nur, was es deterministisch belegen kann
(`@e2e`-Szenarien existieren, `e2e_command` ist unkonfiguriert), und raet nicht.

---

## 8. Gherkin-Specs

Datei: `givn/changes/run-command/specs/run-command.feature`.
Kein Capability-Tag noetig — `capability_tag`
(`src/gherkin/delta.rs:27`) faellt auf den Dateinamen zurueck, der Merge landet
also in `givn/specs/run-command/run-command.feature`.

```gherkin
Feature: Run command declaration

  @e2e
  Scenario: Scaffolding ships an unconfigured run block
    Given  a project root without a givn/ directory
    When  the user runs `givn init`
    Then  the file "givn/commands.yaml" exists
    And  the commands file has run.command set to the sentinel
    And  the README has the run managed block
    And  the README has the run open placeholder

  Scenario: The check reports an unconfigured run block
    Given  a project root with givn/ initialised
    And  the commands file has run.command set to the sentinel
    When  the run block is checked
    Then  the check reports "run.command is unset"
    And  the check reports "run.product_type is unset"

  Scenario: A server without a readiness path is incomplete
    Given  a commands file with product_type "server" and port 8080
    When  the run block is checked
    Then  the check reports "requires run.ready_path"

  Scenario: A server needs exactly one port source
    Given  a commands file with product_type "server", port 8080 and port_env "PORT"
    When  the run block is checked
    Then  the check reports "mutually exclusive"

  Scenario: A server with neither port source is incomplete
    Given  a commands file with product_type "server" and no port source
    When  the run block is checked
    Then  the check reports "exactly one of run.port or run.port_env"

  Scenario: A relative readiness path is rejected
    Given  a commands file with product_type "server", port_env "PORT" and ready_path "healthz"
    When  the run block is checked
    Then  the check reports "must be an absolute path"

  Scenario: Server fields are rejected for a CLI product
    Given  a commands file with product_type "cli" and port 8080
    When  the run block is checked
    Then  the check reports "product_type server only"

  Scenario: A no-op start command is rejected
    Given  a commands file with product_type "cli" and command "true"
    When  the run block is checked
    Then  the check reports "does nothing"

  Scenario: A library may leave the command at the sentinel
    Given  a commands file with product_type "library" and no command
    When  the run block is checked
    Then  the check reports no findings

  Scenario: A complete server block passes
    Given  a commands file with a complete server run block
    When  the run block is checked
    Then  the check reports no findings

  @e2e
  Scenario: Archive is blocked while the run block is unconfigured
    Given  a project root with givn/ initialised
    And  a change ready to archive
    And  the commands file has run.command set to the sentinel
    When  the user runs `givn archive`
    Then  the command exits non-zero
    And  stdout mentions "run block check found"
    And  the change is still active

  @e2e
  Scenario: Archive passes once the run block is complete
    Given  a project root with givn/ initialised
    And  a change ready to archive
    And  a commands file with a complete server run block
    When  the user runs `givn archive`
    Then  the command exits zero

  @e2e
  Scenario: Upgrade backfills the run block into an existing project
    Given  a project root with givn/ initialised
    And  the commands file has no run block
    When  the user runs `givn upgrade`
    Then  the file "givn/commands.yaml" contains "run:"
    And  stdout mentions "fill it in"

  Scenario: The backfill is idempotent
    Given  a commands file that already has a run block
    When  the backfill runs
    Then  the commands file is unchanged

  Scenario: The README renders a complete server block
    Given  a commands file with a complete server run block
    When  the run managed block is rendered
    Then  the rendered block mentions the start command
    And  the rendered block mentions the readiness path
    And  the rendered block has no open placeholder

  Scenario: The README run block is reported when damaged
    Given  a project root with givn/ initialised
    And  a README with a broken run managed block
    When  the user runs `givn readme check`
    Then  the command exits non-zero
    And  stdout mentions "run managed block"

  Scenario: An unknown product type is rejected at parse time
    Given  a commands file with product_type "web"
    When  the effective manifest is loaded
    Then  loading fails mentioning "product_type"
```

**Regressionsschutz, der nicht uebersehen werden darf:** die Aenderung am
README-Template beruehrt das bestehende Szenario
`The check names every open placeholder` in
`givn/specs/readme-enforcement/readme-enforcement.feature`. Es muss ohne
Anpassung gruen bleiben. Faellt es um, ist der Todo-Text falsch gekuerzt worden.

---

## 9. Step-Definitions

- `tests/steps/run_command_steps.rs` — die nicht-`@e2e`-Szenarien. Arbeiten
  direkt auf `Manifest`/`Run` und `check_run`, ohne Prozessstart.
- `tests/steps/run_command_e2e_steps.rs` — die `@e2e`-Szenarien. Rufen die
  `givn`-Binary in einem temporaeren Projektverzeichnis auf, Muster:
  `tests/steps/readme_enforcement_e2e_steps.rs`.
- Registrierung in `tests/steps/mod.rs`.

Der Step `a change ready to archive` existiert in aehnlicher Form bereits in
den Archive-bezogenen Step-Dateien; wiederverwenden statt neu bauen.
`repository_ownership_fixtures_support.rs` liefert das Muster fuer
gemeinsam genutzte Fixtures, falls ein neuer Helfer noetig wird.

---

## 10. Nichtziele

- givn startet das Produkt nicht, prueft keine Erreichbarkeit, oeffnet keinen
  Port und macht keinen HTTP-Request.
- Kein Auslesen von `Dockerfile`, `docker-compose.yml`, `Procfile` oder
  Framework-Konventionen, um `run:` zu erraten.
- Keine Ableitung von `product_type` aus `Cargo.toml` (`[[bin]]` vs `[lib]`)
  oder `package.json`. Das waere Raten mit blockierender Wirkung.
- Kein neues Artefakt im Lifecycle-DAG. Der `run:`-Block ist Konfiguration,
  kein Artefakt.
- Kein Migrations-Bundle (Begruendung in 5.2).

---

## 11. Umsetzung als givn-Change

givn dogfooded sich selbst, also laeuft der Change durch die eigene Kette.

```sh
cd ~/projects/givn
givn new run-command
givn status --change run-command
```

Artefaktkette: `proposal` → `specs` → `design` → `design-review` → `tasks` →
`implement` → `review` → `archive`. Hooks: `tasks.post: [verify]`,
`review.post: [verify, verify-e2e]`. Das arc42-Addon ist aktiv, `arc42-docs`
steht in `apply.requires` — der Change ist ohne arc42-Kapitel nicht
archivierbar.

Task-Reihenfolge in `tasks.md`, so geschnitten, dass jeder Schritt fuer sich
uebersetzt und die Suite gruen laesst:

1. `Run`, `ProductType`, `Default`-Impl, Feld auf `Manifest`, Feld auf
   `OverlayDoc`, Merge in `apply`. Keine Verhaltensaenderung nach aussen.
2. `assets/run.yaml.tmpl` anlegen, in `assets/commands.yaml.tmpl` einfuegen,
   Unit-Test auf Woertlichkeit.
3. `src/cli/run_block.rs` mit `check_run` und den Regeln R1-R10.
4. `src/cli/missing_run_command.rs` und Registrierung in `src/main.rs`.
5. README: Marker, `render_run_block`, `refresh_run_block`, Damage-Check,
   Template-Aenderung, `scaffold_readme`.
6. `givn check review`: Einhaengen der Validierung.
7. `givn archive`: Gate vor der README-Gate.
8. `backfill_commands` plus Einhaengen in den Managed-Upgrade-Pfad.
9. ADR und arc42-Kapitel.

Als Stacked PR: ein PR pro Task-Gruppe, Reihenfolge 1 → 9, jeder PR auf dem
vorigen. Schritte 1-2 sind additiv und ohne Aussenwirkung, ab Schritt 6 wird
das Gate scharf. Wer die Reihenfolge dreht, blockiert sich das eigene
`givn archive` mitten im Change.

**Arbeitsregel aus `kiekste-plan-v3.md`, Plan-Gate 12:** vor jedem `givn
archive` erst `main` in den Change-Branch mergen, nie umgekehrt und nie
rebasen. Sonst laeuft das Post-Merge-Verify (`src/cli/archive.rs:183`) gegen
einen Stand, den es so auf `main` nie gab.

Nach dem Merge:

1. Release der neuen givn-Version.
2. In kiekste den givn-Pin anheben.
3. In kiekste `givn upgrade` laufen lassen — der Backfill schreibt den
   `run:`-Scaffold in `/home/buster/projects/kiekste/givn/commands.yaml`.
4. Den Block in kiekste ausfuellen: kiekste ist selbst ein Server.
5. Erst danach kiekste-Change 10 beginnen.

---

## 12. Akzeptanzkriterien

1. Ein frisch mit `givn init` erzeugtes Projekt hat einen `run:`-Block mit
   Sentinel in `givn/commands.yaml` und einen offenen Platzhalter im
   README-Run-Block.
2. `givn archive` bricht in diesem Projekt mit einer Meldung ab, die
   `givn/commands.yaml` und den fehlenden `run:`-Block nennt.
3. Nach dem Ausfuellen mit `product_type: server`, `command`, `port_env` und
   `ready_path` laeuft `givn archive` durch, und der README-Run-Block nennt
   Startbefehl und Readiness-Pfad.
4. `product_type: library` ohne `command` ist gueltig und blockiert nichts.
5. `product_type: web` laesst `load_effective` mit einer Meldung
   fehlschlagen, die `product_type` nennt.
6. `givn upgrade` in einem bestehenden Projekt ohne `run:` schreibt den
   Scaffold, committet ihn zusammen mit den anderen Managed-Files und sagt in
   der Ausgabe, dass er auszufuellen ist.
7. Ein zweites `givn upgrade` erzeugt keinen weiteren Commit.
8. `givn/specs/readme-enforcement/readme-enforcement.feature` bleibt ohne
   Anpassung gruen.
9. `verify` und `verify-e2e` sind gruen, das Coverage-Gate ebenfalls.
10. Der Change ist archiviert, die Delta-Specs stehen unter
    `givn/specs/run-command/`.

---

## 13. Entscheidungen, die ich getroffen habe und die revidierbar sind

| Nr | Entscheidung | Alternative |
|---|---|---|
| E1 | `run:` in `givn/commands.yaml`, nicht in `givn/config.yaml` | `config.yaml` — aber dort steht Struktur, nicht Kommandos |
| E2 | Archive-Gate unabhaengig vom `readme`-Addon | nur ueber die README erzwingen; dann entkommt ein Projekt ohne README-Addon |
| E3 | Validierung in `check review`, nicht in `check tasks` | `tasks` — wuerde den Startbefehl waehrend der Implementierung erzwingen |
| E4 | Kein Migrations-Bundle, Backfill im Managed-Refresh | Bundle — schwergewichtig, erzeugt einen Change, den ein Agent abarbeiten muss |
| E5 | Verstecktes `missing-run-command`, obwohl givn es nie ausfuehrt | weglassen; dann laeuft ein kopierter Sentinel ins Leere |
| E6 | No-Op-Denylist als einzige Anti-Gaming-Massnahme | Heuristik auf dem Quellcode — raet und blockiert dabei |
| E7 | `ready_timeout_s` Default 60 | 30 oder 120; 60 deckt uebliche Server-Starts ab, ohne einen Haenger lange zu kaschieren |
