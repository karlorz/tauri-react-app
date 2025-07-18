# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Architecture Overview

This is a Tauri-React desktop application that combines a Rust backend with a React frontend. The app features:

- **Rust Backend** (src-tauri/): Handles native desktop functionality, system tray, file operations, and OS integration
- **React Frontend** (src/): Modern UI built with Mantine components, i18n support, and routing
- **Tauri Bridge**: Secure communication between frontend and backend using Tauri's IPC system

## Key Technologies

- **Tauri v2**: Desktop app framework with Rust backend
- **React 18**: Frontend framework with JSX
- **Mantine v7**: UI component library with theming support
- **Vite**: Build tool and dev server
- **pnpm**: Package manager
- **i18next**: Internationalization (currently supports English and French)

## Development Commands

```bash
# Development (starts both frontend and backend)
pnpm dev

# Production build
pnpm build

# Start frontend only (for web development)
pnpm start

# Run tests
pnpm test

# Update dependencies
pnpm update

# Build for release
pnpm rls
```

## Project Structure

### Frontend Architecture (src/)
- `App.jsx`: Main application shell with routing, navigation, and Tauri event handling
- `Providers.jsx`: Root provider wrapper for all context providers
- `main.jsx`: React entry point
- `tauri/`: Tauri-specific React components and providers
  - `TauriProvider.jsx`: Context provider for Tauri state and APIs
  - `TitleBar.jsx`: Custom title bar component for Windows
- `components/`: Reusable UI components
- `views/`: Route components/pages
- `translations/`: i18n JSON files

### Backend Architecture (src-tauri/)
- `src/main.rs`: Application entry point
- `src/lib.rs`: Core application logic, plugins, and event handling
- `src/sys_tray.rs`: System tray functionality and menu handling
- `src/utils.rs`: Utility functions and helper commands
- `tauri.conf.json`: Tauri configuration and permissions

## Key Features

### System Tray Integration
- System tray icon with context menu
- Tray state management (playing/paused states)
- Window show/hide functionality
- Custom tray icons for different states

### Window Management
- Custom title bar on Windows
- Window state persistence
- Single instance enforcement
- Fullscreen detection

### Frontend Features
- Dark/light theme switching (Ctrl+J)
- Responsive navigation with collapsible sidebar
- Internationalization support
- Auto-updater integration
- Notification system

## Development Notes

### Tauri Commands
All Rust functions exposed to frontend must be marked with `#[tauri::command]` and added to `invoke_handler` in `lib.rs:72`.

### Frontend-Backend Communication
- Use `invoke()` from `@tauri-apps/api` to call Rust commands
- Use `listen()` from `@tauri-apps/api/event` for Rust-to-frontend events
- Events are emitted from Rust using `app.emit()`

### State Management
- Tauri context provides global state (OS info, directories, etc.)
- Mantine provides theme state
- Local storage handled via `localforage` and custom hooks

### Custom Hooks
- `useTauriContext()`: Access Tauri-specific state
- `useCookie()`: Persistent cookie-like storage
- `useLocalForage()`: Async local storage

### Styling
- CSS Modules for component-specific styles
- Mantine theme system for global styling
- Custom CSS properties for dynamic theming

## Testing

The project uses Mocha for testing with Selenium WebDriver for UI automation. Test files are located in the `test/` directory.

## Build Configuration

### Vite (vite.config.js)
- Fixed port 1420 for Tauri compatibility
- ES2021 target for modern browsers
- Environment variable prefixes: `VITE_`, `TAURI_`

### Tauri (tauri.conf.json)
- Product name: "tauri-react-app"
- Bundle identifier: "com.tauri-app.app"
- Auto-updater configured for GitHub releases
- Window constraints: minimum 1000x600

## Common Development Tasks

### Adding New Routes
1. Create component in `src/views/`
2. Add to `views` array in `App.jsx:38`
3. Ensure proper i18n key exists

### Adding Tauri Commands
1. Add function to appropriate Rust module
2. Mark with `#[tauri::command]`
3. Add to `invoke_handler` in `lib.rs`
4. Call from frontend using `invoke()`

### Adding System Tray Menu Items
1. Update `create_tray_menu()` in `sys_tray.rs:28`
2. Add handler logic in `on_menu_event` callback
3. Test with different menu item IDs

### Managing State
- Use `TauriProvider` for Tauri-specific state
- Use Mantine's built-in state for UI components
- Use `localforage` for persistent data

## Plugin Configuration

The app uses several Tauri plugins:
- `tauri-plugin-store`: Persistent key-value storage
- `tauri-plugin-window-state`: Window position/size persistence
- `tauri-plugin-single-instance`: Prevent multiple app instances
- `tauri-plugin-updater`: Auto-update functionality
- `tauri-plugin-notification`: System notifications
- `tauri-plugin-dialog`: File dialogs
- `tauri-plugin-fs`: File system operations
- `tauri-plugin-shell`: Shell command execution
- `tauri-plugin-os`: OS information
- `tauri-plugin-process`: Process management