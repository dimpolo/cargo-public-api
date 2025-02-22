use public_api::{Options, PublicApi};
use rustdoc_json::PackageTarget;

#[test]
fn public_api() -> Result<(), Box<dyn std::error::Error>> {
    let rustdoc_json = rustdoc_json::Builder::default()
        .toolchain("nightly".to_owned())
        .build()?;

    let public_api = PublicApi::from_rustdoc_json(rustdoc_json, Options::default())?;

    expect_test::expect_file!["../public-api.txt"].assert_eq(&public_api.to_string());

    Ok(())
}

#[test]
fn package_target_bin() {
    test_alternative_package_target(PackageTarget::Bin("test_crate".into()));
}

#[test]
fn package_target_bin_2() {
    test_alternative_package_target(PackageTarget::Bin("main2".into()));
}

#[test]
fn package_target_test() {
    test_alternative_package_target(PackageTarget::Test("test".into()));
}

#[test]
fn package_target_example() {
    test_alternative_package_target(PackageTarget::Example("example".into()));
}

#[test]
fn package_target_bench() {
    test_alternative_package_target(PackageTarget::Bench("bench".into()));
}

fn test_alternative_package_target(package_target: PackageTarget) {
    let target_dir = tempfile::tempdir().unwrap();

    let path = rustdoc_json::Builder::default()
        .toolchain("nightly".to_owned())
        .manifest_path("tests/test_crate/Cargo.toml")
        .quiet(true) // Make it less noisy to run tests
        .package_target(package_target)
        .target_dir(&target_dir)
        .build()
        .unwrap();

    assert!(path.exists());
}
