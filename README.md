# pwm

A simple password manager for Linux.

Inspired conceptually (although the implementation is completely independent) by [pass](https://www.passwordstore.org/) and [gopass](https://www.justwatch.com/blog/post/announcing-gopass/).

## Installing

Installation requires a working Rust installation. Rust is most easily installed with [rustup](https://rustup.rs/):

```sh
curl https://sh.rustup.rs -sSf | sh
```

### From HEAD

```sh
git clone https://github.com/CmdrMoozy/pwm.git
cd pwm
cargo install
```

## Basic Usage

```sh
# Set a default repository path to avoid having to retype it, and initialize the repository:
pwm config -k default_repository -s $HOME/pwm_repository
pwm init

# Store a password:
pwm set personal/email

# List stored passwords:
pwm ls

# Retrieve a stored password:
pwm get personal/email
```
