use cucumber::when;

use super::GivnWorld;
use crate::common;

#[when("the consumer runs the migration coverage check")]
fn consumer_runs_migration_coverage_check(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("migration fixture project");
    world.last_output = Some(common::run_givn(
        env,
        &["check", "migration", "--change", "migration-test"],
    ));
}
