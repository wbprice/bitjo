# Fuzzy Command Pane Search

## Status

Implemented.

## Related Specification

This feature extends the implemented [Bullet Journal TUI](./BulletJournalTUI.md)
MVP. That spec defines the Rust/ratatui app structure, command pane, journal
pane, command parser, and Markdown persistence model.

## Purpose & User Problem

The command pane should become faster and more forgiving when the user wants to
start an application command. Instead of requiring the user to remember exact
command abbreviations such as `:n` or `:q`, typing `:` opens a fuzzy command
picker.

The fuzzy search is limited to available application commands. It does not
search journal entries or historical journal files.

## Success Criteria

- The journal pane is focused by default.
- Pressing `:` opens a fuzzy command search in the command pane and focuses the command pane.
- Fuzzy command results appear in the command pane.
- Results update live as the user types.
- Matching uses simple subsequence matching.
- UI updates should complete in under 300ms.
- The user can move through command results with the keyboard.
- Selecting a command hides fuzzy search and focuses the command text entry
  state for that command.
- Selecting `note` changes the command pane title to `Enter a note` and prepares
  the pane to accept note text.
- Existing command submission behavior continues to work.
- Exact command tokens can still be submitted from search by pressing `:` and
  typing the remainder of the command, such as `n text`.
- Empty or no-result states are clear and non-disruptive.
- Pressing the escape key focuses the journal pane and hides the fuzzy text
  window.
- Remove the "switch to journal" command.
- When a note has been added, refocus the journal.
- Don't display "No matching commands" if the user has proceeded to enter a note, event, etc using the shortcut 
  - .e.g "No matching commands" error should not be displayed if the user has entered ":n this is a note"
- Remove the app status line from the command pane

## Scope

- A fuzzy command search mode entered by pressing `:` from the journal pane or
  the focused command pane.
- A small registry of application commands that can be searched by friendly
  names such as `note`, `event`, `feeling`, `task`, and `quit`.
- Only commands available from the command pane are searchable.
- Live command filtering while the user types.
- Result rendering inside the command pane.
- Keyboard navigation for result selection: `Up` and `Down` move the highlighted
  result, `Enter` selects it, and `Esc` exits search and focuses the journal
  pane.
- A selected-command entry state where the command pane title describes the
  selected command, such as `Enter a note`.
- Submission behavior that maps selected commands back to the existing command
  execution model.
- Commands that do not need additional text, such as `quit`, execute
  immediately when selected.
- Tests for command matching, search-mode input handling, selected-command
  submission, and existing parser compatibility where practical.

## Constraints

- Keep the app keyboard-driven.
- Continue using Rust and ratatui.
- Preserve existing journal Markdown storage.
- Do not require network access or an external service.
- Avoid heavy dependencies unless the matching behavior justifies one.

## Technical Considerations

- The current command pane has two display lines: input and status. Showing
  results in the command pane may require increasing command-pane height while
  search is active.
- The current `handle_command_key` behavior appends text directly into
  `command_input`. It will need an explicit command-pane mode for normal entry,
  fuzzy search, and selected-command entry.
- The command parser currently treats the first whitespace-delimited token as
  the command. The implementation can preserve this parser while adding a
  command registry that maps friendly command names to existing command tokens.
- Entry commands selected through fuzzy search should collect text and submit
  through the same behavior as `:n <text>`, `:e <text>`, `:f <text>`, and
  `:t <text>`.
- Commands that do not require additional text, such as `quit`, should execute
  immediately when selected.
- Matching should be implemented as a small pure function first so it can be
  unit tested independently from TUI rendering.
- Because the command list is small, subsequence matching and rerendering should
  be comfortably under the 300ms target without indexing.

## Out Of Scope

- Searching journal entries.
- Searching across historical journal files.
- Full-text indexing.
- Regex search.
- Search result persistence.
- Mouse interaction.
- Remote or synced search.
- Dependency-backed fuzzy scoring unless simple matching proves insufficient.
- Journal-pane-only commands such as `complete` and `cancel`.

## Open Questions

None.
