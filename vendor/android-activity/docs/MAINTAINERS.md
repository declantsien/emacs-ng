# Release Process

This can be more automated later, but for now this is a reminder of the steps
for rolling a releases.

Note: we treat the Winit-based examples (that depend on a specific version of
android-activity) separately to avoid an awkward chicken and egg problem. These
examples are instead updated in sync with updating the Winit backend based on
android-activity.

Create a release branch like `v0.4`

Bump the version in `Cargo.toml`

Check if there are any deps that need a version bump in Cargo.toml

Update the CHANGELOG.md

Run `cargo update` for all examples:
`for i in $PWD/examples/*; do cd "$i"; cargo update; done`

Commit all changes with a message like `"Release 0.4"`

Push the release branch, create a PR and wait for a green light from CI

(while waiting for CI) Build and run `agdk-mainloop` on an Android device

Publish to crates.io once there is a CI green light
```
cd android-activity
cargo ndk -t arm64-v8a publish --features=game-activity
```

Merge release branch to `main`


# Updating Winit backend + examples

Update `android-activity` branch for winit to depend on new release of `android-activity` and push

Update examples that reference the latest released version, E.g.:
`sed -i 's|android-activity = { version = "0.3"|android-activity = { version = "0.4"|' examples/*/Cargo.toml`

Run `cargo update` for all examples:
`for i in $PWD/examples/*; do cd "$i"; cargo update; done`

Build and run `agdk-eframe` on an Android device

Commit all changes with a message like `"Update Winit-based examples for Release 0.4"`