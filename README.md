# pwm

[![Build Status](https://travis-ci.org/CmdrMoozy/pwm.svg?branch=master)](https://travis-ci.org/CmdrMoozy/pwm) [![Coverage Status](https://coveralls.io/repos/github/CmdrMoozy/pwm/badge.svg?branch=master)](https://coveralls.io/github/CmdrMoozy/pwm?branch=master)

A simple password manager for Linux.

Inspired conceptually (although the implementation is completely independent) by [pass](https://www.passwordstore.org/) and [gopass](https://www.justwatch.com/blog/post/announcing-gopass/).

## Installing

Installation requires a working Rust installation. pwm is developed and tested against nightly Rust, but other versions may as well (Travis build status may be a useful indicator here). Rust is most easily installed with [rustup](https://rustup.rs/):

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

## Notes on Security

### Zeroing Sensitive Memory

pwm makes some attempt to zero memory used to store sensitive data like encryption keys or decrypted data, however this cannot be guaranteed in all cases. If we are to assume that an attack has access to read all memory, swap, registers, and etc. on your system, [zeroing buffers is insufficient](http://www.daemonology.net/blog/2014-09-06-zeroing-buffers-is-insufficient.html). Because ensuring that sensitive information is not leaked whatsoever is virtually impossible (see the linked article for details), pwm makes some attempt at zeroing memory purely as a defense-in-depth, but at the same time it accepts the potential risk involved in storing sensitive information in memory.
