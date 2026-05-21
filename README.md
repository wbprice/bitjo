# Bullet Journal TUI

A Rust/ratatui terminal app for rapid bullet-journal logging.

## Run

```sh
cargo run
```

The app writes daily Markdown files to `journal/YYYY-MM-DD.md`.

## Commands

The journal pane is focused by default. From the journal pane:

- `:` opens fuzzy command search in the command pane.
- `Up` and `Down` move the highlighted journal entry.

In fuzzy command search:

- `Up` and `Down` move through command results.
- `Enter` selects the highlighted command.
- `Esc` closes command search and returns focus to the journal pane.
- Selecting `note`, `event`, `feeling`, or `task` opens a text entry state for
  that command.
- Selecting `quit` exits immediately.
- In a selected command entry, `Esc` cancels the entry and returns focus to the
  journal pane.

Exact command forms also remain available after opening search. For example,
press `:`, type `n <text>`, and press `Enter` to add a note.

Available exact commands:

- `:n <text>` adds a note.
- `:e <text>` adds an event.
- `:f <text>` adds a feeling.
- `:t <text>` adds an incomplete task.
- `:q` quits.
