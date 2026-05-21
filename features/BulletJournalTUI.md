# Bullet Journal TUI

## Status

Implemented MVP.

## Purpose & User Problem

Build a terminal user interface for people who are already comfortable in TUIs
and like Bullet Journaling. The app should make rapid logging faster and more
structured than keeping a plain Markdown file open in an editor.

The first version focuses on capturing classic bullet-journal style entries
with minimal keystrokes:

- Notes
- Events
- Tasks
- Feelings

## Success Criteria

- A user can launch the app and immediately start typing in the command pane.
- A user can add a note with `:n ...`.
- A user can add an event with `:e ...`.
- A user can add a feeling with `:f ...`.
- A user can add an incomplete task with `:t ...`.
- New entries are rendered in the main journal pane after submission.
- The main journal pane title displays the current date in `MM.DD.Day` format,
  such as `5.21.T`.
- Journal contents are persisted as Markdown.
- The user can switch focus between the command pane and the journal pane.
- In the journal pane, the user can move a highlight across individual entries.
- Completed tasks are represented with `X`.
- A user can quit the app with `:q`.
- Adding a new entry writes the current journal file to disk immediately.
- When a task is highlighted in the journal pane, the user can mark it complete
  with `:x` or cancel it with `:c`.
- When an event is highlighted in the journal pane, the user can cancel it with
  `:c`.
- Completion and cancellation actions are reversible.
- Persisted Markdown should represent strikethrough by wrapping entry text with
  `~~`, such as `X ~~Completed task~~`.
- A completed task cannot be cancelled. The user must toggle completion off
  before cancelling it.

## Scope

Version 1 is a rapid logging MVP.

The screen is divided into two panes:

1. Main journal pane
   - Takes up most of the terminal.
   - Displays the current day's entries.
   - Has a title containing the current date in `MM.DD.Day` format.
   - Supports highlighting individual entries when focused.
2. Command pane
   - Sits at the bottom of the terminal.
   - Is focused by default.
   - Accepts quick commands for creating entries.

Supported entry commands:

- `:n <text>` creates a note.
- `:e <text>` creates an event.
- `:f <text>` creates a feeling.
- `:t <text>` creates an incomplete task.
- `:q` quits the app.

Supported journal-pane actions:

- Highlighted task + `:x` marks the task complete.
- Highlighted task + `:c` marks the task cancelled.
- Highlighted event + `:c` marks the event cancelled.
- Repeating a state action toggles that state off where applicable.
- If a highlighted task is complete, `:c` should be rejected with a clear status
  message.

Initial entry rendering should use recognizable Markdown-compatible bullet
journal symbols:

- Notes: `- <text>`
- Events: `◦ <text>`
- Feelings: `= <text>`
- Incomplete tasks: `· <text>`
- Completed tasks: `X <text>` with the text visually styled with
  strikethrough in the TUI.
- Cancelled tasks/events: original symbol with the text visually styled with
  strikethrough in the TUI.

## Constraints

- Implementation language and TUI stack: Rust with ratatui.
- Storage format: Markdown.
- Optimize for fast keyboard-driven capture.
- Keep the first version local-only.
- Do not add authentication or login.
- Do not add non-Markdown storage backends.

## Technical Considerations

- The app should have an event loop that handles keyboard input, pane focus,
  command editing, entry creation, scrolling, and highlighted-entry movement.
- The app needs a small domain model for journal entries with at least:
  - entry type
  - text
  - completion state for tasks
  - creation date
- Markdown files should be organized predictably by date. A likely default is
  one file per day, such as `journal/YYYY-MM-DD.md`.
- On launch, the app should load today's Markdown file if it exists.
- On entry submission, the app should append to today's in-memory entry list
  and persist the Markdown file.
- After any action that changes an entry, such as completing or cancelling an
  item, the app should persist the Markdown file.
- Date formatting for the pane title should map weekdays to short labels that
  prefer one letter except for Tuesday and Thursday:
  - Monday: `M`
  - Tuesday: `Tu`
  - Wednesday: `W`
  - Thursday: `Th`
  - Friday: `F`
  - Saturday: `Sa`
  - Sunday: `Su`
- Pane focus switching from the command pane to the journal pane should be
  handled by the command `:cw`.
- When the journal pane is focused, `Esc` should return focus to the command
  pane.
- When the journal pane is focused, pressing `:` should open a mini command
  prompt for context-specific commands such as `:c` and `:x`.
- The command parser should reject unknown commands and empty entries with a
  clear inline status message in the command pane.
- Tests should cover command parsing, Markdown rendering, date title formatting,
  and persistence behavior where practical.

## Out Of Scope

- Storage backends other than Markdown.
- Login, authentication, sync, or multi-user features.
- Monthly logs, future logs, task migration, collections, and indexing.
- Rich text editing.
- Mobile, web, or graphical desktop interfaces.
- Importing from existing journal systems.

## Open Questions

None.
