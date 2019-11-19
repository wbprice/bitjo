# Bit Journal CLI
CLI implementation of a Bullet Journal, written in Rust.

## Usage

```
bit-journal-cli 0.1.0

USAGE:
    bit-journal-cli [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add
    cancel
    complete
    help        Prints this message or the help of the given subcommand(s)
    remove
```

### Examples

```
cargo run
```

Lists current journal items

```
Bit Journal v0.1.0
Today is Wed, May  1.
  ⚬ Internal Standup at 4pm
  • Figure out enums
  X Laugh uncontrollably
  - I'm just surprised this worked
```

```
cargo run add note "Eating a bagel"
```

Adds a new note to the journal, then prints the journal

```
Bit Journal v0.1.0
Today is Tue, Nov 19.
  ⚬ Internal Standup at 4pm
  • Figure out enums
  X Laugh uncontrollably
  - I'm just surprised this worked
  - Eating a bagel
The current mode is Normal
```