# git-release-name

A simple tool that takes a sha and returns a random name for the release. The name will be
deterministic based on the version of the tool. This project is broken into three crates.
Each crate provides a different interface to the dictionary. The main library with functionality
is the libray (found in `rn-dictionary`). The other two are `rn-cli` and `rn-web`.


Repo is licensed under MIT.
