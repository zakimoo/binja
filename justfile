default_release_level:="minor"

# Default target
# This will build all the crates in the workspace
default: build


# This will install the necessary tools and set up the git hooks
# See the `install-clippy`, `install-cargo-release`, `install-git-cliff` and `git-hooks` commands
# Setup the project
setup: init-hooks install-clippy install-cargo-release install-git-cliff
  echo "Setup complete"


clippy:
  cargo clippy


build:
    cargo build

build-release:
    cargo build --release


install-clippy:
  rustup update
  rustup component add clippy


install-cargo-release:
  cargo install cargo-release


install-git-cliff:
  cargo install git-cliff


init-hooks:
  pre-commit install

run-hooks:
  pre-commit run


releaseit LEVEL=default_release_level:
  cargo release {{LEVEL}} --execute

# See the `first_release` command
release LEVEL=default_release_level:
  cargo release {{LEVEL}}
