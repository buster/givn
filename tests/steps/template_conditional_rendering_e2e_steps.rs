//! End-to-end step definitions for the template-conditional-rendering delta.
//! These scenarios drive the real `givn` CLI toggles (`addons enable/disable`)
//! as subprocesses and then read the regenerated guidance. The fixture and
//! reading steps live in `template_conditional_rendering_steps` / the shared
//! support module.

use cucumber::when;

use crate::common;
use crate::steps::GivnWorld;
use crate::steps::template_conditional_rendering_support as support;

#[when("a consumer enables the arc42 and domain-modeling addons through the Givn CLI")]
fn consumer_enable_domain_modeling_and_arc42(world: &mut GivnWorld) {
    for addon in ["arc42", "domain_modeling"] {
        let out = common::run_givn(support::env(world), &["addons", "enable", addon]);
        world.last_output = Some(out);
    }
}

#[when(regex = r"^a consumer disables the ([a-z0-9-]+) addon through the Givn CLI$")]
fn consumer_disable_addon(world: &mut GivnWorld, addon: String) {
    let cli_id = addon.replace('-', "_");
    let out = common::run_givn(support::env(world), &["addons", "disable", &cli_id]);
    world.last_output = Some(out);
}
