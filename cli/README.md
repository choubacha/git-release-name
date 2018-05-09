# git-release-name

This is a CLI for the release name library. It has a few options and can be called via either STDIN
or with command args. 


### Installation

From this directory:
```
$ cargo install -f
```

From root:
```
$ cargo install -f --path rn-cli/
```

### Usage

Help instructions:
```
Takes a git sha and uses it's relatively unique combination of letters and number to generate a release name

USAGE:
    git-release-name [OPTIONS] [SHA]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --format <format>    Declares the return format of the phrase. [values: snake, kebab, camel, pascal, title,
                             sentence, upper, lower]

ARGS:
    <SHA>...    Each arg should be a sha. If they are less than 8 characters they will be padded
```

#### For HEAD
Many times you'll want to just send in the current head:
```
$ git-release-name $(git rev-parse HEAD)
obediently purer headspring
```

#### STDIN

If you want to see all the possible release names you can pipe to it:
```
$ git rev-list HEAD | git-release-name
intentionally mirky swineherds
subversively nestlike cablets
laggardly wifeless faker
issuably twinning verso
drawlingly about scapegoat
coordinately spaceless trigraphs
windily vorant bugler
hyperbatically nodose shutter
bumpily sketchy hoodies
saltishly effete prescript
voluntarily velate klaxons
girlishly sketchy gremlins
illusively tangy raggle
reproductively nettly hassock
caressingly blotto fris
forwardly globate benders
independently barbate linkboy
whimperingly tabu caring
unavailably lanose milepost
indigently ternate stitcher
unfairly tawie premiere
ascetically spaceless fantigue
prayerlessly brinded lodger
conjunctly revived navette
transactionally lovesick hoodies
```

#### Formatting

You can also change the format of the returned release name using the `--format` flag:

```
$ git-release-name --format snake $(git rev-parse HEAD)
bumpily_sketchy_hoodies
```
