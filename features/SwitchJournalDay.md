# Switch Journal Day

## Status

Implemented.

## Related Specs

This feature builds on the daily Markdown journal model and keyboard-driven TUI
flow defined in [BulletJournalTUI.md](./BulletJournalTUI.md). It also preserves
the command search and entry-specific action behavior from
[FuzzyCommandPaneSearch.md](./FuzzyCommandPaneSearch.md) and
[EntrySpecificActions.md](./EntrySpecificActions.md).

## Purpose & User Problem

Users need to move between nearby journal days without quitting the app or
manually opening Markdown files. This supports reviewing yesterday, preparing
tomorrow, and adding entries to a non-today daily log while keeping the app's
fast keyboard workflow.

## Success Criteria

- A user can switch from the current journal day to the previous day.
- A user can switch from the current journal day to the next day.
- Switching days loads the corresponding `journal/YYYY-MM-DD.md` file when it
  exists.
- Switching to a day without an existing file shows an empty journal pane.
- The journal pane title updates to the newly displayed day.
- After switching days, the highlighted entry is the last entry for that day.
- If the target day has no entries, no entry is highlighted.
- New entries are written to the currently displayed day, not necessarily today.

## Scope

Add relative day navigation for the journal pane:

- `Left` switches to yesterday relative to the currently displayed journal day
  when the journal pane is focused.
- `Right` switches to tomorrow relative to the currently displayed journal day
  when the journal pane is focused.
- Day switching should keep focus on the journal pane after the target day is
  loaded.
- The status message should confirm the loaded day or report any load error.

Day switching is not exposed as an exact command or fuzzy command in this
feature.

## Constraints

- Keep storage local and Markdown-based.
- Continue using one file per day under `journal/YYYY-MM-DD.md`.
- Do not create an empty Markdown file just because the user views an empty day.
- Preserve current task, event, note, feeling, completion, and cancellation
  behavior.
- Preserve the existing fast keyboard workflow.

## Technical Considerations

- The app currently owns a single loaded `Journal`. Switching days should load a
  new `Journal` for the target date and replace the app's current journal state.
- The app needs to retain or derive the journal root path so it can load another
  date after startup.
- The selected entry should be recalculated with the same "last entry" behavior
  used on startup.
- Entry creation should use the loaded journal's date, so persisted entries go
  to the displayed day's file.
- Day arithmetic should use `chrono` date operations and handle month/year
  boundaries.
- Tests should cover command parsing, switching to existing and empty days,
  selection after switching, and writing new entries to the displayed day.

## Out Of Scope

- Direct date entry.
- Calendar picker.
- Date search.
- Week or month views.
- Task migration between days.
- Cross-day indexing or search.

## Open Questions

None.
