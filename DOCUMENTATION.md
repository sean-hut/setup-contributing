# Documentation

## Install

1. Install `rustup` with one of these options:

    - with your operating system's package manager
    - [instructions to install rust and cargo][rustup-install]

[rustup-install]: <https://doc.rust-lang.org/cargo/getting-started/installation.html>

2. Run `rustup update`.

3. Add `~/.cargo/bin` to your `PATH` environment variable.

4. Clone this git repository with:
   `git clone https://github.com/sean-hut/setup-contributing`

5. In the cloned git repository run:

    `cargo build --release`.

6. Copy the binary into cargo's bin directory.

    Copy
    `setup-contributing/target/release/setup-contributing`
    into the `~/.cargo/bin/`.

7. Make `~/.cargo/bin/setup-contributing` executable with this command:

    `chmod u=rwx ~/.cargo/bin/setup-contributing`

## Runtime Dependencies

This project has no runtime dependencies.

## Use

1. Create a readme for your project.

    1. Navigate to the root directory of your project.
	1. Read the help message by running `setup-contributing --help`
	1. Run `setup-contributing` with your desired combination of flags and options.

## Examples

To see examples of contributing directories created by
`setup-contributing` refer to `example-use.md`.
