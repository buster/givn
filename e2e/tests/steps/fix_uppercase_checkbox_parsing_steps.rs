use crate::harness as common;
use crate::steps::GivnWorld;
use cucumber::given;

#[given(
    regex = r#"^a givn project with a change "([^"]+)" containing tasks\.md with content "([^"]*)"$"#
)]
fn e2e_given_change_with_tasks(world: &mut GivnWorld, id: String, content: String) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    let change_dir = env.path(&format!("givn/changes/{}", id));
    let out = crate::harness::run_givn(env, &["new", &id]);
    assert!(
        out.status.success(),
        "givn new {} failed: {}",
        id,
        String::from_utf8_lossy(&out.stderr)
    );
    let content = content.replace("\\n", "\n");
    std::fs::write(change_dir.join("tasks.md"), content).expect("write tasks.md");
    world.last_change_id = Some(id);
}
