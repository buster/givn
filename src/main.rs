mod cli;
mod config;
mod error;
mod gherkin;
mod hooks;
mod skills;
mod state;

use clap::{Parser, Subcommand};

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
    /// Scaffold givn/ and generate AGENTS.md and skills.
    Init {
        /// Overwrite existing files.
        #[arg(long)]
        force: bool,
    },
    /// Create a new change scaffold.
    New {
        /// Change identifier (kebab-case).
        id: String,
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
    /// Show lifecycle state derived from the filesystem.
    Status {
        /// Change identifier (default: all active changes).
        #[arg(long)]
        change: Option<String>,
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
    /// Regenerate AGENTS.md and skills.
    Skills {
        #[command(subcommand)]
        command: SkillsCommands,
    },
    /// Re-render managed config scaffold, preserve user overrides, bump version.
    Upgrade,
    /// Sentinel default verify.command: prints setup guidance and exits non-zero.
    #[command(name = "missing-testrunner", hide = true)]
    MissingTestrunner,
}

#[derive(Subcommand)]
enum SkillsCommands {
    /// Regenerate AGENTS.md and skills from current templates.
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
        Commands::Init { force } => cli::init::run(force),
        Commands::New { id } => cli::new::run(&id),
        Commands::Instructions { artifact, change, json } => {
            cli::instructions::run(&artifact, change.as_deref(), json)
        }
        Commands::Lint { paths } => cli::lint::run(paths),
        Commands::Check { artifact, change } => {
            cli::check::run(artifact.as_deref(), change.as_deref())
        }
        Commands::Status { change } => cli::status::run(change.as_deref()),
        Commands::Apply { change } => cli::apply::run(&change),
        Commands::Archive { change } => cli::archive::run(&change),
        Commands::Eject { target, force } => cli::eject::run(&target, force),
        Commands::Skills { command: SkillsCommands::Sync } => cli::skills::run_sync(),
        Commands::Upgrade => cli::upgrade::run(),
        Commands::MissingTestrunner => cli::missing_testrunner::run(),
    }
}
