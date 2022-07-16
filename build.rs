use std::env;

fn main() {
    if String::from("release") == env::var("PROFILE").expect("Expected build profile to be provided") {
        let _proc = std::process::Command::new("yarn")
            .args(&["build:production"])
            .spawn()
            .expect("Failed to run 'yarn build' to package CSS & JS");
    }
}
