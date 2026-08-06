use crate::common;
use crate::steps::{GivnWorld, stderr};
use cucumber::{given, then};

#[given(regex = r#"^a givn project with a change "([^"]+)" and proposal\.md$"#)]
fn e2e_given_givn_project_with_change_and_proposal(world: &mut GivnWorld, id: String) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    let out = crate::common::run_givn(env, &["new", &id]);
    assert!(
        out.status.success(),
        "givn new {} failed: {}",
        id,
        String::from_utf8_lossy(&out.stderr)
    );
    world.last_change_id = Some(id);
}

#[then("stderr says the change id must be kebab-case")]
fn stderr_says_kebab(world: &mut GivnWorld) {
    let s = stderr(world);
    assert!(
        s.contains("kebab-case"),
        "stderr missing kebab-case hint: {}",
        s,
    );
}
