//! Step definitions for the `spec-gap` capability.
//!
//! The scenarios request the served proposal or review guidance on an
//! `ise`-enabled project; none reads the asset file.

use crate::steps::GivnWorld;
use crate::steps::serve_extended_rules_steps as serving;
use cucumber::when;

fn request(world: &mut GivnWorld, id: &str) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    world.last_output = Some(serving::request_change_guidance(env, id));
}

#[when("the consumer requests the review guidance for an active change")]
fn consumer_requests_review_guidance_for_an_active_change(world: &mut GivnWorld) {
    request(world, "review");
}
