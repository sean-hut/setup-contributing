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

[Git][git] is a runtime dependencies.  Please make sure that you have Git installed.

You will also need to make sure that you have configured Git's `user.name` and `user.email` variables.  These Git configuration variables are used when setting up the contributing directory structure and in the contributing agreement if you choose to include the optional contributing agreement rule.

You can check the value of the configuration variables with these commands.

`git config user.name`

`git config user.email`

[git]: <https://git-scm.com/>

## Use

1. Create a readme for your project.

    1. Navigate to the root directory of your project.
	1. Read the help message by running `setup-contributing --help`
	1. Run `setup-contributing` with your desired combination of flags and options.

## Examples

To see examples of contributing directories created by
`setup-contributing` refer to `example-use.md`.
