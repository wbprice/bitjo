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
    add         Adds a new entry of a given type to the journal
    complete    Toggles the completion of the nth entry in the list
    emph        Toggles the importance of the nth entry in the list
    help        Prints this message or the help of the given subcommand(s)
    remove      Removes the nth entry in the list
```

### Examples

#### List entries

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

#### Add an entry

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
```

#### Remove an entry

```
cargo run remove note 0
```

Removes the 0th note from the journal

```
Bit Journal v0.1.0
Today is Tue, Nov 19.
  • Figure out enums
  X Laugh uncontrollably
  - I'm just surprised this worked
  - Eating a bagel
```
