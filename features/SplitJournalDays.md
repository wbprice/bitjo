# Split Journal Days

## Status

Implemented.

## Related Specs

This feature builds on the daily Markdown journal model and keyboard-driven TUI
flow defined in [BulletJournalTUI.md](./BulletJournalTUI.md), plus the existing
relative day navigation from [SwitchJournalDay.md](./SwitchJournalDay.md).

## Summary

Add a toggleable split journal view that displays an adjacent two-day journal
window, starting with yesterday and today. The layout should adapt to terminal
size, using side-by-side panes when there is enough horizontal space and stacked
panes on smaller screens.

## Purpose & User Problem

Users want to compare adjacent journal days without repeatedly switching the
single displayed day. Initially, this supports review and comparison between
yesterday and today. Later, the same split display can support a task migration
workflow, but migration itself is not part of this feature.

## Success Criteria

- A user can toggle split view on and off with `:split`.
- When split view first turns on, the UI displays yesterday's journal and
  today's journal at the same time.
- On larger terminal sizes, the two journals are displayed side by side.
- On smaller terminal sizes, the two journals are stacked vertically.
- Exactly one journal pane is active at a time.
- Pressing `Left` while split view is active focuses the previous-day pane when
  the current-day pane is active.
- Pressing `Left` while the previous-day pane is already active shifts the
  visible two-day window back one day, such as from yesterday/today to two days
  ago/yesterday.
- Pressing `Right` while split view is active focuses the current-day pane when
  the previous-day pane is active.
- Pressing `Right` while the current-day pane is already active shifts the
  visible two-day window forward one day, such as from yesterday/today to
  today/tomorrow.
- Entry creation, completion, cancellation, and selection movement apply only to
  the active journal pane.
- When split view is off, the current single-journal view and existing day
  navigation behavior remain available.
- Toggling split view off returns to a single focused journal view without
  losing unsaved journal changes.

## Scope

Add a split display mode to the journal pane:

- `:split` toggles between single-journal view and split-journal view.
- Split view loads and displays exactly two adjacent days, initially yesterday
  and today.
- Split view has an active/focused journal pane, visibly distinguished from the
  inactive pane.
- `Left` and `Right` select the active split pane when moving within the visible
  two-day window.
- Pressing `Left` from the left/older pane replaces the two panes with the
  previous two-day window and keeps the older pane active.
- Pressing `Right` from the right/newer pane replaces the two panes with the
  next two-day window and keeps the newer pane active.
- `Up` and `Down` move the highlighted entry within the active pane only.
- Commands that create or mutate entries operate on the active pane only.
- The command pane remains shared across the whole app.
- Status messages should make it clear when split view is toggled and which pane
  is active when useful.

## Constraints

- Preserve the existing Rust and ratatui application stack.
- Continue using one Markdown file per day under `journal/YYYY-MM-DD.md`.
- Do not create an empty Markdown file just because a day is displayed in split
  view.
- Preserve the existing fast keyboard workflow.
- Preserve current note, event, feeling, task, completion, and cancellation
  behavior.
- Keep the initial split view fixed to yesterday and today.
- Split view may move only by adjacent two-day windows in this feature.

## Technical Considerations

- The app currently owns a single loaded `Journal`; split view will require
  state for two loaded adjacent journals plus an active-pane indicator.
- Split view should track the visible date window so it can shift backward or
  forward by one day when the user navigates past either edge.
- Selection state should be tracked independently for each visible journal day.
- Entry creation should use the active pane's journal date so persisted entries
  go to the correct daily Markdown file.
- Entry actions should inspect and mutate only the active pane's highlighted
  entry.
- Saving should continue to write only journals that receive changes.
- The UI layout should choose horizontal or vertical split based on the available
  terminal area.
- Existing `Left`/`Right` day switching behavior should remain unchanged when
  split view is off.
- Tests should cover toggling split view, loading yesterday/today, independent
  selection, active-pane switching, shifting the visible date window backward
  and forward, active-pane entry creation, active-pane entry actions, responsive
  layout behavior where practical, and preservation of single-view day
  navigation.

## Out of Scope

- Task migration.
- Copying entries between days.
- Date picker.
- Direct date entry.
- Cross-day search.
- More than two visible journal days.
- Configuring which two days appear in split view.

## Open Questions

None.
