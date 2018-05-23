# `git-release-name`

A simple tool that takes a sha and returns a random name for the release. The name will be
deterministic based on the version of the tool. This project is broken into three crates.
Each crate provides a different interface to the dictionary. The main library with functionality
is the libray (found in `rn-dictionary`). The other two are `rn-cli` and `rn-web`.

## Installation

Clone this repo and setup the rust compiler and cargo using rustup:
https://rustup.rs/

### CLI
Once that works (test: `cargo -v`) you can install the cli:

```bash
$ cargo install --force --path=cli
```

### Web
If you want to use the web app, there's a docker container for it:

```bash
docker run -it -p 6767:6767 kbacha/git-release-name
```

Then you can use curl:

```bash
$ curl "0.0.0.0:6767/api/release-name/$(git rev-parse HEAD)"
```

## License

Repo is licensed under MIT.
