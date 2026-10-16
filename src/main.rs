mod cli;
mod cobertura;
mod config;
mod error;
mod fossil;
mod gherkin;
mod git;
mod history;
mod hooks;
mod integrity_hook;
mod overlap;
mod proof;
mod scm;
mod skills;
mod spec;
mod state;
mod subprocess;

use std::path::PathBuf;

use clap::{CommandFactory, FromArgMatches, Parser, Subcommand, ValueEnum};
use clap_complete::{Shell, generate};

/// givn — from Given to done.
///
/// A single-binary, Gherkin-first spec-driven workflow tool.
#[derive(Parser)]
#[command(name = "givn", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scaffold givn/ and generate skills.
    Init {
        /// Overwrite existing files.
        #[arg(long)]
        force: bool,
        /// Do not enable addons (arc42, absolute_mode).
        #[arg(long)]
        no_addons: bool,
        /// Agents to install skills/commands to, comma-separated
        /// (e.g. `opencode,claude`, `omp`, or `all`). Skips the interactive
        /// prompt. Only takes effect on fresh config creation (or with
        /// `--force`), same as the prompt it replaces.
        #[arg(long, value_delimiter = ',')]
        agents: Vec<String>,
    },
    /// Create one attributed revision through the selected SCM backend.
    ///
    /// Records the named paths (or every pending change) and commits them.
    /// The managed attribution trailer is appended when the harness exports
    /// attribution context (GIVN_ATTRIBUTION_ACTIVE); otherwise the message
    /// is committed unchanged. Fails when the checkout has no pending
    /// changes or no backend owns the project.
    Commit {
        /// Revision message.
        #[arg(short, long)]
        message: String,
        /// Paths to record (defaults to every pending change).
        paths: Vec<String>,
    },
    /// Create a new change scaffold.
    New {
        /// Change identifier (kebab-case).
        id: String,
        /// Artifact ids to skip (comma-separated, e.g. `proposal,design`).
        /// Writes `.givn-skip` so `givn status` shows them as ⊘ skipped.
        #[arg(long, value_delimiter = ',')]
        skip: Vec<String>,
    },
    /// Show instructions. With an artifact id, prints that artifact's
    /// writing instructions (unchanged). Without one, prints context-aware
    /// generic guidance: what to run next, given the project's current state.
    Instructions {
        /// Artifact id (e.g. proposal, specs, design, tasks, review).
        /// Omit for context-aware generic guidance on what to do next.
        artifact: Option<String>,
        /// Change identifier.
        #[arg(long)]
        change: Option<String>,
        /// Output as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Static check of .feature files (syntax, @wip, delta tags). Runs NO tests.
    Lint {
        /// Change identifier — limit check to givn/<id>/specs/.
        #[arg(long)]
        change: Option<String>,
        /// Paths to check (default: all .feature files under givn/).
        paths: Vec<String>,
        /// Emit findings as a JSON array on stdout.
        #[arg(long)]
        json: bool,
    },
    /// Run an artifact's post hooks (e.g. the test suite via verify.command).
    Check {
        /// Artifact id whose hooks to run (default: all artifacts with output).
        artifact: Option<String>,
        /// Change identifier.
        #[arg(long)]
        change: Option<String>,
    },
    /// Show lifecycle state: artifact checklist, current stage, next required and optional steps.
    Status {
        /// Change identifier (default: all active changes).
        #[arg(long)]
        change: Option<String>,
        /// Output as JSON (machine-readable, for use by commands and scripts).
        #[arg(long)]
        json: bool,
    },
    /// Show the artifact workflow graph: nodes, dependency edges, optional/
    /// gates-archive flags. Without --change, prints the general workflow
    /// from the manifest. With --change, overlays live status per artifact.
    Graph {
        /// Change identifier (default: general workflow only).
        #[arg(long)]
        change: Option<String>,
        /// Output as JSON (machine-readable, for use by commands and scripts).
        #[arg(long)]
        json: bool,
    },
    /// Read the development record (ADR-073): one scope's recorded entries —
    /// or, with the `ingest` subcommand, the agent intake that appends one
    /// reported payload's events.
    ///
    /// Reading never records: the read view runs without a recorder, so the
    /// store keeps exactly the lines it had. The intake records normally.
    History {
        /// The intake subcommand; without it the command is the read view.
        #[command(subcommand)]
        command: Option<HistoryCommands>,
        /// Change identifier — limit the read to that change.
        #[arg(long)]
        change: Option<String>,
        /// Ideation topic identifier — limit the read to that topic.
        #[arg(long)]
        topic: Option<String>,
        /// Emit the record as JSON (the machine-readable view).
        #[arg(long)]
        json: bool,
    },
    /// Render the development report (ADR-073) into one self-contained HTML
    /// document and print its path.
    ///
    /// Reading never records: this command runs without a recorder, so the
    /// store keeps exactly the lines it had.
    Timeline {
        /// Change identifier — report that change.
        #[arg(long)]
        change: Option<String>,
        /// Ideation topic identifier — report that topic.
        #[arg(long)]
        topic: Option<String>,
        /// Write the report here (default: `givn/history/reports/<scope>.html`).
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Archive a completed change (gated: runs hooks of apply.requires artifacts).
    Archive {
        /// Change identifier.
        #[arg(long)]
        change: String,
        /// Stream every gate command's output with [HH:MM:SS] prefixes, so a
        /// hang is visible at the exact line it stopped on.
        #[arg(long)]
        verbose: bool,
    },
    /// Write an embedded default facet into the project for editing.
    Eject {
        /// Target to eject, e.g. "design:template", "skill:givn-propose", "manifest".
        target: String,
        /// Overwrite if the target already exists.
        #[arg(long)]
        force: bool,
    },
    /// Manage optional addon artifact sets (arc42, etc.).
    Addons {
        #[command(subcommand)]
        command: AddonsCommands,
    },
    /// README enforcement: deterministic completeness check.
    Readme {
        #[command(subcommand)]
        command: ReadmeCommands,
    },
    /// Regenerate skills and agent commands.
    Skills {
        #[command(subcommand)]
        command: SkillsCommands,
    },
    /// Work with the permanent spec corpus: coverage inventory, duplicate
    /// report, tree overview, lexical scenario search, and proposal routing.
    Spec {
        #[command(subcommand)]
        command: SpecCommands,
    },
    /// Re-render managed config scaffold, preserve user overrides, bump version.
    Upgrade,
    /// Start the product declared in the run block of givn/commands.yaml.
    Run {
        /// Caller-provided port, handed to the product through run.port_env.
        #[arg(long)]
        port: Option<u16>,
        /// Print one sh-eval-able start line without executing.
        #[arg(long)]
        print: bool,
    },
    /// Sentinel default verify.command: prints setup guidance and exits non-zero.
    #[command(name = "missing-testrunner", hide = true)]
    MissingTestrunner,

    /// Sentinel default run.command: prints setup guidance and exits non-zero.
    #[command(name = "missing-run-command", hide = true)]
    MissingRunCommand,

    /// Generate shell completion script and print it to stdout.
    ///
    /// Add to your shell profile:
    ///   bash:  eval "$(givn completions bash)"
    ///   zsh:   eval "$(givn completions zsh)"
    ///   fish:  givn completions fish | source
    #[command(name = "completions")]
    Completions {
        /// Shell to generate completions for.
        shell: Shell,
    },
    /// Internal commit-attribution helpers used by generated integrations.
    #[command(hide = true)]
    Attribution {
        #[command(subcommand)]
        command: AttributionCommands,
    },
}

#[derive(Subcommand)]
enum AttributionCommands {
    /// Best-effort stamp of a Git commit message file.
    Stamp {
        #[arg(long)]
        message_file: PathBuf,
    },
    /// Convert a Claude Code Bash hook payload into an optional command update.
    ClaudePreToolUse,
}

/// The `history` command's subcommands: the read view is the command itself.
#[derive(Subcommand)]
enum HistoryCommands {
    /// Read one reported payload from stdin (the agent intake) and append the
    /// events it accepts.
    Ingest,
    /// Map one raw harness hook payload from stdin (the process-per-hook
    /// entry point: the harness runs every hook as its own process, so the
    /// generated shim pipes its payload here).
    HarnessHook {
        /// The harness whose payload is read (`claude`).
        #[arg(long)]
        harness: String,
    },
}

#[derive(Subcommand)]
enum AddonsCommands {
    /// List all available addons and their enabled/disabled status.
    List,
    /// Enable an addon in givn/config.yaml.
    Enable {
        /// Addon name (e.g. arc42).
        name: String,
    },
    /// Disable an addon in givn/config.yaml.
    Disable {
        /// Addon name (e.g. arc42).
        name: String,
    },
}

#[derive(Subcommand)]
enum SkillsCommands {
    /// Regenerate skills and agent commands from current templates.
    Sync,
    /// Manage the configured skill/command install targets.
    Targets {
        #[command(subcommand)]
        command: TargetsCommands,
    },
}

#[derive(Subcommand)]
enum TargetsCommands {
    /// Add agent target(s) to the existing selection in givn/config.yaml
    /// and sync. Only the `skills: targets:` line is touched.
    Add {
        /// Agents to add, comma-separated (e.g. `claude,agents`, or `all`).
        #[arg(value_delimiter = ',')]
        agents: Vec<String>,
    },
}

#[derive(Subcommand)]
enum ReadmeCommands {
    /// Report open placeholders, remnants, and managed-block damage; exit 1
    /// while findings remain.
    Check,
}

#[derive(Subcommand)]
enum SpecCommands {
    /// Emit the machine-readable specification coverage inventory.
    Coverage {
        /// Compare the current inventory against a prior JSON snapshot.
        #[arg(long)]
        baseline: Option<PathBuf>,
        /// Output JSON instead of a summary.
        #[arg(long)]
        format: Option<String>,
    },
    /// Report duplicate and related permanent scenarios.
    Duplicates {
        /// Output format.
        #[arg(long, value_enum, default_value_t = SpecFormat::Text)]
        format: SpecFormat,
    },
    /// Walk the permanent spec corpus by group: Actor/Goal per group,
    /// scenario and @e2e counts per capability, phrasing reuse ratio per
    /// group. Deterministic — no embeddings, no network, no score.
    Tree {
        /// Output format.
        #[arg(long, value_enum, default_value_t = SpecFormat::Text)]
        format: SpecFormat,
    },
    /// Lexical (not semantic) search over permanent scenario titles and
    /// step text — parsed from the AST, so docstring payload cannot create
    /// phantom hits. No ranking model, no fuzzy matching.
    Find {
        /// Substring to search for (case-insensitive).
        query: String,
    },
    /// Route a proposal against the permanent corpus: ranked
    /// EXTEND/NEW candidates with evidence. Advisory only — the decision is
    /// recorded in the proposal's Capability Routing table, never derived
    /// from this output at review time.
    Route {
        /// Active change whose proposal.md to route.
        #[arg(long)]
        change: Option<String>,
        /// Proposal file to route (alternative to --change).
        #[arg(long)]
        file: Option<PathBuf>,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum SpecFormat {
    Text,
    Json,
}

fn main() {
    let matches = Cli::command().get_matches();
    let command_name = matches.subcommand_name().unwrap_or("givn").to_string();
    let cli = Cli::from_arg_matches(&matches).unwrap_or_else(|error| error.exit());

    // The recording wrapper: one `command` entry per invocation, measured
    // around dispatch and written after it, best-effort (src/history.rs). It
    // is not entered at all for the two reading commands.
    let recorder =
        records(&cli.command).then(|| history::install(&command_name, &invocation_label()));
    let result = dispatch(cli);
    let (outcome, exit_code) = if result.is_ok() {
        ("ok", 0)
    } else {
        ("failed", 1)
    };
    if let Some(recorder) = &recorder {
        recorder.finish(outcome, exit_code);
    }

    if let Err(e) = result {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}

/// Whether this invocation records. The record's two read views — `givn
/// history` without its intake subcommand, and `givn timeline` — are
/// deliberately not recorded: an inspection that appended an entry about itself
/// would put the newest entry into every report it produced (design-review Q8).
/// Every other command records normally, the agent intake included — a
/// reported payload is activity Givn recorded on the agent's behalf.
fn records(command: &Commands) -> bool {
    match command {
        Commands::Timeline { .. } => false,
        Commands::History { command, .. } => command.is_some(),
        _ => true,
    }
}

/// The invocation's own text (arguments after the program name), the command
/// entry's bounded subject label.
fn invocation_label() -> String {
    std::env::args().skip(1).collect::<Vec<String>>().join(" ")
}

fn dispatch(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Init {
            force,
            no_addons,
            agents,
        } => cli::init::run(force, no_addons, &agents),
        Commands::Commit { message, paths } => cli::commit::run(&message, &paths),
        Commands::New { id, skip } => cli::new::run(&id, &skip),
        Commands::Instructions {
            artifact,
            change,
            json,
        } => cli::instructions::run(artifact.as_deref(), change.as_deref(), json),
        Commands::Lint {
            change,
            paths,
            json,
        } => {
            let code = cli::lint::run(change.as_deref(), paths, json)?;
            if code != 0 {
                std::process::exit(code);
            }
            Ok(())
        }
        Commands::Check { artifact, change } => {
            cli::check::run(artifact.as_deref(), change.as_deref())
        }
        Commands::Status { change, json } => cli::status::run(change.as_deref(), json),
        Commands::History {
            command,
            change,
            topic,
            json,
        } => match command {
            Some(HistoryCommands::Ingest) => {
                // The intake's machine contract: only an unreadable payload is
                // a non-zero exit, and the process exits before the recording
                // wrapper runs, so a refused payload records nothing.
                let code = cli::history::ingest()?.exit_code();
                if code != 0 {
                    std::process::exit(code);
                }
                Ok(())
            }
            Some(HistoryCommands::HarnessHook { harness }) => cli::history::harness_hook(&harness),
            None => cli::history::run(change.as_deref(), topic.as_deref(), json),
        },
        Commands::Timeline { change, topic, out } => {
            cli::timeline::run(change.as_deref(), topic.as_deref(), out.as_deref()).map(|_| ())
        }
        Commands::Graph { change, json } => cli::graph::run(change.as_deref(), json),
        Commands::Archive { change, verbose } => cli::archive::run(&change, verbose),
        Commands::Eject { target, force } => cli::eject::run(&target, force),
        Commands::Addons { command } => match command {
            AddonsCommands::List => cli::addons::run_list(),
            AddonsCommands::Enable { name } => cli::addons::run_enable(&name),
            AddonsCommands::Disable { name } => cli::addons::run_disable(&name),
        },
        Commands::Readme {
            command: ReadmeCommands::Check,
        } => {
            let code = cli::readme::run_check()?;
            if code != 0 {
                std::process::exit(code);
            }
            Ok(())
        }
        Commands::Skills { command } => match command {
            SkillsCommands::Sync => cli::skills::run_sync(),
            SkillsCommands::Targets {
                command: TargetsCommands::Add { agents },
            } => cli::skills::run_targets_add(&agents),
        },
        Commands::Spec { command } => match command {
            SpecCommands::Coverage { baseline, format } => {
                cli::spec::run_coverage(baseline.as_deref(), format.as_deref() == Some("json"))
            }
            SpecCommands::Duplicates { format } => {
                cli::spec::run_duplicates(matches!(format, SpecFormat::Json))
            }
            SpecCommands::Tree { format } => {
                cli::spec::run_tree(matches!(format, SpecFormat::Json))
            }
            SpecCommands::Find { query } => cli::spec::run_find(&query),
            SpecCommands::Route { change, file } => {
                cli::spec::run_route(change.as_deref(), file.as_deref())
            }
        },
        Commands::Upgrade => cli::upgrade::run(),
        Commands::Run { port, print } => std::process::exit(cli::run::execute(port, print)),
        Commands::MissingTestrunner => cli::missing_testrunner::run(),
        Commands::MissingRunCommand => cli::missing_run_command::run(),
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "givn", &mut std::io::stdout());
            Ok(())
        }
        Commands::Attribution { command } => match command {
            AttributionCommands::Stamp { message_file } => {
                cli::attribution::stamp_message(&message_file)
            }
            AttributionCommands::ClaudePreToolUse => cli::attribution::claude_pre_tool_use(),
        },
    }
}
