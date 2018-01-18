# git-release-name

A simple tool that takes a sha and returns a random name for the release. The name will be
deterministic based on the version of the tool.

# Installation

```
$ cargo install -f
```

# Usage

```
$ git-release-name $(git rev-parse HEAD)
obediently purer headspring
```

You can also change the format of the returned release name using the `--format` flag:

```
$ git-release-name --format snake $(git rev-parse HEAD)
bumpily_sketchy_hoodies
```

# Contributing

If you are contributing words, please add them in alphabetical order to the appropriate part of speech list.
