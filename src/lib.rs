//! Disposable canary crate for boat heavy-command offload smoke tests.
//!
//! This crate has no dependencies and no product code. It exists so an
//! offloaded `cargo test --locked` has something small, public and
//! secret-free to build. Run modes are documented in `README.md`:
//!
//! - plain `cargo test --locked` runs the normal test set and finishes
//!   in seconds;
//! - `--features slow-tests` adds a 60-second sleeping test for the
//!   cancel and timeout steps;
//! - `--features lockfile-write` adds a test that appends a byte to the
//!   guest copy of `Cargo.lock`, exercising staged-return behaviour.
//!
//! The features are declared in `Cargo.toml` so the generated
//! `Cargo.lock` stays valid for every mode under `--locked`.

/// Adds two numbers. Kept trivial on purpose.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn it_adds_two_numbers() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(0, 0), 0);
    }

    /// Sleeping test for the cancel/timeout smoke steps. Enabled only
    /// with `--features slow-tests` so the default run stays fast.
    /// Prints a marker first so the smoke can confirm streaming output
    /// before it interrupts the run.
    #[cfg(feature = "slow-tests")]
    #[test]
    fn sleeps_for_sixty_seconds() {
        println!("BOAT_CANARY_SLEEPING");
        std::thread::sleep(std::time::Duration::from_secs(60));
    }

    /// Optional staged-return exercise: appends one byte to the
    /// checkout's `Cargo.lock`. Offloaded, the change must come back
    /// as a staged artifact and must never touch the live source.
    /// Enabled only with `--features lockfile-write`.
    #[cfg(feature = "lockfile-write")]
    #[test]
    fn appends_to_cargo_lock() {
        let lock = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock");
        let mut bytes = std::fs::read(&lock).expect("read Cargo.lock");
        bytes.push(b'\n');
        std::fs::write(&lock, bytes).expect("write Cargo.lock");
    }
}
