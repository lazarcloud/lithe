# Lithe Syntax Highlighting

VS Code extension for Lithe (`.lzr`) files.

## Features

- Syntax highlighting for HTML, CSS, and JavaScript/TypeScript.
- Support for Lithe-specific syntax:
  - `{expression}` interpolation.
  - `{#if}`, `{#each}`, `{#await}` blocks.
  - `bind:`, `on:`, and other directives.

## Installation

To install this extension locally:

```bash
bun run install-extension
```

This script links the extension to both your local VS Code and VS Code Server (for remote development) extensions directories.

To uninstall:
```bash
bun run uninstall-extension
```

## Activating Changes

After installing or making changes to the grammar, reload VS Code:
1. Open the Command Palette (`Ctrl+Shift+P` or `Cmd+Shift+P`).
2. Run the command **Developer: Reload Window**.
