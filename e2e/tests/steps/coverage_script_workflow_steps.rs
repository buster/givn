use crate::steps::GivnWorld;
use cucumber::when;

#[when("the user archives the change")]
fn archive_change(world: &mut GivnWorld) {
    let id = world
        .last_change_id
        .clone()
        .expect("a completed change is required");
    let env = world.env.as_ref().expect("environment is set");
    crate::harness::commit_fixture(env);
    crate::steps::archive_proof_support::refresh_change_receipt(world, &id);
    world.last_output = Some(crate::harness::run_givn(env, &["archive", "--change", &id]));
}
