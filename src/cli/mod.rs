pub mod absolute_mode;
pub mod addons;
pub mod agents_block;
pub mod attribution;

pub mod archive;
pub mod check;
pub mod commit;
pub mod coverage_gate;
pub mod eject;
pub mod git;
pub mod graph;
// Skeletons (tasks.md S4): declared here so the crate compiles; wired into
// dispatch by the scenarios that need each command.
#[allow(dead_code)]
pub mod history;
pub mod init;
pub mod instructions;
pub mod lint;
pub mod migration;
pub mod missing_run_command;
pub mod missing_testrunner;
pub mod new;
pub mod project_hint;
pub mod readme;
pub mod run;
pub mod run_block;
pub mod skills;
pub mod spec;
pub mod status;
// Skeleton (tasks.md S4): wired into dispatch by the report scenarios.
#[allow(dead_code)]
pub mod timeline;
pub mod upgrade;
