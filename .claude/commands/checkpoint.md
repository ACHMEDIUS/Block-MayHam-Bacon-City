---
description: "Automatically commit all modified files with proper grouping and commit message style"
allowed-tools: ["Bash", "TodoWrite"]
---

# Checkpoint Command

This command will automatically:

1. Check git status for modified/untracked files
2. Group files logically by feature/context
3. Create commits following the established message style: `type: brief description`

The commit types used are:

- `feat:` - New features or major additions
- `enhance:` - Improvements to existing features
- `fix:` - Bug fixes
- `style:` - Styling/visual changes
- `refactor:` - Code restructuring without changing behavior
- `test:` - Adding or updating tests
- `chore:` - Routine tasks and maintenance

Files are grouped logically:

- **Audio/Recording**: Components and functions related to audio processing
- **UI Components**: Frontend components and dialogs
- **Sharing System**: Share functionality files
- **Templates System**: Template-related files
- **App Pages**: Next.js page components and middleware
- **Backend Functions**: Firebase functions and APIs
- **Core Architecture**: Core entities, validation, and types
- **Dependencies**: Package.json updates

## Usage

```
/checkpoint
```

This will automatically analyze your current changes and create appropriate commits. In addition to this, you can also specify custom commit messages by including them in the command, and specify files to ignore.

## Important Rules

- **NEVER add co-authored attribution or Claude Code references** to commit messages
- Keep commit messages clean and simple: `type: brief description`
- Follow the project's established commit message format exactly
