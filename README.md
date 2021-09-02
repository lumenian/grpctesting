This project demonstrates a Tauri app communicating with a Go "app backend" via gRPC.

The UI is created using Svelte.js (https://svelte.dev/) and Tauri (https://tauri.studio/en/).

# Requirements

In order to run this project you will need the following:

## Go

The gRPC server and app backend are written in Go. (https://go.dev/)

## Rust & Tauri

Tauri requires Rust (https://www.rust-lang.org/). Depending on your OS, you will also need to
install the Tauri-specific development dependencies.

### Linux

(https://tauri.studio/en/docs/getting-started/setup-linux)

### Windows

(https://tauri.studio/en/docs/getting-started/setup-windows)

### macOS

(https://tauri.studio/en/docs/getting-started/setup-macos)

## Node.js

Tauri no longer requires Node.js but Svelte does. (https://nodejs.org/en/)

You can use NPM or Yarn. Yarn is preferred but optional. (https://yarnpkg.com/)

# Installation

From the main project folder run _npm install_ or _yarn_.

To start the development server run _npm run tauri dev_ or _yarn tauri dev_.

To start the gRPC server/app backend run _make server_.
