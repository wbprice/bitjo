# Entry Specific Actions

## Status

Implemented.

Correction: completed-task strikethrough behavior is superseded by
[Completed Tasks No Strikethrough](../bugs/CompletedTasksNoStrikethrough.md).

## Related Specifications

This feature extends the implemented [Bullet Journal TUI](./BulletJournalTUI.md)
MVP and the implemented [Fuzzy Command Pane Search](./FuzzyCommandPaneSearch.md)
feature. Those specs define the journal pane, command pane, command registry,
entry highlighting, keyboard-driven flow, and Markdown persistence model.

## Purpose & User Problem

When a journal entry is highlighted, available actions should be specific to
that entry's type and state. The user should be able to act on the highlighted
entry without needing to remember which commands are valid for every entry type,
and invalid actions should not mutate unrelated entries.

This feature treats entry-specific actions as context-specific actions for the
currently highlighted journal entry. Plain note entries do not introduce new
actions in this scope.

## Success Criteria

- When a task entry is highlighted, task actions are available.
- A highlighted task can be marked complete.
- A highlighted task can be cancelled when it is not complete.
- A completed task can not be marked cancelled.
- When an event entry is highlighted, event actions are available.
- A highlighted event can be cancelled.
- When a plain note entry is highlighted, invoking task/event-specific actions
  does nothing.
- When a feeling entry is highlighted, invoking task/event-specific actions does
  nothing.
- Action results are persisted to Markdown using the existing Bullet Journal TUI
  persistence rules.
- The UI does not mutate a non-highlighted entry when an action is invoked.

## Scope

- Context-specific actions are available only for the currently highlighted
  journal entry.
- Task entries support completion and cancellation actions.
- Event entries support cancellation actions.
- Plain note entries have no new actions in this feature.
- Feeling entries have no new actions in this feature.
- Actions are invoked only when a given entry is highlighted in the journal
  pane.

## Constraints

- Preserve the existing Rust and ratatui application stack.
- Preserve the existing Markdown storage model.
- Preserve the existing persisted representation described in the
  [Bullet Journal TUI](./BulletJournalTUI.md) spec: cancelled events/tasks use
  strikethrough, and completed tasks use the `X` task marker.
- Do not add a network service, database, authentication, or sync.
- Do not change existing journal entries unless the highlighted entry receives a
  valid action.

## Technical Considerations

- The action handler should inspect the highlighted entry before mutating state.
- Valid actions should be derived from the highlighted entry type and, for
  tasks, the current completion/cancellation state.
- Invalid actions on non-applicable entries should be no-ops.
- Markdown rendering and parsing should continue to rely on the behavior already
  defined by the linked Bullet Journal TUI spec.
- Tests should cover valid task actions, valid event actions, and invalid
  actions on note/feeling entries.

## Out Of Scope

- Adding new actions for plain note entries.
- Adding actions for feeling entries.
- Applying actions to entries that are not currently highlighted.
- Bulk actions across multiple entries.
- Searching journal entries.
- Changing the Markdown file organization.
- Changing the fuzzy command search behavior except where it already routes to
  highlighted-entry actions.

## Open Questions

None.
