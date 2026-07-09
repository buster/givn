mod cli;
mod config;
mod error;
mod gherkin;
mod hooks;
mod skills;
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
        /// Do not install extras skills and commands.
        #[arg(long)]
        no_extras: bool,
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
    /// Show instructions for an artifact.
    Instructions {
        /// Artifact id (e.g. proposal, specs, design, tasks, review).
        artifact: String,
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
    /// Track task progress for a change.
    Apply {
        /// Change identifier.
        #[arg(long)]
        change: String,
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
    /// Manage optional project-level extras (skills + commands).
    Extras {
        #[command(subcommand)]
        command: ExtrasCommands,
    },
    /// Regenerate skills and agent commands.
    Skills {
        #[command(subcommand)]
        command: SkillsCommands,
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
enum ExtrasCommands {
    /// Enable extras in givn/config.yaml and sync skills + commands.
    Install,
    /// Disable extras in givn/config.yaml and remove generated extras files.
    Uninstall,
}
#[derive(Subcommand)]
enum SkillsCommands {
    /// Regenerate skills and agent commands from current templates.
    Sync,
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
        Commands::Init { force, no_extras } => cli::init::run(force, no_extras),
        Commands::New { id, skip } => cli::new::run(&id, &skip),
        Commands::Instructions {
            artifact,
            change,
            json,
        } => cli::instructions::run(&artifact, change.as_deref(), json),
        Commands::Lint { change, paths } => {
            let code = cli::lint::run(change.as_deref(), paths)?;
            if code != 0 {
                std::process::exit(code);
            }
            Ok(())
        }
        Commands::Check { artifact, change } => {
            cli::check::run(artifact.as_deref(), change.as_deref())
        }
        Commands::Status { change, json } => cli::status::run(change.as_deref(), json),
        Commands::Apply { change } => cli::apply::run(&change),
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
        Commands::Upgrade => cli::upgrade::run(),
        Commands::Extras { command } => match command {
            ExtrasCommands::Install => cli::extras::run_install(),
            ExtrasCommands::Uninstall => cli::extras::run_uninstall(),
        },
        Commands::MissingTestrunner => cli::missing_testrunner::run(),
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "givn", &mut std::io::stdout());
            Ok(())
        }
    }
}
