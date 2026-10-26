# boat-offload-canary

Disposable public Rust canary for boat heavy-command offload smoke
tests. No dependencies, no product code, no secrets. It exists so an
offloaded `cargo test --locked` has something small and safe to build
on a pool VM.

## Run modes

All modes keep the committed `Cargo.lock` valid, so `--locked` always
passes without network access.

| Mode | Command | What it proves |
| --- | --- | --- |
| Normal | `cargo test --locked` | Fast exit-0 offload round trip. |
| Cancel / timeout | `cargo test --locked --features slow-tests -- sleeps_for_sixty_seconds --nocapture` | Waits for the `BOAT_CANARY_SLEEPING` marker in streamed output, then interrupt (SIGINT to the shim) or let a short command timeout fire. |
| Staged return (optional) | `cargo test --locked --features lockfile-write -- appends_to_cargo_lock` | Appends one byte to the guest checkout's `Cargo.lock`; the change must return as a staged artifact (shim exit 74) with live source unchanged. |

Notes:

- Do not pass `--target-dir`, `--manifest-path` or similar path
  overrides to an offloaded run; the offload transport refuses them.
- `rust-toolchain.toml` pins the `stable` channel so host and guest
  rustup resolve the same toolchain.
