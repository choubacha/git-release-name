# git-release-name

A simple tool that takes a sha and returns a random name for the release. The name will be deterministic based on the version of the tool.

# Installation

```
$ cargo install -f
```

# Usage

```
$ git-release-name $(git rev-parse HEAD)
obediently purer headspring
```
