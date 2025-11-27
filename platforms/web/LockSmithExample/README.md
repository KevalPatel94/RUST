# LockSmith Web Example

A React + TypeScript web application demonstrating the LockSmith Rust core compiled to WebAssembly.

## Features

- **Localization**: Switch between English, Spanish, and French
- **Password Validation**: Real-time password validation with localized error messages
- **Demo Values**: Pre-filled passwords to test different validation scenarios
- **Modern UI**: Clean, responsive design with dark/light mode support

## Prerequisites

- Node.js 18+ and npm
- The WASM package must be built first (see main README)

## Setup

1. Build the WASM package from the repository root:
   ```bash
   just package web
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Start the development server:
   ```bash
   npm run dev
   ```

4. Open the URL shown in the terminal (typically `http://localhost:5173`)

## Build for Production

```bash
npm run build
```

The built files will be in the `dist/` directory.

## Project Structure

```
LockSmithExample/
├── src/
│   ├── App.tsx          # Main application component
│   ├── App.css          # Application styles
│   ├── main.tsx         # Application entry point
│   └── index.css        # Global styles
├── index.html           # HTML template
├── package.json         # Dependencies and scripts
├── tsconfig.json        # TypeScript configuration
├── vite.config.ts       # Vite configuration
└── README.md            # This file
```

## Usage

1. Select a language from the locale buttons (English, Spanish, or French)
2. Enter a password in the input field or click a sample password button
3. View the real-time validation feedback
4. Toggle password visibility with the Show/Hide button

## Notes

- The app depends on `locksmith` from `file:../pkg`. Re-run `just package web` after any Rust changes.
- The WASM module is loaded asynchronously on app startup.
- All text is localized using the Rust core's localization system.
