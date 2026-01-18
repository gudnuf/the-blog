---
name: brainstorming
description: Guide users from a vision of a finished product to clear requirements through structured dialogue. Use when users say "help me brainstorm...", "I have an idea for...", "let's think through...", or similar phrases indicating they have a goal but need help articulating the pieces required to achieve it. Ends with /brainstorming-complete to generate a planning prompt.
---

# Brainstorming

Help users go from a vision of the finished product to understanding the pieces required to build it.

## Dialogue Approach

During brainstorming:

1. **Present options and tradeoffs** - When there are multiple valid approaches, lay them out with pros/cons
2. **Summarize understanding** - Periodically reflect back what you've understood to confirm alignment
3. **Ask narrowing questions** - Target questions that reduce ambiguity and clarify requirements

Focus on extracting:
- What the end state looks like
- Key technical decisions and constraints
- Components/pieces needed
- Dependencies between pieces
- Open questions that need resolution

## Ending the Session

When the user runs `/brainstorming-complete`:

1. Generate a **structured planning prompt** optimized for Claude's planning mode
2. Include a **short human summary** of what was decided

### Planning Prompt Format

```
## Goal
[One sentence describing the end state]

## Context
[Relevant project context, existing code, constraints]

## Requirements
- [Concrete requirement 1]
- [Concrete requirement 2]
- ...

## Technical Decisions
- [Decision 1]: [Choice made] - [Brief rationale]
- [Decision 2]: [Choice made] - [Brief rationale]
- ...

## Components
1. [Component/piece 1] - [What it does]
2. [Component/piece 2] - [What it does]
- ...

## Open Questions
- [Any unresolved questions to address during planning]
```

### Human Summary Format

A 2-3 sentence plain-language summary of what we're building and the key decisions made.
