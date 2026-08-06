use crate::common;
use crate::steps::GivnWorld;
use cucumber::given;

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
