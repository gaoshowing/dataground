# db-tool

db-tool is an open source database client built with Tauri, Rust, Vue 3, and TypeScript.

The project is in early development. The current codebase focuses on the desktop shell, multi-window frontend structure, and the foundation for keeping shared application state in Rust.

## Features

- Tauri 2 desktop application
- Vue 3 + TypeScript frontend
- Vite multi-page entry setup for independent windows
- Rust command layer for opening application windows
- Separate main, query, and settings window entries

## Tech Stack

- Tauri 2
- Rust
- Vue 3
- TypeScript
- Vite

## Project Structure

```txt
.
├── index.html              # Main window HTML entry
├── query.html              # Query window HTML entry
├── settings.html           # Settings window HTML entry
├── src/
│   ├── entries/            # Vue entry files for each window
│   ├── windows/            # Window-level Vue components
│   └── styles.css          # Shared frontend styles
└── src-tauri/              # Tauri and Rust application code
```

## Getting Started

### Requirements

- Node.js
- Yarn
- Rust
- Tauri system dependencies for your platform

See the Tauri prerequisites guide for platform-specific setup:
https://tauri.app/start/prerequisites/

### Install Dependencies

```bash
yarn install
```

### Start the Desktop App

```bash
yarn tauri dev
```

Tauri starts the Vite dev server automatically through `beforeDevCommand`, so you usually should not run `yarn dev` separately while using `yarn tauri dev`.

### Frontend-Only Development

```bash
yarn dev
```

Then open:

- http://localhost:1420/
- http://localhost:1420/query.html
- http://localhost:1420/settings.html

### Build

```bash
yarn build
```

### Rust Check

```bash
cd src-tauri
cargo check
```

## Multi-Window Model

The frontend uses Vite multi-page entries:

- `src/entries/main.ts`
- `src/entries/query.ts`
- `src/entries/settings.ts`

Each entry mounts a dedicated window component from `src/windows`.

Tauri opens secondary windows through Rust commands:

- `open_query_window`
- `open_settings_window`

This keeps the desktop window lifecycle in Rust while allowing each Vue entry to stay focused on a single window UI.

## Roadmap

- Database connection management
- SQL editor integration
- Query execution and result grid
- Connection profiles and local settings
- Cross-window state managed from Rust
- Import/export workflows

## Contributing

Contributions are welcome once the project direction stabilizes. For now, please keep changes small and focused.

Before opening a pull request, make sure these checks pass:

```bash
yarn build
cd src-tauri
cargo check
```

## License

This project is licensed under the MIT License. See [LICENSE](./LICENSE) for details.
