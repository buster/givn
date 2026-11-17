use crate::harness as common;
use crate::steps::GivnWorld;
use cucumber::given;

#[given("a givn project with a standard manifest")]
fn e2e_given_standard_manifest(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
}
