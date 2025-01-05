# rust-git-automate


## LOCAL COMMANDS:

### `cargo run`:
This command will run the project in the development mode, but will execute the command for the current directory and will be only able to do so if the `origin` of the repository is set.

### `cargo build --release`
This command will build the project in release mode, and deploy the binary in the `target/release` directory. The binary name will be `rust-git-automate`(which is the name of the project's root directory).

## GLOBAL COMMANDS:
### `cargo install --path <path_to_this_project>`:

This command will install the binary of this project in the system. The binary will be installed in the `~/.cargo/bin` directory. The binary name will be `rust-git-automate`. 