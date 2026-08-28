//! End-to-end step definitions for the template-conditional-rendering delta.
//! These scenarios drive the real `givn` CLI toggles (`addons enable/disable`)
//! as subprocesses and then read the regenerated guidance. The fixture and
//! reading steps live in `template_conditional_rendering_steps` / the shared
//! support module.

use cucumber::when;

use crate::common;
use crate::steps::GivnWorld;
use crate::steps::template_conditional_rendering_support as support;

#[when("a consumer enables the arc42 addon through the Givn CLI")]
fn consumer_enable_arc42(world: &mut GivnWorld) {
    let out = common::run_givn(support::env(world), &["addons", "enable", "arc42"]);
    world.last_output = Some(out);
}

#[when("a consumer disables the arc42 addon through the Givn CLI")]
fn consumer_disable_arc42(world: &mut GivnWorld) {
    let out = common::run_givn(support::env(world), &["addons", "disable", "arc42"]);
    world.last_output = Some(out);
}

#[when("a consumer disables the domain-modeling addon through the Givn CLI")]
fn consumer_disable_domain_modeling(world: &mut GivnWorld) {
    let out = common::run_givn(
        support::env(world),
        &["addons", "disable", "domain_modeling"],
    );
    world.last_output = Some(out);
}
