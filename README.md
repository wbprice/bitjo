# Bullet Journal TUI

A Rust/ratatui terminal app for rapid bullet-journal logging.

## Run

```sh
cargo run
```

The app writes daily Markdown files to `journal/YYYY-MM-DD.md`.

## Commands

In the command pane:

- `:n <text>` adds a note.
- `:e <text>` adds an event.
- `:f <text>` adds a feeling.
- `:t <text>` adds an incomplete task.
- `:cw` focuses the journal pane.
- `:q` quits.

In the journal pane:

- `Up` and `Down` move the highlighted entry.
- `Esc` returns to the command pane.
- `:` opens the mini command prompt.
- `:x` toggles completion for a highlighted task.
- `:c` toggles cancellation for a highlighted task or event.
