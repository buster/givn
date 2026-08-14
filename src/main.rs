mod cli;
mod cobertura;
mod config;
mod error;
mod gherkin;
mod hooks;
mod integrity_hook;
mod overlap;
mod skills;
mod spec;
mod state;

use clap::{CommandFactory, Parser, Subcommand};
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
    /// Archive a completed change (gated: runs hooks of apply.requires artifacts).
    Archive {
        /// Change identifier.
        #[arg(long)]
        change: String,
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
    /// Regenerate skills and agent commands.
    Skills {
        #[command(subcommand)]
        command: SkillsCommands,
    },
    /// Build and query the local permanent-scenario retrieval index.
    Spec {
        #[command(subcommand)]
        command: SpecCommands,
    },
    /// Re-render managed config scaffold, preserve user overrides, bump version.
    Upgrade,
    /// Sentinel default verify.command: prints setup guidance and exits non-zero.
    #[command(name = "missing-testrunner", hide = true)]
    MissingTestrunner,

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
}

#[derive(Subcommand)]
enum SpecCommands {
    /// Build or refresh the permanent-scenario index.
    Index,
    /// Search the local index for related scenarios.
    Search {
        /// Behavior description or completed scenario text.
        query: String,
    },
    /// Refresh the permanent index and report advisory candidates for a change.
    Review {
        /// Active change identifier.
        #[arg(long)]
        change: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = dispatch(cli);
    if let Err(e) = result {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}

fn dispatch(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Init { force, no_addons } => cli::init::run(force, no_addons),
        Commands::New { id, skip } => cli::new::run(&id, &skip),
        Commands::Instructions {
            artifact,
            change,
            json,
        } => cli::instructions::run(artifact.as_deref(), change.as_deref(), json),
        Commands::Lint { change, paths, json } => {
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
        Commands::Graph { change, json } => cli::graph::run(change.as_deref(), json),
        Commands::Archive { change } => cli::archive::run(&change),
        Commands::Eject { target, force } => cli::eject::run(&target, force),
        Commands::Addons { command } => match command {
            AddonsCommands::List => cli::addons::run_list(),
            AddonsCommands::Enable { name } => cli::addons::run_enable(&name),
            AddonsCommands::Disable { name } => cli::addons::run_disable(&name),
        },
        Commands::Skills {
            command: SkillsCommands::Sync,
        } => cli::skills::run_sync(),
        Commands::Spec { command } => match command {
            SpecCommands::Index => cli::spec::run_index(),
            SpecCommands::Search { query } => cli::spec::run_search(&query),
            SpecCommands::Review { change } => cli::spec::run_review(&change),
        },
        Commands::Upgrade => cli::upgrade::run(),
        Commands::MissingTestrunner => cli::missing_testrunner::run(),
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "givn", &mut std::io::stdout());
            Ok(())
        }
    }
}
