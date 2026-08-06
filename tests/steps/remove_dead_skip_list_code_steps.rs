use crate::common;
use crate::steps::GivnWorld;
use cucumber::given;

#[given(
    regex = r#"^a givn project with a change "([^"]+)" that has proposal\.md and a \.givn-skip file listing "([^"]+)"$"#
)]
fn e2e_given_change_with_skip(world: &mut GivnWorld, id: String, skipped: String) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    let out = crate::common::run_givn(env, &["new", &id]);
    assert!(
        out.status.success(),
        "givn new {} failed: {}",
        id,
        String::from_utf8_lossy(&out.stderr)
    );
    let change_dir = env.path(&format!("givn/changes/{}", id));
    std::fs::write(change_dir.join(".givn-skip"), format!("{}\n", skipped))
        .expect("write .givn-skip");
    world.last_change_id = Some(id);
}
