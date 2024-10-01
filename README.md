<p align="center">
	<br><span>Selectron testing</span><br>
</p>

## Overview

Written in **Rust** and **Javascript** on top of the cross-platform WebView rendering library [Wry/Tauri](https://crates.io/crates/wry).

All Javascript code is executed within embedded system-native Web Views - one for the NodeJS 'backend', one for each Electron App-GUI browser window.
The Electron and Node APIs are emulated with corresponding Rust API calls. All communcation between the Web Views and Rust runs on synchronous and asynchronous XMLHttpRequests.

### Features
-   build size down to about 10MB
-   cross platform for linux, macos, windows, ios and android
-   debugging frontend and backend with native browser dev-tools

As for now some basic Electron and Node APIs are - partly - implemented:
-   common Electron App startup and BrowserWindow methods
-   Electron IPC and preload.js
-   parts of the Electron dialog API (OpenDialog, SaveDialog, MessageBox)
-   basic parts of NodeJS file system API (access, fstat, mkdir, readfile, writefile, watch)
-   parts of NodeJS process API (child_process spawn)

### Try out the Test App (Folder /Resources)

The Test App is configured to start up by default when Electrico is started from the project folder

	pnpm install --recursive
	cargo run

When started in debug mode, it opens a browser dev tools window for the 'node backend' where debugging takes place. Also all GUI windows are shown with dev tools.

To start without dev tools, run

	cargo run --release

## Credits

- Fork of [Electrico](https://github.com/thomastschurtschenthaler/electrico) by Thomas Tschurtschenthaler
