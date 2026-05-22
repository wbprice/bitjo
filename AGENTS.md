# Agent Instructions

## Development Workflow: Spec → Code

THESE INSTRUCTIONS ARE CRITICAL!

They dramatically improve the quality of the work you create.

### Phase 1: Requirements First

When asked to implement a feature or a bug fix, or any other changes, ALWAYS start by asking:
"Should I create a Spec for this task first?"

IF user agrees:

If the user wants to design a new feature:
- Track the new file in Git
- Create a markdown file in `./features/FeatureName.md`

If the user wants to fix a bug:
- Create a markdown file in `/bugs/BugName.md`
- Track the new file in Git
- Bugs specs should be very tightly scoped and focus on what existing behavior is and what expected behavior should be.

In either case:

- Interview the user to clarify:
- Purpose & user problem
- Success criteria
- Scope & constraints
- Technical considerations
- Out of scope items

Do not restate information from existing specifications in detail. 
Instead, include a brief summary and include a link to the original specification. 

### Phase 2: Review & Refine

After drafting the Spec:

- Present it to the user
- Ask: "Does this capture your intent? Any changes needed?"
- Iterate until user approves
- End with: "Spec looks good? Type 'GO!' when ready to implement"

### Phase 3: Implementation

ONLY after user types "GO!" or explicitly approves:

- Begin coding based on the Spec
- Reference the Spec for decisions
- Update Spec if scope changes, but ask user first.

### File Organization

\`\`\`

/
├── features/
│ ├── FeatureName.md # Shared/committed Specs
│ └── .local/ # Git-ignored experimental Specs
│ └── Experiment.md
├── bugs/
│ ├── BugName.md # Shared/committed Specs
│ └── .local/ # Git-ignored experimental Specs
│ └── Experiment.md

\`\`\`

**Remember: Think first, ask clarifying questions, _then_ code. The Spec is your north star.**

(source: https://lukebechtel.com/blog/vibe-speccing)

### Code Quality

Code formatting should follow Rust conventions. Allow cargo fmt changes even if the change seems unrelated to the
feature being implemented.