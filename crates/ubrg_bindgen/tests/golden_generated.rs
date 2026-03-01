use std::fs;
use std::path::PathBuf;

use ubrg_bindgen::cli::GenerateArgs;

#[test]
fn golden_simple_fixture() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let fixture = repo.join("fixtures/simple/src/simple.udl");
    let expected = repo.join("fixtures/simple/expected/simple.rb");
    let out_dir = repo.join("target/test-generated-ruby");

    let _ = fs::remove_dir_all(&out_dir);

    ubrg_bindgen::ruby::generate_bindings(&GenerateArgs {
        source: fixture,
        out_dir: out_dir.clone(),
        config: None,
    })
    .expect("generation should succeed");

    let generated = fs::read_to_string(out_dir.join("simple.rb")).expect("generated file");
    let expected = fs::read_to_string(expected).expect("expected file");
    assert_eq!(generated, expected);
}
