# Bullet Journal TUI

A Rust/ratatui terminal app for rapid bullet-journal logging.

<img width="864" height="468" alt="image" src="https://github.com/user-attachments/assets/62eff046-ddad-4f1e-b940-5b9dc0ae931a" />

## Run

```sh
cargo run
```

The app writes daily Markdown files to `journal/YYYY-MM-DD.md`.

## Commands

The journal pane is focused by default. From the journal pane:

- `:` opens fuzzy command search in the command pane.
- `Up` and `Down` move the highlighted journal entry.
- `Left` and `Right` move to the previous or next journal day.

In fuzzy command search:

- `Up` and `Down` move through command results.
- `Enter` selects the highlighted command.
- `Esc` closes command search and returns focus to the journal pane.
- Selecting `note`, `event`, `feeling`, or `task` opens a text entry state for
  that command.
- Selecting `quit` exits immediately.
- In a selected command entry, `Esc` cancels the entry and returns focus to the
  journal pane.
- When a task is highlighted, `complete` and `cancel` are available as
  entry-specific actions.
- When an event is highlighted, `cancel` is available as an entry-specific
  action.

Exact command forms also remain available after opening search. For example,
press `:`, type `n <text>`, and press `Enter` to add a note.

Available exact commands:

- `:n <text>` adds a note.
- `:e <text>` adds an event.
- `:f <text>` adds a feeling.
- `:t <text>` adds an incomplete task.
- `:q` quits.
- `:x` marks a highlighted task complete or reopens it.
- `:c` cancels or reopens a highlighted task/event. Completed tasks must be
  reopened before they can be cancelled.
