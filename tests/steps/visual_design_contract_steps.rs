use cucumber::then;

use crate::steps::GivnWorld;

#[then(regex = r##"^the ejected design template contains "(.+)"$"##)]
fn ejected_design_template_contains(_world: &mut GivnWorld, _needle: String) {
    unimplemented!()
}
