
<p align="center">
	<br><span>(Selectron source - updated: 2024-10-08)</span><br>
</p>

## Overview

Selectron uses Webviews running an Electron App-GUI frontend, and backend Node APIs emulated by Rust API calls via synchronous and asynchronous XMLHttpRequests.

### <- 🔗 back

<p align="left"><span><pre>

./
├── Resources
│   ├── [app.html](#resourcesapphtml)
│   ├── [main.js](#resourcesmainjs)
│   ├── [package.json](#resourcespackagejson)
│   ├── [preload.js](#resourcespreloadjs)
│   ├── [styles.css](#resourcesstylescss)
│   └── [terminal.js](#resourcesterminaljs)
├── src
│   ├── electron
│   │   ├── [electron.rs](#srcelectronelectronrs)
│   │   ├── [menu.rs](#srcelectronmenurs)
│   │   ├── [mod.rs](#srcelectronmodrs)
│   │   └── [types.rs](#srcelectrontypesrs)
│   ├── js
│   │   ├── backend
│   │   │   ├── [backend.html](#srcjsbackendbackendhtml)
│   │   │   ├── [electrico.js](#srcjsbackendelectricojs)
│   │   │   ├── [electron.js](#srcjsbackendelectronjs)
│   │   │   ├── [node.js](#srcjsbackendnodejs)
│   │   │   ├── [package-lock.json](#srcjsbackendpackage-lockjson)
│   │   │   └── [package.json](#srcjsbackendpackagejson)
│   │   ├── frontend
│   │   │   ├── [electrico.js](#srcjsfrontendelectricojs)
│   │   │   ├── [package-lock.json](#srcjsfrontendpackage-lockjson)
│   │   │   └── [package.json](#srcjsfrontendpackagejson)
│   │   └── shared
│   │       ├── [require.js](#srcjssharedrequirejs)
│   │       └── [shared.js](#srcjssharedsharedjs)
│   ├── node
│   │   ├── [common.rs](#srcnodecommonrs)
│   │   ├── [ipc.rs](#srcnodeipcrs)
│   │   ├── [mod.rs](#srcnodemodrs)
│   │   ├── [node.rs](#srcnodenoders)
│   │   ├── [process.rs](#srcnodeprocessrs)
│   │   └── [types.rs](#srcnodetypesrs)
│   ├── [backend.rs](#srcbackendrs)
│   ├── [common.rs](#srccommonrs)
│   ├── [frontend.rs](#srcfrontendrs)
│   ├── [ipcchannel.rs](#srcipcchannelrs)
│   ├── [main.rs](#srcmainrs)
│   └── [types.rs](#srctypesrs)
├── [.gitignore](#gitignore)
├── [Cargo.toml](#Cargotoml)
├── [README.md](#READMEmd)
└── [ResourcesLink.json](#ResourcesLinkjson)

</pre>
</span>
</p>

## Source

/.gitignore:
-----------------------

.DS_Store
/.cargo
/Cargo.lock
/target
/Resources/node_modules
Resources/package-lock.json


-----------------------

/.pnpm-workspace.yaml:
-----------------------

packages:
  - 'Resources/*'
  - 'src/js/backend/*'
  - 'src/js/frontend/*'


-----------------------

/Cargo.toml:
-----------------------

<p align="left"><span><pre>

```
[package]
name = "electrico"
version = "0.5.0"
edition = "2021"
authors = ["Thomas Tschurtschenthaler"]
license = "MIT OR Apache-2.0"
description = "Lightweight Electron 'compatible' App Container."

[package.metadata.bundle.ttschurtschenthaler.electrico]
name = "Electrico"
identifier = "com.ttschurtschenthaler.electrico"
version = "1.0.0"
copyright = "Copyright (c) Thomas Tschurtschenthaler 2024. All rights reserved."
category = "App"
short_description = "Lightweight Electron 'compatible' App Container."
long_description = """
"""

[dependencies]
log = {version = "0.4.22"}
env_logger = {version = "0.11.5"}
include_dir = {version = "0.7.3"}
wry = { version = "0.45.0", features = ["linux-body"]}
tao = { version = "0.30.2"}
muda = {version = "0.15.0"}
serde_json = { version = "1.0.128"}
json_comments = {version = "0.2.2"}
serde = {version = "1.0.210"}
rfd = {version="0.15.0"}
tokio = {version = "1.40.0", features = ["full"]}
reqwest = {version = "0.12.8"}
substring = {version = "1.4.5"}
open = {version = "5.3.0"}
mime_guess = {version = "2.0.5"}
lazy_static = {version = "1.5.0"}
base64 = {version = "0.22.1"}
directories = {version = "5.0.1"}
notify = {version = "6.1.1"}
interprocess = {version = "2.2.1", features = ["tokio"]}
uuid = { version = "1.10.0", features = ["v4"]}
urlencoding = {version = "2.1.3"}
queues = {version = "1.1.0"}

[target."cfg(any(target_os = \"linux\", target_os = \"dragonfly\", target_os = \"freebsd\", target_os = \"openbsd\", target_os = \"netbsd\"))".dependencies]
webkit2gtk = { version = "=2.0.1", features = [ "v2_40" ]}
```

</pre></span></p>

-----------------------

/Overview.md:
-----------------------


<p align="center">
	<br><span>(Overview by Qwen Coder)</span><br>
</p>


<p align="left"><span>

# Electrico

Electrico appears to be a desktop application built using Electron with a Rust backend. Here’s a more detailed summary of what Electrico does:

## Overview

Electrico includes a demo application (see Resources) that showcases version information and provides an interface for interacting with system-level operations.
The application uses Electron for the frontend and Rust for the backend, leveraging Webview2 for rendering web content.

## Frontend (JavaScript/HTML/CSS)

- **[app.html](#resourcesapphtml)** - The main HTML file that serves as the entry point for the application.
- **[main.js](#resourcesmainjs)** - A JavaScript file that handles the initialization of the Electron application and sets up the IPC communication.
- **[preload.js](#resourcespreloadjs)** - This script is executed in the context of the web page before the renderer process starts. It uses `contextBridge` to expose specific functions from the main process to the renderer process, enhancing security by limiting direct access to the main process's functionality.
- **[styles.css](#resourcesstylescss)** - Stylesheet for customizing the appearance of the application.
- **[terminal.js](#resourcesterminaljs)** - A JavaScript file that likely handles terminal-like interactions within the application.
##### &
- **[package.json](#resourcespackagejson)** - for installing Electron, thou [it's not needed by Electrico.](https://github.com/thomastschurtschenthaler/electrico/issues/5#issuecomment-2388206145)

## Backend (Rust)

The Rust backend is structured as follows:

- **[electron.rs](#srcelectronelectronrs)** - Contains code related to Electron integration, possibly including initialization and event handling.
- **[menu.rs](#srcelectronmenurs)** - Manages the application's menu bar, allowing users to interact with different features.
- **[mod.rs](#srcelectronmodrs)** - The root module for the `electron` crate, which likely includes reexports of other modules.
- **[types.rs](#srcelectrontypesrs)** - Define data types used in the Electron backend.

The Rust backend also includes a separate directory for handling Node.js functionality:

- **[node.rs](#srcnodenoders)** - Contains code related to Node.js integration.
- **[mod.rs](#srcnodemodrs)** - The root module for the `node` crate, which likely includes reexports of other modules.
- **[types.rs](#srcnodetypesrs)** - Define data types used in the Node.js backend.

## Main Application Logic

- **[backend.rs](#srcbackendrs)** - Manages the backend logic, possibly including interactions with external services or system-level operations.
- **[common.rs](#srccommonrs)** - Contains common code that can be reused across different parts of the application.
- **[frontend.rs](#srcfrontendrs)** - Handles the frontend logic, likely interacting with the Electron and Node.js backends.
- **[ipcchannel.rs](#srcipcchannelrs)** - Manages IPC communication between the main process and renderer processes.
- **[main.rs](#srcmainrs)** - The entry point for the Rust application, initializing the Electron and Node.js environments.
- **[types.rs](#srctypesrs)** - Define data types for the main app logic.

## Additional Files

- **[package.json](#resourcespackagejson)** - Contains metadata about the sample project and dependencies.
- **[Cargo.toml](#Cargotoml)** - Defines the project's build configuration and dependencies.
- **[README.md](#READMEmd)** - Provides documentation and instructions for setting up and running the project.
- **[ResourcesLink.json](#ResourcesLinkjson)** - Likely contains configuration or linking information for the application.

## Summary

Electrico is a desktop application that combines Electron for the frontend with Rust for the backend, using Webview2 for rendering web content.
The application showcases version information and provides an interface for interacting with system-level operations.
The Rust backend handles the core logic, while the JavaScript frontend manages user interactions and IPC communication.
This setup allows for a powerful combination of web technologies and native Rust capabilities, providing a robust platform for building cross-platform desktop applications.

</span></p>


-----------------------

/README.md:
-----------------------

<p align="center">
	<br><span>Selectron</span><br>
</p>

<p align="left"><span>

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

</span></p>


-----------------------

/Resources/app.html:
-----------------------


<p align="left"><span><pre>

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8">
    <link href="./styles.css" rel="stylesheet">
    <title>Hello Electrico</title>
  </head>
  <body>
    <h1>Hello Electrico!</h1>
    We are using Node.js <span id="node-version"></span>,
    Chromium <span id="chrome-version"></span>,
    and Electron <span id="electron-version"></span>.
    <div id="terminal"></div>
    <script src="./terminal.js"></script>
  </body>
</html>
```

</pre></span></p>

-----------------------

/Resources/main.js:
-----------------------

<p align="left"><span><pre>

```javascript
const { app, ipcMain, BrowserWindow} = require('electron/main')
const path = require('node:path');
const { spawn } = require('node:child_process');
const fs = require('node:fs');

let mainWindow=null;
function writeOutput(text, level) {
  mainWindow.webContents.send("writeOutput", text, level);
}

ipcMain.on('shellcommand', function(event, command) {
  if (command.cmd=="") {
    writeOutput("command empty\n", "error");
    return;
  }
  const child = spawn(command.cmd, command.args);
  writeOutput(`child process started with pid: ${child.pid}\n`, "info");
  child.stdout.on('data', (data) => {
      writeOutput(`${data}`, "info");
  });

  child.stderr.on('data', (data) => {
      writeOutput(`${data}`, "error");
  });

  child.on('close', (code) => {
      writeOutput(`child process exited with code ${code}\n`, code==0?"info":"error");
  });
  if (command.stdin!="") {
      child.stdin.write(command.stdin);
  }
  event.returnValue=child.pid;
});
let watcher = null; let watchpath = null;
ipcMain.on('startwatch', function(event, path) {
  if (path=="") {
    writeOutput("path empty\n", "error");
    return;
  }
  if (watcher!=null) {
    watcher.close();
  }
  watcher = fs.watch(path, {recursive:true});
  watcher.on("change", (type, file) => {
    writeOutput(`file watch event: ${type}; ${file}\n`, "info");
  });
  watchpath=path;
  writeOutput(`file watch started: ${path}\n`, "info");
});
ipcMain.on('stopwatch', function(event) {
  if (watcher!=null) {
    watcher.close();
    writeOutput(`file watch stopped: ${watchpath}\n`, "info");
    watcher=null;
    watchpath=null;
  }
});

function createWindow () {
  mainWindow = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      preload: path.join(__dirname, 'preload.js')
    }
  })

  mainWindow.loadFile('app.html');
}
app.setName("Electrico Testapp");

app.whenReady().then(() => {
  createWindow()
  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow()
    }
  });
})

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit()
  }
})
```

</pre></span></p>

-----------------------

/Resources/package.json:
-----------------------

<p align="left"><span><pre>

```json
{
  "name": "electrico-example",
  "version": "1.0.0",
  "description": "Electrico Example App",
  "build": {
    "productName": "Electrico Example App"
  },
  "main": "main.js",
  "author": "Thomas Tschurtschenthaler",
  "scripts": {
    "start": "electron ."
  },
  "devDependencies": {
   
  },
  "dependencies": {
    "electron": "32.1.2"
  }
}
```

</pre></span></p>

-----------------------

/Resources/preload.js:
-----------------------

<p align="left"><span><pre>

```javascript
window.addEventListener('DOMContentLoaded', () => {
    const replaceText = (selector, text) => {
      const element = document.getElementById(selector)
      if (element) element.innerText = text
    }
  
    for (const type of ['chrome', 'node', 'electron']) {
      replaceText(`${type}-version`, process.versions[type])
    }
});
const { ipcRenderer, contextBridge } = require('electron');
contextBridge.exposeInMainWorld("callshell", (command)=>{
  ipcRenderer.send("shellcommand", command);
});
contextBridge.exposeInMainWorld("onWriteOutput", (callback) => {
  ipcRenderer.removeAllListeners("writeOutput");
  ipcRenderer.on("writeOutput", callback);
});
contextBridge.exposeInMainWorld("platform", process.platform);
contextBridge.exposeInMainWorld("startwatch", (path)=>{
  ipcRenderer.send("startwatch", path);
});
contextBridge.exposeInMainWorld("stopwatch", ()=>{
  ipcRenderer.send("stopwatch");
});
```

</pre></span></p>

-----------------------

/Resources/styles.css:
-----------------------

<p align="left"><span><pre>

```css
.output {
    color:beige;
    background-color: darkslateblue;
    width: 50rem;
    height: 30rem;
    overflow: auto;
}
.info {
    color:greenyellow;
}
.error {
    color:lightsalmon;
}
.path {
    width: 30rem;
}
```

</pre></span></p>

-----------------------

/Resources/terminal.js:
-----------------------

<p align="left"><span><pre>

```javascript
let html = `<h2>Send Shell Command</h2>
            <div>
                <label for="cmd">Command:</label>
                <input type="text" id="cmd" value="${platform=='win32'?'cmd':'ls'}">
                <label for="args">Arguments:</label>
                <input type="text" id="args" value="${platform=='win32'?'/C dir':'-ltr'}">
                <label for="stdin">stdin:</label>
                <input type="text" id="stdin">
                <button id="send">Send</button>
            </div>
            <div>
                <label for="cmd">Watch File Path:</label>
                <input type="text" class="path" id="path" value="/Users/thomastschurtschenthaler/Documents/workspaces/electrico">
                <button id="startwatch">Start</button>
                <button id="stopwatch">Stop</button>
            </div>
            <div>
                <label for="output">Output:</label>
                <div class="output" id="output"></div>
            </div>`;
document.getElementById("terminal").innerHTML=html;
document.getElementById("send").onclick = (e) => {
    let cmd = document.getElementById("cmd").value.trim();
    let args = document.getElementById("args").value.trim();
    let stdin = document.getElementById("stdin").value.trim();
    args = args=args!=""?args.split(" "):null;
    callshell({cmd:cmd, args:args, stdin:stdin});
};
document.getElementById("startwatch").onclick = (e) => {
    let path = document.getElementById("path").value.trim();
    startwatch(path);
};
document.getElementById("stopwatch").onclick = (e) => {
    stopwatch();
};
window.onWriteOutput((event, text, level) => {
    text = text.replaceAll("<", "&lt;").replaceAll(">", "&gt;").replaceAll("\r\n", "<br>").replaceAll("\n", "<br>").replaceAll("\r", "<br>");
    let html = level!=null?("<span class='"+level+"'>"+text+"</span>"):text;
    document.getElementById("output").innerHTML+=html;
});
```

</pre></span></p>

-----------------------

/ResourcesLink.json:
-----------------------

{
    //"link": "<link to 'Resources' folder of a electron app>"
}


-----------------------

/src/backend.rs:
-----------------------

<p align="left"><span><pre>

```rust
use std::{collections::HashMap, fs::File, path::PathBuf, sync::mpsc::{self, Receiver, Sender}};
use muda::MenuId;
use notify::{Event, RecommendedWatcher};
use substring::Substring;
use log::{debug, error, trace};
use include_dir::{include_dir, Dir};
use tao::{dpi::PhysicalSize, event_loop::{EventLoop, EventLoopProxy}, window::{Window, WindowBuilder}};
use urlencoding::decode;
use wry::{http::Request, RequestAsyncResponder, WebView, WebViewBuilder};
use serde_json::Error;
use crate::{common::{append_js_scripts, build_file_map, escape, handle_file_request, is_module_request, respond_404, DataQueue}, ipcchannel::IPCMsg, types::{BackendCommand, ChildProcess, NETConnection, NETServer}};
use crate::types::{Package, ElectricoEvents, Command};

pub struct Backend {
    _window:Window,
    webview:WebView,
    command_sender:Sender<BackendCommand>,
    command_receiver:Receiver<BackendCommand>,
    child_process:HashMap<String, Sender<ChildProcess>>,
    fs_watcher:HashMap<String, RecommendedWatcher>,
    fs_files:HashMap<i64, File>,
    net_server:HashMap<String, Sender<NETServer>>,
    net_connections:HashMap<String, Sender<NETConnection>>,
    data_queue:DataQueue
}

impl Backend {
    pub fn new(src_dir:PathBuf, package:&Package, event_loop:&EventLoop<ElectricoEvents>, proxy:EventLoopProxy<ElectricoEvents>) -> Backend {
        let (command_sender, command_receiver): (Sender<BackendCommand>, Receiver<BackendCommand>) = mpsc::channel();

        let mut backendjs:String = String::new();
        const JS_DIR_SHARED: Dir = include_dir!("src/js/shared");
        backendjs = append_js_scripts(backendjs, JS_DIR_SHARED, Some(".js"));
        const JS_DIR_BACKEND: Dir = include_dir!("src/js/backend");
        backendjs = append_js_scripts(backendjs, JS_DIR_BACKEND, Some("electrico.js"));
        let backend_js_files = build_file_map(&JS_DIR_BACKEND);
        let init_script = backendjs+"\nwindow.__electrico.loadMain('"+package.main.to_string().as_str()+"');";
        
        let mut window_builder = WindowBuilder::new()
            .with_title("Electrico Node backend");

        #[cfg(target_os = "macos")] {
            #[cfg(debug_assertions)] {
                window_builder = window_builder.with_inner_size(PhysicalSize::new(1,1));
            }
            #[cfg(not(debug_assertions))] {
                window_builder = window_builder.with_visible(false);
            }
        }
        #[cfg(not(target_os = "macos"))] {
            window_builder = window_builder.with_visible(false);
        }

        let window = window_builder
            .build(event_loop)
            .unwrap();
        
        let cmd_handler = move |request: Request<Vec<u8>>, responder:RequestAsyncResponder| {
            let path = request.uri().path().to_string();
            trace!("backend cmd request {} {}", path, request.body().len());
            let cmdmsg:String;
            let data_blob:Option<Vec<u8>>;
            if let Some(queryenc) = request.uri().query() {
                if let Ok(query) = decode(queryenc) {
                    cmdmsg=query.to_string();
                    data_blob = Some(request.body().to_vec());
                } else {
                    error!("url decoder error");
                    respond_404(responder);
                    return;
                }
            } else {
                let msgr =  String::from_utf8(request.body().to_vec());
                match msgr {
                    Ok(msg) => {
                        trace!("backend cmd request body {}", msg.as_str());
                        cmdmsg=msg;
                        data_blob=None;
                    },
                    Err(e) => {
                        error!("utf8 error {}", e);
                        respond_404(responder);
                        return;
                    }
                }
            }
            
            let commandr:Result<Command, Error> = serde_json::from_str(cmdmsg.as_str());
            match commandr {
                Ok (command) => {
                    let _ = proxy.send_event(ElectricoEvents::ExecuteCommand{command, responder, data_blob});
                }
                Err(e) => {
                    error!("json serialize error {}", e.to_string());
                    respond_404(responder);
                    return;
                }
            }
        };
        let tokio_runtime = tokio::runtime::Builder::new_multi_thread().worker_threads(5).enable_io().enable_time().build().unwrap();
        
        let fil_handler = move |request: Request<Vec<u8>>, responder:RequestAsyncResponder| {
            let rpath = request.uri().path().to_string();
            trace!("backend fil: request {}", rpath);
            let fpath = rpath.substring(1, rpath.len()).to_string();
            
            let file = src_dir.join(fpath.clone());
            trace!("trying load file {}", file.clone().as_mut_os_str().to_str().unwrap());
            handle_file_request(&tokio_runtime, is_module_request(request.uri().host()), fpath, file, &backend_js_files, responder);
        };
        let mut is_windows="false";
        #[cfg(target_os = "windows")] {
            is_windows = "true";
        }

        #[cfg(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        ))]
        let builder = WebViewBuilder::new(&window);
    
        #[cfg(not(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        )))]
        let builder = {
            use tao::platform::unix::WindowExtUnix;
            use wry::WebViewBuilderExtUnix;
            let vbox = window.default_vbox().unwrap();
            WebViewBuilder::new_gtk(vbox)
        };

        let webview = builder
            .with_url("fil://file/backend.html")
            .with_asynchronous_custom_protocol("fil".into(), fil_handler)
            .with_asynchronous_custom_protocol("cmd".into(), cmd_handler)
            .with_devtools(true)
            .with_incognito(true)
            .with_initialization_script(("window.__is_windows=".to_string()+is_windows+";"+init_script.as_str()).as_str())
            .build().unwrap();
          
        #[cfg(debug_assertions)]
        webview.open_devtools();
        Backend {
            _window:window,
            webview:webview,
            command_sender,
            command_receiver,
            child_process: HashMap::new(),
            fs_watcher: HashMap::new(),
            fs_files: HashMap::new(),
            net_server: HashMap::new(),
            net_connections: HashMap::new(),
            data_queue: DataQueue::new()
        }
    }
    pub fn command_callback(&mut self, command:String, message:String) {
        let _ = self.webview.evaluate_script(format!("window.__electrico.callback['{}']('{}')", command, message).as_str());
    }
    pub fn call_ipc_channel(&mut self, browser_window_id:&String, request_id:&String, params:String, sender:Sender<IPCMsg>) {
         let request_id2 = request_id.clone();
         trace!("call_ipc_channel {} {}", &request_id2, &params);
         _ = self.webview.evaluate_script_with_callback(
            format!("window.__electrico.callIPCChannel('{}@{}@@{}');", browser_window_id, request_id, escape(&params)).as_str()
            , move |r| {
                if r.len()>0 {
                    let _ = sender.send(IPCMsg::Called);
                    trace!("call_ipc_channel OK {}", &request_id2);
                } else {
                    trace!("call_ipc_channel not OK {}", &request_id2);
                }
            });
    }
    pub fn window_close(&mut self, id:&Option<String>) {
        if let Some(id) = id {
            let _ = self.webview.evaluate_script(format!("window.__electrico.callAppOn('window-close', '{}');", id).as_str());
        } else {
            let _ = self.webview.evaluate_script(format!("window.__electrico.callAppOn('window-close');").as_str());
        }
    }
    pub fn window_all_closed(&mut self) {
        let _ = self.webview.evaluate_script(format!("window.__electrico.callAppOn('window-all-closed');").as_str());
    }
    pub fn menu_selected(&mut self, id:MenuId) {
        let _ = self.webview.evaluate_script(format!("window.__electrico.menuSelected('{}');", id.as_ref()).as_str());
    }
    pub fn dom_content_loaded(&mut self, id:&String) {
        let _ = self.webview.evaluate_script(format!("window.__electrico.domContentLoaded('{}');", id).as_str());
    }
    pub fn child_process_callback(&mut self, pid:String, stream:String, data:Option<Vec<u8>>) {
        if stream=="stdin" {
            if let Some(sender) = self.child_process.get(&pid) {
              trace!("ChildProcessData stdin {} {:?}", pid, data);
              if let Some(data) = data {
                let _ = sender.send(ChildProcess::StdinWrite {data});
              }
            }
        } else {
            trace!("child_process_callback {} {}", stream, pid);
            if let Some(data) = data {
                self.data_queue.add(&pid, data);
            }
            let retry_sender = self.command_sender.clone();
            let _ = self.webview.evaluate_script_with_callback(&format!("window.__electrico.call(()=>{{window.__electrico.child_process.callback.on_{}('{}');}});", stream, pid), move |r| {
                if r.len()==0 {
                    trace!("child_process_callback not OK - resending");
                    let _ = retry_sender.send(BackendCommand::ChildProcessCallback { pid:pid.clone(), stream:stream.clone(), data:None });
                }
            });
        }
    }
    pub fn child_process_exit(&mut self, pid:String, exit_code:Option<i32>) {
        let call_script:String;
        if let Some(exit_code) = exit_code {
            call_script=format!("window.__electrico.child_process.callback.on_close('{}', {});", pid, exit_code.to_string());
        } else {
            call_script=format!("window.__electrico.child_process.callback.on_close('{}');", pid);
        }
        let retry_sender = self.command_sender.clone();
        let _ = self.webview.evaluate_script_with_callback(&format!("window.__electrico.call(()=>{{{}}});", call_script.as_str()), move |r| {
            if r.len()==0 {
                trace!("child_process_exit not OK - resending");
                let _ = retry_sender.send(BackendCommand::ChildProcessExit { pid: pid.clone(), exit_code: exit_code.clone() });
            }
        });
    }
    pub fn fs_watch_callback(&mut self, wid:String, event:Event) {
        let call_script = format!("window.__electrico.fs_watcher.on_event('{}', '{:?}', '{}')",
            wid, 
            event.kind,
            escape(&event.paths.iter().map(|x| x.as_os_str().to_str().unwrap()).collect::<Vec<_>>().join(";")));
        let retry_sender = self.command_sender.clone();
        let _ = self.webview.evaluate_script_with_callback(&format!("window.__electrico.call(()=>{{{}}});", call_script.as_str()), move |r| {
            if r.len()==0 {
                trace!("fs_watch_callback not OK - resending");
                let _ = retry_sender.send(BackendCommand::FSWatchEvent { wid:wid.clone(), event:event.clone() });
            }
        });
    }
    pub fn command_sender(&mut self) -> Sender<BackendCommand> {
        self.command_sender.clone()
    }
    pub fn child_process_start(&mut self, pid:String, sender:Sender<ChildProcess>) {
        trace!("child_process_start {}", pid);
        self.child_process.insert(pid, sender);
    }
    pub fn child_process_disconnect(&mut self, pid:String) {
        trace!("child_process_disconnect {}", pid);
        if let Some(sender) = self.child_process.get(&pid) {
            let _ = sender.send(ChildProcess::Disconnect);
        }
    }
    pub fn fs_open(&mut self, fd:i64, file:File) {
        trace!("fs_open {}", fd);
        self.fs_files.insert(fd, file);
    }
    pub fn fs_close(&mut self, fd:i64) {
        trace!("fs_close {}", fd);
        if let Some(_file) = self.fs_files.get(&fd) {
            self.fs_files.remove(&fd);
        }
    }
    pub fn fs_get(&mut self, fd:i64) -> Option<&File>{
        trace!("fs_get {}", fd);
        return self.fs_files.get(&fd);
    }
    pub fn watch_start(&mut self, wid:String, watcher:RecommendedWatcher) {
        trace!("watch_start {}", wid);
        self.fs_watcher.insert(wid, watcher);
    }
    pub fn watch_stop(&mut self, wid:String) {
        trace!("watch_stop {}", wid);
        if let Some(_watcher) = self.fs_watcher.get(&wid) {
            self.fs_watcher.remove(&wid);
        }
    }
    pub fn net_server_conn_start(&mut self, hook:String, id:String, sender:Sender<NETConnection>) {
        let call_script=format!("window.__electrico.net_server.callback.on_start('{}', '{}');", hook, id);
        self.net_connections.insert(id.clone(), sender.clone());
        let retry_sender = self.command_sender.clone();
        let _ = self.webview.evaluate_script_with_callback(&format!("window.__electrico.call(()=>{{{}}});", call_script.as_str()), move |r| {
            if r.len()==0 {
                trace!("net_server_conn_start not OK - resending");
                let _ = retry_sender.send(BackendCommand::NETServerConnStart { hook:hook.clone(), id:id.clone(), sender:sender.clone()});
            }
        });
    }
    pub fn net_server_close(&mut self, id:String) {
        if let Some(sender) = self.net_server.get(&id) {
            let _ = sender.send(NETServer::Close);
            self.net_server.remove(&id);
        }
    }
    pub fn net_connection_close(&mut self, id:String) {
        if let Some(sender) = self.net_connections.get(&id) {
            let _ = sender.send(NETConnection::EndConnection);
        }
    }
    pub fn net_client_conn_start(&mut self, id:String, sender:Sender<NETConnection>) {
        self.net_connections.insert(id.clone(), sender.clone());
    }
    pub fn net_connection_data(&mut self, id:String, data:Option<Vec<u8>>) {
        if let Some(data) = data {
            self.data_queue.add(&id, data);
        }
        let call_script=format!("window.__electrico.net_server.callback.on_data('{}');", id);
        let retry_sender = self.command_sender.clone();
        let _ = self.webview.evaluate_script_with_callback(&format!("window.__electrico.call(()=>{{{}}});", call_script.as_str()), move |r| {
            if r.len()==0 {
                trace!("net_connection_data not OK - resending");
                let _ = retry_sender.send(BackendCommand::NETConnectionData { id:id.clone(), data:None });
            }
        });
    }
    pub fn net_connection_end(&mut self, id:String) {
        if let Some(sender) = self.net_connections.get(&id) {
            let _ = sender.send(NETConnection::Disconnect);
            self.net_connections.remove(&id);
        }
        let call_script=format!("window.__electrico.net_server.callback.on_end('{}');", id);
        let retry_sender = self.command_sender.clone();
        let _ = self.webview.evaluate_script_with_callback(&format!("window.__electrico.call(()=>{{{}}});", call_script.as_str()), move |r| {
            if r.len()==0 {
                trace!("net_connection_end not OK - resending");
                let _ = retry_sender.send(BackendCommand::NETConnectionEnd { id:id.clone()});
            }
        });
    }
    pub fn net_write_connection(&mut self, id:String, data:Vec<u8>) {
        if let Some(sender) = self.net_connections.get(&id) {
            let _ = sender.send(NETConnection::Write { data });
        } else {
            error!("net_write_connection no sender for id {}", id);
        }
    }
    pub fn get_data_blob(&mut self, id:String) -> Option<Vec<u8>> {
        let data:Option<Vec<u8>>;
        if let Some(d) = self.data_queue.take(&id) {
            data = Some(d.to_vec());
        } else {
            data = None;
        };
        return data;
    }
    pub fn process_commands(&mut self) {
        if let Ok(command) = self.command_receiver.try_recv() {
            match command {
                BackendCommand::ChildProcessCallback { pid, stream, data } => {
                    trace!("ChildProcessCallback");
                    self.child_process_callback(pid, stream, data);
                },
                BackendCommand::ChildProcessExit { pid, exit_code } => {
                    trace!("ChildProcessExit");
                    if self.child_process.contains_key(&pid) {
                        self.child_process.remove(&pid);
                    }
                    self.child_process_exit(pid, exit_code);
                }
                BackendCommand::FSWatchEvent { wid, event } => {
                    trace!("FSWatchEvent");
                    self.fs_watch_callback(wid, event);
                },
                BackendCommand::NETServerStart { id, sender } => {
                    trace!("NETServerStart");
                    self.net_server.insert(id, sender);
                }
                BackendCommand::NETServerConnStart { hook, id, sender } => {
                    trace!("NETServerConnStart {}", hook);
                    self.net_server_conn_start(hook, id, sender);
                },
                BackendCommand::NETClientConnStart { id, sender } => {
                    trace!("NETClientConnStart {}", id);
                    self.net_client_conn_start(id, sender);
                },
                BackendCommand::NETConnectionData {id,  data } => {
                    trace!("NETConnectionData {}", id);
                    self.net_connection_data(id, data);
                },
                BackendCommand::NETConnectionEnd { id } => {
                    trace!("NETServerConnEnd {}", id);
                    self.net_connection_end(id);
                },
            }
        }
    }
    pub fn shutdown(&mut self) {
        self.fs_watcher.clear();
        self.fs_files.clear();
    }
}
```

</pre></span></p>

-----------------------

/src/common.rs:
-----------------------

<p align="left"><span><pre>

```rust
use std::{collections::HashMap, fs::{self, File}, path::{Path, PathBuf}};

use include_dir::{include_dir, Dir};
use queues::{IsQueue, Queue};
use reqwest::{header::{ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE}, StatusCode};
use tokio::runtime::Runtime;
use wry::{http::Response, RequestAsyncResponder};
use log::{debug, info, trace};
use std::io::Read;

pub const CONTENT_TYPE_TEXT: &str = "text/plain;charset=utf-8";
pub const CONTENT_TYPE_HTML: &str = "text/html;charset=utf-8";
pub const CONTENT_TYPE_CSS: &str = "text/css;charset=utf-8";
pub const CONTENT_TYPE_JSON: &str = "application/json;charset=utf-8";
pub const CONTENT_TYPE_JS: &str = "text/javascript;charset=utf-8";
pub const CONTENT_TYPE_BIN: &str = "application/octet-stream";
pub const JS_DIR_FRONTEND: Dir = include_dir!("src/js/frontend");

pub fn append_js_scripts(script:String, dir:Dir, filter:Option<&str>) -> String {
    let mut res = script.clone();
    for f in dir.files() {
        let path = f.path().file_name().unwrap().to_str().unwrap().to_string();
        if let Some(filter) = filter {
            if path.ends_with(filter) {
                res += f.contents_utf8().unwrap_or("");
            }
        }
    }
    res
}

pub fn build_file_map(dir:&Dir) -> HashMap<String, Vec<u8>> {
    let mut res:HashMap<String, Vec<u8>> = HashMap::new();
    for f in dir.files() {
        let path = f.path().to_str().unwrap().to_string();
        let content = f.contents();
        res.insert(path, Vec::from(content));
    }
    for d in dir.dirs() {
        let d_res = build_file_map(d);
        res.extend(d_res.into_iter())
    }
    res
}

fn respond_not_found(module:bool, responder:RequestAsyncResponder) {
    if module {
        respond_status(StatusCode::MOVED_PERMANENTLY, CONTENT_TYPE_HTML.to_string(), "package".to_string().into_bytes(), responder);
    } else {
        respond_status(StatusCode::NOT_FOUND, CONTENT_TYPE_HTML.to_string(), "not found".to_string().into_bytes(), responder);
    }
}

pub fn handle_file_request(tokio_runtime:&Runtime, module:bool, path:String, full_path:PathBuf, resources:&HashMap<String, Vec<u8>>, responder:RequestAsyncResponder)  {
    let resources_rt=resources.clone();
    tokio_runtime.spawn(
        async move {
            if full_path.exists() {
                match File::open(full_path.clone()) {
                    Ok (mut f) => {
                        let mut buffer = Vec::new();
                        match f.read_to_end(&mut buffer) {
                            Ok(_r) => {
                                respond_status(
                                    StatusCode::OK, 
                                    mime_guess::from_path(full_path).first_or_octet_stream().to_string(), 
                                    buffer, responder);
                            },
                            Err(_e) => {
                                trace!("file not found {}", full_path.to_str().unwrap());
                                respond_not_found(module, responder);
                            }
                        }
                        
                    },
                    Err (_e) => {
                        trace!("file not found {}", full_path.as_os_str().to_str().unwrap());
                        respond_not_found(module, responder);
                    }
                }
            } else {
                if let Some(res) = resources_rt.get(&path) {
                    respond_status(
                        StatusCode::OK, 
                        mime_guess::from_path(path).first_or_octet_stream().to_string(), 
                        res.to_vec(), responder); 
                } else {
                    trace!("file not found {}", path);
                    respond_not_found(module, responder);
                }
            }
        }
    );
}

pub fn escape(s:&String) -> String {
    s.replace("\\", "\\\\").replace("'", "\\'").replace("\n", "\\n").replace("\r", "\\r")
}

pub fn respond_ok(responder:RequestAsyncResponder) {
    responder.respond(Response::builder()
        .status(StatusCode::OK)
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(CONTENT_TYPE, CONTENT_TYPE_HTML)
        .body(Vec::from("OK".to_string().as_bytes())).unwrap())
}

pub fn respond_404(responder:RequestAsyncResponder) {
    responder.respond(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(CONTENT_TYPE, CONTENT_TYPE_HTML)
        .body(Vec::from("404".to_string().as_bytes())).unwrap())
}

pub fn respond_status(status:StatusCode, content_type: String, body:Vec<u8>, responder:RequestAsyncResponder) {
    responder.respond(Response::builder()
        .status(status)
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(CONTENT_TYPE, content_type)
        .body(body).unwrap())
}

pub fn respond_client_error(error:String, responder:RequestAsyncResponder) {
    responder.respond(Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(CONTENT_TYPE, CONTENT_TYPE_TEXT)
        .body(Vec::from(error.as_bytes())).unwrap())
}

pub fn check_and_create_dir(dir:&Path) {
    if !dir.exists() {
        info!("creating directory: {}", dir.as_os_str().to_str().unwrap());
        let _ = fs::create_dir_all(dir);
    }
}
pub fn is_module_request(host:Option<&str>) -> bool {
    if let Some(host) = host {
        if host=="mod" {
            return true;
        }
    }
    false
}
pub struct DataQueue {
    data_blobs:HashMap<String, Queue<Vec<u8>>>
}
impl DataQueue {
    pub fn new() -> DataQueue {
        DataQueue {
            data_blobs:HashMap::new()
        }
    }
    pub fn add(&mut self, k:&String, data:Vec<u8>) {
        if let Some(q) = self.data_blobs.get_mut(k) {
           let _ = q.add(data);
        } else {
            let mut q = Queue::new();
            let _ = q.add(data);
            self.data_blobs.insert(k.clone(), q);
        }
    }
    pub fn take(&mut self, k:&String) -> Option<Vec<u8>>{
        if let Some(q) = self.data_blobs.get_mut(k) {
            let ret:Option<Vec<u8>>;
            if let Ok(data) = q.peek() {
                let _ = q.remove();
                ret = Some(data);
            } else {
                ret = None;
            }
            if q.size()==0 {
                self.data_blobs.remove(k);
            }
            return ret;
        }
        None
    }
}
```

</pre></span></p>

-----------------------

/src/electron/electron.rs:
-----------------------

<p align="left"><span><pre>

```rust
use std::{path::{Path, PathBuf}, sync::mpsc::{self, Receiver, Sender}, time::Duration};
use log::{debug, info, trace};
use muda::Menu;
use reqwest::StatusCode;
use rfd::MessageLevel;
use tao::event_loop::{ControlFlow, EventLoopProxy, EventLoopWindowTarget};
use tokio::{runtime::Runtime, time::sleep};
use wry::RequestAsyncResponder;

use crate::{backend::Backend, common::{check_and_create_dir, respond_404, respond_ok, respond_status, CONTENT_TYPE_JSON, CONTENT_TYPE_TEXT}, electron::types::BrowserWindowDevToolsCall, frontend::Frontend, node::node::AppEnv, types::{ElectricoEvents, Package}};
use super::{menu::create_menu, types::{BrowserWindowBoundsAction, BrowserWindowMaximizedAction, BrowserWindowMinimizedAction, ElectronCommand}};

pub fn process_electron_command(tokio_runtime:&Runtime, event_loop:&EventLoopWindowTarget<ElectricoEvents>, proxy:EventLoopProxy<ElectricoEvents>,
    app_env:&mut AppEnv, rsrc_dir:&PathBuf, package:&Package,
    frontend:&mut Frontend, backend:&mut Backend, command:ElectronCommand,responder:RequestAsyncResponder) -> Option<Menu> {
    let mut menu_ret: Option<Menu> = None;
    match command {
        ElectronCommand::BrowserWindowCreate { params } => {
            trace!("BrowserWindowCreate {}", params.id);
            frontend.create_window(params.id.clone(), event_loop, proxy, params);
            respond_ok(responder);
        },
        ElectronCommand::BrowserWindowLoadfile { params } => {
            trace!("BrowserWindowLoadFile {} {}", params.id, params.file);
            frontend.load_url(&params.id, params.file);
            backend.command_callback("BrowserWindowLoadfile".to_string(), params.id);
            respond_ok(responder);
        },
        ElectronCommand::BrowserWindowShow { id , shown} => {
            frontend.show(&id, shown);
            respond_ok(responder);
        },
        ElectronCommand::BrowserWindowBounds { id, params} => {
            match params {
                BrowserWindowBoundsAction::Get => {
                    let bounds = frontend.content_bounds(&id);
                    if let Some(bounds) = bounds {
                        match serde_json::to_string(&bounds) {
                            Ok(json) => {
                                respond_status(StatusCode::OK, CONTENT_TYPE_JSON.to_string(), json.into_bytes(), responder);
                            },
                            Err(e) => {
                                respond_status(StatusCode::INTERNAL_SERVER_ERROR, CONTENT_TYPE_TEXT.to_string(), format!("BrowserWindowBoundsAction::Get json serialization error: {}", e).into_bytes(), responder);
                            }
                        }
                    } else {
                        respond_404(responder);
                    }
                },
                BrowserWindowBoundsAction::Set {bounds} => {
                    frontend.set_content_bounds(&id, bounds);
                    respond_ok(responder);
                }
            }
        },
        ElectronCommand::BrowserWindowMaximized { id, params} => {
            match params {
                BrowserWindowMaximizedAction::Get => {
                    let maximized = frontend.is_maximized(&id);
                    respond_status(StatusCode::OK, CONTENT_TYPE_JSON.to_string(), maximized.to_string().into_bytes(), responder);       
                },
                BrowserWindowMaximizedAction::Set {maximized} => {
                    frontend.set_maximized(&id, maximized);
                    respond_ok(responder);
                }
            }
        },
        ElectronCommand::BrowserWindowMinimized { id, params} => {
            match params {
                BrowserWindowMinimizedAction::Get => {
                    let minimized = frontend.is_minimized(&id);
                    respond_status(StatusCode::OK, CONTENT_TYPE_JSON.to_string(), minimized.to_string().into_bytes(), responder);       
                },
                BrowserWindowMinimizedAction::Set {minimized} => {
                    frontend.set_minimized(&id, minimized);
                    respond_ok(responder);
                }
            }
        },
        ElectronCommand::BrowserWindowClose { id} => {
            frontend.close(event_loop, &id);
            if frontend.count()==0 {
                backend.window_all_closed();
            }
            respond_ok(responder);
        },
        ElectronCommand::ChannelSendMessage {id, channel, args} => {
            frontend.send_channel_message(proxy, id, channel, args);
            respond_ok(responder);
        },
        ElectronCommand::ExecuteJavascript {id, script} => {
            frontend.execute_javascript(&id, &script);
            respond_ok(responder);
        },
        ElectronCommand::BrowserWindowDevTools { params } => {
            trace!("BrowserWindowDevTools {}", params.id);
            match params.call {
                BrowserWindowDevToolsCall::Open => {
                    frontend.open_devtools(&params.id);
                },
                BrowserWindowDevToolsCall::Close => {
                    frontend.close_devtools(&params.id);
                }
            }
            respond_ok(responder);
        },
        ElectronCommand::AppQuit {exit} => {
            respond_ok(responder);
            tokio_runtime.spawn(
                async move {
                    sleep(Duration::from_millis(100)).await;
                    let _ = proxy.send_event(ElectricoEvents::Exit);
                }
            );
        },
        ElectronCommand::AppSetName { name } => {
            app_env.app_name = Some(name);
            respond_ok(responder);
        },
        ElectronCommand::SetApplicationMenu { menu } => {
            if let Some(menu) = menu {
                menu_ret = Some(create_menu(frontend.get_actual_window(), menu, &app_env.app_name));
            }
            respond_ok(responder);
        },
        ElectronCommand::GetAppPath { path } => {
            if let Some(path) = path {
                if path=="appData" {
                    if let Some(proj_dirs) = directories::ProjectDirs::from("", "", package.name.as_str()) {
                        let dir = proj_dirs.config_dir().as_os_str().to_str().unwrap().to_string();
                        check_and_create_dir(Path::new(&dir));
                        respond_status(StatusCode::OK, CONTENT_TYPE_TEXT.to_string(), dir.into_bytes(), responder);
                    } else {
                        respond_404(responder);
                    }
                } else if path=="logs" {
                    if let Some(proj_dirs) = directories::ProjectDirs::from("", "", package.name.as_str()) {
                        let dir = proj_dirs.cache_dir().as_os_str().to_str().unwrap().to_string();
                        check_and_create_dir(Path::new(&dir));
                        respond_status(StatusCode::OK, CONTENT_TYPE_TEXT.to_string(), dir.into_bytes(), responder);
                    } else {
                        respond_404(responder);
                    }
                } else if path=="temp" {
                    if let Some(proj_dirs) = directories::ProjectDirs::from("", "", package.name.as_str()) {
                        let dir = proj_dirs.cache_dir().as_os_str().to_str().unwrap().to_string();
                        check_and_create_dir(Path::new(&dir));
                        respond_status(StatusCode::OK, CONTENT_TYPE_TEXT.to_string(), dir.into_bytes(), responder);
                    } else {
                        respond_404(responder);
                    }
                } else if path=="userData" {
                    if let Some(proj_dirs) = directories::ProjectDirs::from("", "", package.name.as_str()) {
                        let dir = proj_dirs.config_dir().as_os_str().to_str().unwrap().to_string();
                        check_and_create_dir(Path::new(&dir));
                        respond_status(StatusCode::OK, CONTENT_TYPE_TEXT.to_string(), dir.into_bytes(), responder);
                    } else {
                        respond_404(responder);
                    }
                } else if path=="userHome" {
                    if let Some(user_dirs) = directories::UserDirs::new() {
                        let dir = user_dirs.home_dir().as_os_str().to_str().unwrap().to_string();
                        respond_status(StatusCode::OK, CONTENT_TYPE_TEXT.to_string(), dir.into_bytes(), responder);
                    } else {
                        respond_404(responder);
                    }
                }
            } else {
                respond_status(StatusCode::OK, CONTENT_TYPE_TEXT.to_string(), rsrc_dir.as_os_str().to_str().unwrap().to_string().into_bytes(), responder);
            }
        },
        ElectronCommand::GetAppVersion => {
            respond_status(StatusCode::OK, CONTENT_TYPE_TEXT.to_string(), package.version.clone().into_bytes(), responder);
        },
        ElectronCommand::ShowOpenDialogSync { options } => {
            let mut picked:Option<Vec<String>>=None;
            if let Some(properties) = options.properties {
                let mut fd = rfd::FileDialog::new();
                if let Some(filters) = options.filters {
                    for filter in filters {
                        let ext:Vec<&str> = filter.extensions.iter().map(|e| &**e).collect();
                        fd = fd.add_filter(filter.name, &ext);
                    }
                }
                if let Some(title) = options.title {
                    fd = fd.set_title(title);
                }
                if let Some(default_path) = options.default_path {
                    fd = fd.set_directory(default_path);
                }
                if properties.contains(&"createDirectory".to_string()) {
                    fd = fd.set_can_create_directories(true);
                }
                if properties.contains(&"openFile".to_string()) {
                    if properties.contains(&"multiSelections".to_string()) {
                        match fd.pick_files() {
                            Some(sel) => {
                                picked = Some(sel.into_iter().map(|p| p.as_os_str().to_str().unwrap().to_string()).collect());
                            },
                            None => {}
                        }
                    } else {
                        match fd.pick_file() {
                            Some(sel) => {
                                let mut p:Vec<String> = Vec::new();
                                p.push(sel.as_os_str().to_str().unwrap().to_string());
                                picked = Some(p);
                            },
                            None => {}
                        }
                    }
                } else if properties.contains(&"openDirectory".to_string()) {
                    if properties.contains(&"multiSelections".to_string()) {
                        match fd.pick_folders() {
                            Some(sel) => {
                                picked = Some(sel.into_iter().map(|p| p.as_os_str().to_str().unwrap().to_string()).collect());
                            },
                            None => {}
                        }
                    } else {
                        match fd.pick_folder() {
                            Some(sel) => {
                                let mut p:Vec<String> = Vec::new();
                                p.push(sel.as_os_str().to_str().unwrap().to_string());
                                picked = Some(p);
                            },
                            None => {}
                        }
                    }
                } 
            }
            match serde_json::to_string(&picked) {
                Ok(json) => {
                    respond_status(StatusCode::OK, CONTENT_TYPE_JSON.to_string(), json.into_bytes(), responder);
                },
                Err(e) => {
                    respond_status(StatusCode::INTERNAL_SERVER_ERROR, CONTENT_TYPE_TEXT.to_string(), format!("ShowOpenDialog json serialization error: {}", e).into_bytes(), responder);
                }
            }
        },
        ElectronCommand::ShowOpenDialog { window_id, options } => {
            if let Some(window_id) = window_id {
                frontend.set_focus(&window_id);
            }
            tokio_runtime.spawn(
                async move {
                    let mut picked:Option<Vec<String>>=None;
                    if let Some(properties) = options.properties {
                        let mut fd = rfd::AsyncFileDialog::new();
                        if let Some(filters) = options.filters {
                            for filter in filters {
                                let ext:Vec<&str> = filter.extensions.iter().map(|e| &**e).collect();
                                fd = fd.add_filter(filter.name, &ext);
                            }
                        }
                        if let Some(title) = options.title {
                            fd = fd.set_title(title);
                        }
                        if let Some(default_path) = options.default_path {
                            fd = fd.set_directory(default_path);
                        }
                        if properties.contains(&"createDirectory".to_string()) {
                            fd = fd.set_can_create_directories(true);
                        }
                        if properties.contains(&"openFile".to_string()) {
                            if properties.contains(&"multiSelections".to_string()) {
                                match fd.pick_files().await {
                                    Some(sel) => {
                                        picked = Some(sel.into_iter().map(|p| p.path().as_os_str().to_str().unwrap().to_string()).collect());
                                    },
                                    None => {}
                                }
                            } else {
                                match fd.pick_file().await {
                                    Some(sel) => {
                                        let mut p:Vec<String> = Vec::new();
                                        p.push(sel.path().as_os_str().to_str().unwrap().to_string());
                                        picked = Some(p);
                                    },
                                    None => {}
                                }
                            }
                        } else if properties.contains(&"openDirectory".to_string()) {
                            if properties.contains(&"multiSelections".to_string()) {
                                match fd.pick_folders().await {
                                    Some(sel) => {
                                        picked = Some(sel.into_iter().map(|p| p.path().as_os_str().to_str().unwrap().to_string()).collect());
                                    },
                                    None => {}
                                }
                            } else {
                                match fd.pick_folder().await {
                                    Some(sel) => {
                                        let mut p:Vec<String> = Vec::new();
                                        p.push(sel.path().as_os_str().to_str().unwrap().to_string());
                                        picked = Some(p);
                                    },
                                    None => {}
                                }
                            }
                        } 
                    }
                    match serde_json::to_string(&picked) {
                        Ok(json) => {
                            respond_status(StatusCode::OK, CONTENT_TYPE_JSON.to_string(), json.into_bytes(), responder);
                        },
                        Err(e) => {
                            respond_status(StatusCode::INTERNAL_SERVER_ERROR, CONTENT_TYPE_TEXT.to_string(), format!("ShowOpenDialog json serialization error: {}", e).into_bytes(), responder);
                        }
                    }
                }
            );
        },
        ElectronCommand::ShowSaveDialogSync { options } => {
            let mut picked:Option<String>=None;
            if let Some(properties) = options.properties {
                let mut fd = rfd::FileDialog::new();
                if let Some(filters) = options.filters {
                    for filter in filters {
                        let ext:Vec<&str> = filter.extensions.iter().map(|e| &**e).collect();
                        fd = fd.add_filter(filter.name, &ext);
                    }
                }
                if let Some(title) = options.title {
                    fd = fd.set_title(title);
                }
                if let Some(default_path) = options.default_path {
                    fd = fd.set_directory(default_path);
                }
                if properties.contains(&"createDirectory".to_string()) {
                    fd = fd.set_can_create_directories(true);
                }
                match fd.save_file() {
                    Some(sel) => {
                        picked = Some(sel.as_os_str().to_str().unwrap().to_string());
                    },
                    None => {}
                }
            }
            match serde_json::to_string(&picked) {
                Ok(json) => {
                    respond_status(StatusCode::OK, CONTENT_TYPE_JSON.to_string(), json.into_bytes(), responder);
                },
                Err(e) => {
                    respond_status(StatusCode::INTERNAL_SERVER_ERROR, CONTENT_TYPE_TEXT.to_string(), format!("ShowOpenDialog json serialization error: {}", e).into_bytes(), responder);
                }
            }
        },
        ElectronCommand::ShowSaveDialog { window_id, options } => {
            if let Some(window_id) = window_id {
                frontend.set_focus(&window_id);
            }
            tokio_runtime.spawn(
                async move {
                    let mut fd = rfd::AsyncFileDialog::new();
                    if let Some(filters) = options.filters {
                        for filter in filters {
                            let ext:Vec<&str> = filter.extensions.iter().map(|e| &**e).collect();
                            fd = fd.add_filter(filter.name, &ext);
                        }
                    }
                    if let Some(title) = options.title {
                        fd = fd.set_title(title);
                    }
                    if let Some(default_path) = options.default_path {
                        fd = fd.set_directory(default_path);
                    }
                    if let Some(properties) = options.properties {
                    if properties.contains(&"createDirectory".to_string()) {
                            fd = fd.set_can_create_directories(true);
                        }
                    }
                    match fd.save_file().await {
                        Some(sel) => {
                            let picked = sel.path().as_os_str().to_str().unwrap().to_string();
                            respond_status(StatusCode::OK, CONTENT_TYPE_JSON.to_string(), picked.into_bytes(), responder);
                        },
                        None => {
                            respond_status(StatusCode::OK, CONTENT_TYPE_JSON.to_string(), "".to_string().into_bytes(), responder);
                        }
                    }
                }
            );
        },
        ElectronCommand::ShowMessageBoxSync { options } => {
            let mut dialog = rfd::MessageDialog::new();
            let mut desc = options.message;
            if let Some(detail) = options.detail {
                desc = desc + " - " + detail.as_str();
            }
            dialog = dialog.set_description(desc);
            if let Some(title) = options.title {
                dialog = dialog.set_title(title.as_str());
            }
            if let Some(msg_type) = options.msg_type {
               if msg_type=="error" {
                   dialog = dialog.set_level(MessageLevel::Error);
               } else if msg_type=="info" {
                   dialog = dialog.set_level(MessageLevel::Info);
               }
            }
            respond_ok(responder);
            dialog.show();
            
        },
        ElectronCommand::GetPrimaryDisplay => {
            respond_ok(responder);
        },
        ElectronCommand::ShellOpenExternal { url } => {
            let _  = open::that(url);
            respond_ok(responder);
        },
        ElectronCommand::PrintToPDF { id } => {
            // TODO - silent printing
            frontend.print(&id);
            tokio_runtime.spawn(
                async move {
                    sleep(Duration::from_secs(10)).await;
                    respond_ok(responder);
                }
            );
        }
    }
    menu_ret
}
```

</pre></span></p>


-----------------------

/src/electron/menu.rs:
-----------------------

<p align="left"><span><pre>

```rust
use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;
use log::debug;
use muda::{accelerator::{Accelerator, Code, Modifiers}, Menu, MenuItem, PredefinedMenuItem, Submenu};
use substring::Substring;
use tao::window::Window;

use crate::electron::types::AppMenu;

lazy_static! {
    static ref MODIFIERS: Mutex<HashMap<&'static str, Modifiers>> = {
        let mut m:HashMap<&str, Modifiers> = HashMap::new();
        m.insert("Cmd", Modifiers::SUPER);
        m.insert("Ctrl", Modifiers::CONTROL);
        m.insert("Alt", Modifiers::ALT);
        m.insert("Shift", Modifiers::SHIFT);
        #[cfg(target_os = "macos")]
        m.insert("CmdOrCtrl", Modifiers::SUPER);
        #[cfg(target_os = "windows")]
        m.insert("CmdOrCtrl", Modifiers::CONTROL);
        Mutex::new(m)
    };
    static ref KEYS: Mutex<HashMap<&'static str, Code>> = {
        let mut k:HashMap<&str, Code> = HashMap::new();
        k.insert("A", Code::KeyA);
        k.insert("B", Code::KeyB);
        k.insert("C", Code::KeyC);
        k.insert("D", Code::KeyD);
        k.insert("E", Code::KeyE);
        k.insert("F", Code::KeyF);
        k.insert("G", Code::KeyG);
        k.insert("H", Code::KeyH);
        k.insert("I", Code::KeyI);
        k.insert("J", Code::KeyJ);
        k.insert("K", Code::KeyK);
        k.insert("L", Code::KeyL);
        k.insert("M", Code::KeyM);
        k.insert("N", Code::KeyN);
        k.insert("O", Code::KeyO);
        k.insert("P", Code::KeyP);
        k.insert("Q", Code::KeyQ);
        k.insert("R", Code::KeyR);
        k.insert("S", Code::KeyS);
        k.insert("T", Code::KeyT);
        k.insert("U", Code::KeyU);
        k.insert("V", Code::KeyV);
        k.insert("W", Code::KeyW);
        k.insert("X", Code::KeyX);
        k.insert("Y", Code::KeyY);
        k.insert("Z", Code::KeyZ);
        k.insert("-", Code::Minus);
        k.insert("+", Code::NumpadAdd);
        k.insert("=", Code::Equal);
        Mutex::new(k)
    };    
}

fn populate_menu(sub_menu:&Submenu, menu:&Vec<super::types::Menu>, app_name:&Option<String>, keys_map:&std::sync::MutexGuard<'_, HashMap<&str, Code>>, mods_map:&std::sync::MutexGuard<'_, HashMap<&str, Modifiers>>) {
    for item in menu.iter() {
        let mut accelerator:Option<Accelerator> = None;
        if let Some(acc) = item.accelerator.clone() {
            let keystr:&str;
            let mut acc_modifier:Option<Modifiers> = None;
            if let Some(pos) = acc.rfind("+") {
                keystr = acc.substring(pos+1, acc.len());
                let modstr = acc.substring(0, pos);
                let mut modifier = Modifiers::empty();
                for modpart in modstr.split("+") {
                    if let Some(modif) = mods_map.get(modpart) {
                        modifier = modifier | *modif;
                    }
                }
                acc_modifier = Some(modifier);
            } else {
                keystr = acc.as_str();
            }
            if let Some(key) = keys_map.get(keystr) {
                accelerator = Some(Accelerator::new(acc_modifier, *key));
            }
        }
        if let Some(label) = item.label.clone() {
            if let Some(submenu) = &item.submenu {
                let sub_sub_menu = Submenu::new(label, true);
                populate_menu(&sub_sub_menu, submenu, app_name, keys_map, mods_map);
                let _ = sub_menu.append(&sub_sub_menu);
            } else {
                let _ = sub_menu.append(
                    &MenuItem::with_id(item.id.as_str(), label, true, accelerator)
                );
            }
        } else if let Some(item_type) = item.item_type.clone() {
            if item_type=="separator" {
            let _ = sub_menu.append(&PredefinedMenuItem::separator());
            }
        } else if let Some(role) = item.role.clone() {
            if role=="quit" {
                let mut label = "Quit".to_string();
                if let Some(app_name) = app_name {
                    label = label + " " + app_name.as_str();
                }
                let _ = sub_menu.append(
                    &MenuItem::with_id("quit", label.as_str(), true, accelerator)
                );
            } else if role=="cut" {
                let _ = sub_menu.append(&PredefinedMenuItem::cut(None));
            } else if role=="copy" {
                let _ = sub_menu.append(&PredefinedMenuItem::copy(None));
            } else if role=="paste" {
                let _ = sub_menu.append(&PredefinedMenuItem::paste(None));
            } else if role=="undo" {
                let _ = sub_menu.append(&PredefinedMenuItem::undo(None));
            } else if role=="redo" {
                let _ = sub_menu.append(&PredefinedMenuItem::redo(None));
            } else if role=="hide" {
                let _ = sub_menu.append(&PredefinedMenuItem::hide(None));
            } else if role=="hideothers" {
                let _ = sub_menu.append(&PredefinedMenuItem::hide_others(None));
            } else if role=="unhide" {
                let _ = sub_menu.append(&PredefinedMenuItem::bring_all_to_front(None));
            } else if role=="close" {
                let _ = sub_menu.append(&PredefinedMenuItem::close_window(None));
            } else if role=="toggleDevTools" {
                let mut label: String = "Toggle Developer Tools".to_string();
                if let Some(lab) = item.label.clone() {
                    label = lab;
                }
                let _ = sub_menu.append(&MenuItem::with_id("toggleDevTools", label, true, accelerator));
            }
        }
    }
}

pub fn create_menu(window:Option<&Window>, menu:Vec<AppMenu>, app_name:&Option<String>) -> Menu {
    let keys_map: std::sync::MutexGuard<'_, HashMap<&str, Code>> = KEYS.lock().unwrap();
    let mods_map: std::sync::MutexGuard<'_, HashMap<&str, Modifiers>> = MODIFIERS.lock().unwrap();
    let main_menu = Menu::new();
    for app_menu in menu.iter() {
        let mut label = "".to_string();
        if let Some(mlab) = app_menu.label.clone() {
            label = mlab;
        }
        let sub_menu = Submenu::new(label, true);
        populate_menu(&sub_menu, &app_menu.submenu, app_name, &keys_map, &mods_map);
        let _ = main_menu.append(&sub_menu);
    }
    if let Some(window) = window {
        #[cfg(target_os = "windows")] {
            use tao::platform::windows::WindowExtWindows;
            unsafe {main_menu.init_for_hwnd(window.hwnd() as _).unwrap();}
        }
        #[cfg(target_os = "linux")] {
            use tao::platform::unix::WindowExtUnix;
            let _ = main_menu.init_for_gtk_window(window.gtk_window(), window.default_vbox());
        }
    }
    #[cfg(target_os = "macos")]
    main_menu.init_for_nsapp();
    main_menu
}
```

</pre></span></p>


-----------------------

/src/electron/mod.rs:
-----------------------

pub mod types;
pub mod electron;
pub mod menu;

-----------------------

/src/electron/types.rs:
-----------------------

<p align="left"><span><pre>

```rust
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BrowserWindowLoadFileParamConfigWebPreferences {
  pub preload: Option<String>,
  #[serde(rename = "nodeIntegration")]
  pub node_integration: bool,
  #[serde(rename = "contextIsolation")]
  pub context_isolation: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct BrowserWindowLoadFileParamConfig {
  pub title: String,
  pub x: Option<i32>,
  pub y: Option<i32>,
  pub width: Option<i32>,
  pub height: Option<i32>,
  pub resizable: bool,
  pub modal: bool,
  pub show: bool,
  pub icon: Option<String>,
  #[serde(rename = "webPreferences")]
  pub web_preferences: BrowserWindowLoadFileParamConfigWebPreferences
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct BrowserWindowCreateParam {
  pub id: String,
  pub config: BrowserWindowLoadFileParamConfig
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct BrowserWindowLoadFileParam {
  pub id: String,
  pub file: String,
  pub config: BrowserWindowLoadFileParamConfig
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum BrowserWindowDevToolsCall {
  Open,
  Close
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct BrowserWindowDevToolsParam {
  pub id: String,
  pub call: BrowserWindowDevToolsCall
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Menu {
  pub id: String,
  pub label: Option<String>,
  pub accelerator: Option<String>,
  pub role: Option<String>,
  #[serde(rename = "type")]
  pub item_type: Option<String>,
  pub submenu: Option<Vec<Menu>>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AppMenu {
  pub id: String,
  pub label: Option<String>,
  pub submenu: Vec<Menu>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FileFilter {
  pub name: String,
  pub extensions: Vec<String>,
}


#[derive(serde::Serialize, serde::Deserialize)]
pub struct FileDialogOptions {
  pub title: Option<String>,
  #[serde(rename = "defaultPath")]
  pub default_path: Option<String>,
  #[serde(rename = "buttonLabel")]
  pub button_label: Option<String>,
  pub filters:Option<Vec<FileFilter>>,
  pub properties: Option<Vec<String>>,
  pub message: Option<String>,
  
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ShowMessageBoxOptions {
  pub message: String,
  #[serde(rename = "type")]
  pub msg_type: Option<String>,
  #[serde(rename = "buttonLabel")]
  pub title: Option<String>,
  pub detail: Option<String>,
}
impl ShowMessageBoxOptions {
  pub fn new(message: String) -> ShowMessageBoxOptions {
    ShowMessageBoxOptions {message, msg_type:None, title:None, detail:None }
  }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Rectangle {
  pub x: i32,
  pub y: i32,
  pub width: u32,
  pub height: u32,
}
impl Rectangle {
  pub fn new(x: i32, y: i32, width: u32, height: u32) -> Rectangle {
    Rectangle { x, y, width, height }
  }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Display {
  pub bounds: Rectangle
}
impl Display {
  pub fn new(bounds: Rectangle) -> Display {
    Display { bounds }
  }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "method")]
pub enum BrowserWindowBoundsAction {
  Set {bounds: Rectangle},
  Get
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "method")]
pub enum BrowserWindowMaximizedAction {
  Set {maximized: bool},
  Get
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "method")]
pub enum BrowserWindowMinimizedAction {
  Set {minimized: bool},
  Get
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "command")]
pub enum ElectronCommand {
    BrowserWindowCreate {params: BrowserWindowCreateParam},
    BrowserWindowLoadfile {params: BrowserWindowLoadFileParam},
    BrowserWindowShow {id: String, shown:bool},
    BrowserWindowClose {id: String},
    BrowserWindowBounds {id: String, params:BrowserWindowBoundsAction},
    BrowserWindowMaximized {id: String, params:BrowserWindowMaximizedAction},
    BrowserWindowMinimized {id: String, params:BrowserWindowMinimizedAction},
    BrowserWindowDevTools {params: BrowserWindowDevToolsParam},
    ChannelSendMessage {id: String, channel: String, args: String},
    ExecuteJavascript {id: String, script:String},
    AppQuit {exit: bool},
    AppSetName {name: String},
    GetAppPath {path: Option<String>},
    GetAppVersion,
    SetApplicationMenu {menu:Option<Vec<AppMenu>>},
    ShowOpenDialogSync {options:FileDialogOptions},
    ShowOpenDialog {window_id:Option<String>, options:FileDialogOptions},
    ShowSaveDialogSync {options:FileDialogOptions},
    ShowSaveDialog {window_id:Option<String>, options:FileDialogOptions},
    ShowMessageBoxSync {options:ShowMessageBoxOptions},
    GetPrimaryDisplay,
    ShellOpenExternal {url:String},
    PrintToPDF {id: String}
}
```

</pre></span></p>


-----------------------

/src/frontend.rs:
-----------------------

<p align="left"><span><pre>

```rust
use log::{debug, error, trace, warn};
use include_dir::{include_dir, Dir};
use reqwest::StatusCode;
use substring::Substring;
use std::{collections::HashMap, fs, path::PathBuf};
use tao::{dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize}, event_loop::{EventLoopProxy, EventLoopWindowTarget}, window::{Icon, Window, WindowBuilder, WindowId}};
use serde_json::Error;
use wry::{http::Request, RequestAsyncResponder, WebView, WebViewBuilder};
use crate::{common::{append_js_scripts, escape, is_module_request, respond_status, CONTENT_TYPE_TEXT, JS_DIR_FRONTEND}, electron::types::{BrowserWindowCreateParam, Rectangle, ShowMessageBoxOptions}, types::{Command, ElectricoEvents, FrontendCommand}};

pub struct FrontendWindow {
    window:Window,
    webview:WebView,
    id:WindowId,
    client_path_base:Option<PathBuf>
}
impl FrontendWindow {
    pub fn new(window:Window, webview:WebView, id:WindowId) -> FrontendWindow {
        FrontendWindow {
            window:window,
            webview:webview,
            id:id,
            client_path_base:None
        }
    }
    pub fn set_client_path_base(&mut self, client_path_base:Option<PathBuf>) {
        self.client_path_base = client_path_base;
    }
}

pub struct Frontend {
    window_ids:HashMap<WindowId, String>,
    windows:HashMap<String, FrontendWindow>,
    opened_windows:usize,
    frontendalljs:String,
    rsrc_dir:PathBuf
}
impl Frontend {
    pub fn new(rsrc_dir:PathBuf) -> Frontend {
        let mut frontendalljs:String = String::new();
        const JS_DIR_SHARED: Dir = include_dir!("src/js/shared");
        frontendalljs = append_js_scripts(frontendalljs, JS_DIR_SHARED, Some(".js"));
        frontendalljs = append_js_scripts(frontendalljs, JS_DIR_FRONTEND, Some("electrico.js"));
        Frontend {
            window_ids:HashMap::new(),
            windows:HashMap::new(),
            opened_windows:0,
            frontendalljs:frontendalljs,
            rsrc_dir:rsrc_dir
        }
    }
    pub fn create_window(&mut self, id:String, event_loop:&EventLoopWindowTarget<ElectricoEvents>, proxy:EventLoopProxy<ElectricoEvents>, config_params:BrowserWindowCreateParam) {
        let ipc_proxy = proxy.clone();
        let ipc_handler_id = id.clone();
        let ipc_handler = move |request: Request<Vec<u8>>, responder:RequestAsyncResponder| {
            trace!("frontend ipc request {}", request.uri().path().to_string());
            let msgr =  String::from_utf8(request.body().to_vec());
            match msgr {
                Ok(msg) => {
                  trace!("frontend ipc request body {}", msg.as_str());
                  let commandr:Result<FrontendCommand, Error> = serde_json::from_str(msg.as_str());
                  match commandr {
                    Ok (command) => {
                      match command {
                        FrontendCommand::PostIPC {request_id, nonce, params } => {
                            trace!("frontend ipc call {} {}", nonce, params);
                            if nonce!=ipc_handler_id {
                                error!("frontend ipc call nonce does not match - forbidden client-nonce:{} backend-nonce:{}", nonce, ipc_handler_id.clone());
                                respond_status(StatusCode::FORBIDDEN, CONTENT_TYPE_TEXT.to_string(), "forbidden".to_string().into_bytes(), responder);
                                return;
                            }
                            let _ = ipc_proxy.send_event(ElectricoEvents::ExecuteCommand{command:Command::PostIPC {browser_window_id:ipc_handler_id.clone(), request_id, params}, responder, data_blob:None});
                        },
                        FrontendCommand::GetProcessInfo {nonce} => {
                            if nonce!=ipc_handler_id {
                                error!("frontend GetProcessInfo call nonce does not match - forbidden client-nonce:{} backend-nonce:{}", nonce, ipc_handler_id.clone());
                                respond_status(StatusCode::FORBIDDEN, CONTENT_TYPE_TEXT.to_string(), "forbidden".to_string().into_bytes(), responder);
                                return;
                            }
                            let _ = ipc_proxy.send_event(ElectricoEvents::ExecuteCommand{command:Command::Node { invoke: crate::node::types::NodeCommand::GetProcessInfo }, responder, data_blob:None});
                        },
                        FrontendCommand::DOMContentLoaded {title } => {
                            let _ = ipc_proxy.send_event(ElectricoEvents::ExecuteCommand{command:Command::DOMContentLoaded {browser_window_id:ipc_handler_id.clone(), title}, responder, data_blob:None});
                        },
                        FrontendCommand::Alert {message } => {
                            let _ = ipc_proxy.send_event(ElectricoEvents::ExecuteCommand{command:Command::Electron { invoke: crate::electron::types::ElectronCommand::ShowMessageBoxSync { options: ShowMessageBoxOptions::new(message) } }, responder, data_blob:None});
                        }
                      }
                    }
                    Err(e) => {
                      error!("json serialize error {}", e.to_string());
                    }
                  }
                },
                Err(e) => {
                  error!("utf8 error {}", e.to_string());
                }
            }
        };
        
        let mut preload:String = "".to_string();
        if let Some(preloadstr) = config_params.config.web_preferences.preload {
             preload=preloadstr;
        }
        
        let window = WindowBuilder::new()
            .build(&event_loop)
            .unwrap();
   
       
        window.set_title(&config_params.config.title);
        if !&config_params.config.show {
            window.set_visible(false);
        }
        window.set_resizable(config_params.config.resizable);
        
        if let Some(x) = config_params.config.x {
            if let Some(y) = config_params.config.y {
                let lpos = LogicalPosition::new(x, y);
                let pos:PhysicalPosition<i32> = lpos.to_physical(window.current_monitor().unwrap().scale_factor());
                window.set_outer_position(pos);
            }
        }
        if let Some(width) = config_params.config.width {
            if let Some(height) = config_params.config.height {
                window.set_inner_size(LogicalSize::new(width, height));
            }
        }

        if let Some(icon_path) = config_params.config.icon {
            match fs::read(&icon_path) {
                Ok(ifile) => {
                    match Icon::from_rgba(ifile, 32, 32) {
                        Ok(icon) => {
                            #[cfg(any(
                                target_os = "windows",
                                target_os = "macos",
                                target_os = "ios",
                                target_os = "android"
                            ))]
                            window.set_window_icon(Some(icon));
                        },
                        Err(e) => {
                            warn!("icon could not be built: {}", e);
                        }
                    }
                },
                Err(e) => {
                    error!("icon file not found: {}, {}", icon_path, e);
                }
            }
        }

        let preload_init = preload.clone();
        let nav_handler_id = id.clone();
        let nav_proxy = proxy.clone();
        let nav_handler = move |page: String| {
            trace!("nav_handler {}", page);
            let _ = nav_proxy.send_event(ElectricoEvents::FrontendNavigate {browser_window_id:nav_handler_id.clone(), page, preload:preload.clone()});
            true
        };
        let preload_file = self.rsrc_dir.clone().join(preload_init.clone());
        let mut preload_script = "".to_string();
        if preload_init.len()>0 {
            match std::fs::read_to_string(preload_file) {
                Ok(preloadfilejs) => {
                    preload_script = preloadfilejs;
                },
                Err(_e) => {
                    error!("cant load preload file {}", preload_init);
                }
            }
        }

        let fil_handler_id = id.clone();
        let fil_handler = move |request: Request<Vec<u8>>, responder:RequestAsyncResponder| {
            let mut fpath = request.uri().path().to_string();
            if fpath.starts_with("/") {
                fpath = fpath.substring(1, fpath.len()).to_string();
            }
            fpath = fpath.replace("..", "");
            trace!("frontend fil: fpath {}", fpath);
            let _ = proxy.send_event(ElectricoEvents::ExecuteCommand {command:Command::BrowserWindowReadFile {
                browser_window_id:fil_handler_id.clone(), 
                file_path: fpath, 
                module:is_module_request(request.uri().host())
            }, responder, data_blob:None});
        };

        let mut is_windows="false";
        #[cfg(target_os = "windows")] {
            is_windows = "true";
        }

        #[cfg(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        ))]
        let builder = WebViewBuilder::new(&window);
    
        #[cfg(not(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        )))]
        let builder = {
            use tao::platform::unix::WindowExtUnix;
            use wry::WebViewBuilderExtUnix;
            let vbox = window.default_vbox().unwrap();
            WebViewBuilder::new_gtk(vbox)
        };

        let webview = builder
            .with_asynchronous_custom_protocol("fil".into(), fil_handler)
            .with_asynchronous_custom_protocol("ipc".into(), ipc_handler)
            .with_initialization_script(("window.__is_windows=".to_string()+is_windows+";var __electrico_nonce='"+config_params.id.clone().as_str()+"'; window.__electrico_preload=function(document, ph){\nph.before(__electrico_nonce); var window=document.window; var require=document.window.require;\n"+preload_script.as_str()+"\nph.after();\n};\n"+self.frontendalljs.as_str()+"\n__electrico_nonce='';\n").as_str())
            .with_navigation_handler(nav_handler)
            .with_devtools(true)
            .build().unwrap();

        #[cfg(debug_assertions)]
        webview.open_devtools();

        let w_id = window.id().clone();
        self.window_ids.insert(window.id().clone(), config_params.id.clone());
        let fwindow: FrontendWindow = FrontendWindow::new(window, webview, w_id);
        self.windows.insert(config_params.id.clone(), fwindow);
        self.opened_windows+=1;
    }
    pub fn load_url(&mut self, id:&String, fpath:String) {
        if let Some(window) = self.windows.get_mut(id) {
            if fpath.starts_with("http://") || fpath.starts_with("https://") {
                #[cfg(debug_assertions)]
                window.set_client_path_base(Some(self.rsrc_dir.clone()));
                #[cfg(not(debug_assertions))]
                window.set_client_path_base(None);
                
                let _ = window.webview.load_url(fpath.as_str());
            } else {
                let mut url="fil://file/";
                #[cfg(target_os = "windows")] {
                    url = "http://fil.file/";
                }
                window.set_client_path_base(Some(self.rsrc_dir.clone()));
                let _ = window.webview.load_url((url.to_string() + fpath.as_str()).as_str());
            }
            
        } else {
            error!("load_url - frontend_webview not there - id: {}", id);
        }
    }
    pub fn show(&mut self, id:&String, shown: bool) {
        if let Some(window) = self.windows.get(id) {
            window.window.set_visible(shown);
        } else {
            error!("show - frontend_webview not there - id: {}", id);
        }
    }
    pub fn send_channel_message(&mut self, proxy: EventLoopProxy<ElectricoEvents>, id:String, channel:String, args:String) {
        if let Some(window) = self.windows.get(&id) {
            let _ = window.webview.evaluate_script_with_callback(format!("window.__electrico.sendChannelMessage('{}@{}');", &channel, escape(&args)).as_str(), move |r| {
                if r.len()==0 {
                    trace!("send_channel_message not OK - resending");
                    let _ = proxy.send_event(ElectricoEvents::SendChannelMessageRetry { browser_window_id: id.clone(), channel:channel.clone(), args:args.clone()});
                }
            });
        } else {
            error!("send_channel_message - frontend_webview not there - id: {}", id);
        }
    }
    pub fn execute_javascript(&mut self, id:&String, script:&String) {
        if let Some(window) = self.windows.get(id) {
            let _ = window.webview.evaluate_script(script);
        } else {
            error!("execute_javascript - frontend_webview not there - id: {}", id);
        }
    }
    pub fn open_devtools(&mut self, id:&String) {
        if let Some(window) = self.windows.get(id) {
            #[cfg(debug_assertions)]
            window.webview.open_devtools();
        } else {
            error!("open_devtools - frontend_webview not there - id: {}", id);
        }
    }
    pub fn close_devtools(&mut self, id:&String) {
        if let Some(window) = self.windows.get(id) {
            #[cfg(debug_assertions)]
            window.webview.close_devtools();
        } else {
            error!("close_devtools - frontend_webview not there - id: {}", id);
        }
    }
    pub fn get_id(&mut self, id:&WindowId) -> Option<&String> {
        self.window_ids.get(id)
    }
    pub fn get_client_path_base(&mut self, id:&String) -> &Option<PathBuf> {
        if let Some(window) = self.windows.get(id) {
            &window.client_path_base
        } else {
            error!("get_client_path_base - frontend_webview not there - id: {}", id);
            &None
        }
    }
    pub fn content_bounds(&mut self, id:&String) -> Option<Rectangle> {
        if let Some(win) = self.windows.get(id) {
            if let Ok(pos) = win.window.outer_position() {
                let lpos:LogicalPosition<i32> = pos.to_logical(win.window.current_monitor().unwrap().scale_factor());
                let size = win.window.outer_size().to_logical(win.window.current_monitor().unwrap().scale_factor());
                let bounds = Rectangle::new(lpos.x, lpos.y, size.width, size.height);
                return Some(bounds);
            }
        }
        None
    }
    pub fn set_content_bounds(&mut self, id:&String, bounds:Rectangle) {
        if let Some(win) = self.windows.get(id) {
            win.window.set_outer_position(PhysicalPosition::new(bounds.x, bounds.y));
            win.window.set_inner_size(PhysicalSize::new(bounds.width, bounds.height));
        } else {
            error!("set_content_bounds - frontend_webview not there - id: {}", id);
        }
    }
    pub fn is_maximized(&mut self, id:&String) -> bool {
        if let Some(win) = self.windows.get(id) {
            return win.window.is_maximized();
        }
        false
    }
    pub fn set_maximized(&mut self, id:&String, maximized:bool) {
        if let Some(win) = self.windows.get(id) {
            win.window.set_visible(true);
            win.window.set_maximized(maximized);
        }
    }
    pub fn is_minimized(&mut self, id:&String) -> bool {
        if let Some(win) = self.windows.get(id) {
            return win.window.is_minimized();
        }
        false
    }
    pub fn set_minimized(&mut self, id:&String, minimized:bool) {
        if let Some(win) = self.windows.get(id) {
            win.window.set_minimized(minimized);
        }
    }
    pub fn close(&mut self,  event_loop:&EventLoopWindowTarget<ElectricoEvents>, id:&String) {
        if let Some(win) = self.windows.get(id) {
            if self.window_ids.len()>1 {
                self.window_ids.remove(&win.id);
                self.windows.remove(id);
            } else {
                #[cfg(target_os = "macos")] {
                    use tao::platform::macos::EventLoopWindowTargetExtMacOS;
                    event_loop.hide_application();
                }
                #[cfg(not(target_os = "macos"))] {
                    self.window_ids.remove(&win.id);
                    self.windows.remove(id);
                }
            }
            if self.opened_windows>0 {
                self.opened_windows-=1;
            }
        }
    }
    pub fn count(&mut self) -> usize {
        self.opened_windows
    }
    pub fn set_focus(&mut self, id:&String) {
        if let Some(window) = self.windows.get(id) {
            window.window.set_focus();
        }
    }
    pub fn print(&mut self, id:&String) {
        if let Some(window) = self.windows.get(id) {
            let _ = window.webview.print();
        }
    }
    pub fn toggle_dev_tools(&mut self) {
        for (_id, win) in self.windows.iter() {
            if win.window.is_focused() {
                #[cfg(debug_assertions)] {
                    if win.webview.is_devtools_open() {
                        win.webview.close_devtools();
                    } else {
                        win.webview.open_devtools();
                    }
                }
            }
        }
    }
    pub fn get_actual_window(&mut self) -> Option<&Window>  {
        if let Some(win) = self.windows.values().last() {
            return Some(&win.window);
        }
        None
    }
    pub fn dom_content_loaded(&mut self, id:&String, title:String) {
        if let Some(window) = self.windows.get(id) {
            if title.len()>0 {
                let _ = window.window.set_title(title.as_str());
            }
        } else {
            error!("set_title - frontend_webview not there - id: {}", id);
        }
    }   
}
```

</pre></span></p>


-----------------------

/src/ipcchannel.rs:
-----------------------

<p align="left"><span><pre>

```rust
use std::{collections::HashMap, sync::mpsc::Sender, time::SystemTime};

use log::{error, debug};

pub enum IPCMsg {
    Called,
    //Pending,
    Response {params:String}
}

pub struct IPCChannel {
    ipc_channel:HashMap<String, Sender<IPCMsg>>,
    ipc_channel_timeout:HashMap<String, SystemTime>
}

impl IPCChannel {
    pub fn new() -> IPCChannel {
        IPCChannel {
            ipc_channel:HashMap::new(),
            ipc_channel_timeout:HashMap::new()
        }
    }
    pub fn start(&mut self, k: String, v: Sender<IPCMsg>) {
        self.clean_timeout();   
        self.ipc_channel_timeout.insert(k.clone(), SystemTime::now());
        self.ipc_channel.insert(k, v);
    }
    pub fn get(&mut self, k: &String) ->  Option<&Sender<IPCMsg>> {
        self.ipc_channel.get(k)
    }
    pub fn end(&mut self, k:&String) {
        self.ipc_channel_timeout.remove(k);
        self.ipc_channel.remove(k);
    }
    pub fn clean_timeout(&mut self) {
        for (k, v) in self.ipc_channel_timeout.clone().into_iter() {
            match v.elapsed() {
                Ok(elapsed) => {
                    if elapsed.as_secs()>600 {
                        debug!("ipc_response_clean_timeout request timed out {}", k);
                        self.ipc_channel.remove(&k);
                        self.ipc_channel_timeout.remove(&k); 
                    }
                },
                Err(e) => {
                    error!("ipc_response_clean_timeout SystemTimeError {}", e.to_string());
                }
            }
        }
    }
}
```

</pre></span></p>


-----------------------

/src/js/backend/backend.html:
-----------------------

<h3>Electrico Node Backend</h3>

-----------------------

/src/js/backend/electrico.js:
-----------------------

<p align="left"><span><pre>

```javascript
var __electrico_nonce=null;
(function() {
    let wkeys = ['location', 'screen', '__is_windows', 'createWindow', 'setTimeout', 'fetch', '__init_shared', '__init_require', 'btoa', 'atob'];
    for (let k in window) {
        if (!wkeys.includes(k)) {
            window[k]=()=>{};
        } else {
            //console.log("excluded",k);
        }
    }
    window.__init_shared(window);
    function createCMDRequest(async, name) {
        const req = new XMLHttpRequest();
        req.open("POST", window.__create_protocol_url("cmd://cmd/"+(name!=null?name:"execute")), async);
        return req;
    }
    window.createCMDRequest=createCMDRequest;
    let e_command = function(action) {
        return new Proxy({}, {
            get(target, call, rec) {
                return function(params, data_blob) {
                    let command; let async=false;
                    if (call.startsWith("async")) {
                        async=true;
                        command=call.substring(5);
                    } else if (call.startsWith("sync")) {
                        command=call.substring(4);
                    } else {
                        command=call;
                    }
                    if (params==null) {
                        params={};
                    }
                    let body; let urlcmd=null;
                    let cmdjson = JSON.stringify({"action":action, invoke:{"command":command, ...params}});
                    if (data_blob!=null) {
                        urlcmd=cmdjson;
                        body=data_blob;
                    } else {
                        body=cmdjson;
                    }
                    const req = new XMLHttpRequest();
                    req.open("POST", window.__create_protocol_url("cmd://cmd/"+action+"."+call+(urlcmd!=null?("?"+encodeURIComponent(urlcmd)):"")), async);
                    req.send(body);
                    if (async) {
                        return {
                            then: cb => {
                                req.onreadystatechange = function() {
                                    if (this.readyState == 4) {
                                        if (req.status == 200) {
                                            cb(null, req.responseText);
                                        } else {
                                            cb(req.responseText, null);
                                        }
                                    }
                                };
                            }
                        }
                    } else {
                        if (req.status==200) {
                            return {r:req.responseText};
                        } else {
                            return {e:req.responseText};
                        }
                    }
                };
            }
        });
    }
    window.$e_node=e_command("Node");
    window.$e_electron=e_command("Electron");

    function createLogMsg(level, logmsg, logdata) {
        return {"params":{"level": level, "logmsg":logmsg, "logdata":JSON.stringify(logdata)}};
    }
    
    window.onerror = (event) => {
        window.__electrico.error=event;
    };
    let console_log = window.console.log;
    let console_debug = window.console.debug;
    let console_error = window.console.error;
    let console_warn = window.console.warn;
    let console_trace = window.console.trace;
    window.console.log = (logmsg, ...logdata) => {
        console_log(logmsg, ...logdata);
        $e_node.asyncConsoleLog(createLogMsg("Info", logmsg+"", logdata));
    };
    window.console.info = window.console.log;
    window.console.debug = (logmsg, ...logdata) => {
        console_debug(logmsg, ...logdata);
        $e_node.asyncConsoleLog(createLogMsg("Debug", logmsg+"", logdata));
    };
    window.console.error = (logmsg, ...logdata) => {
        console_error(logmsg, ...logdata);
        for (let i=0; i<logdata.length; i++) {
            if (logdata[i] instanceof Error) logdata[i]=logdata[i].message;
        }
        $e_node.asyncConsoleLog(createLogMsg("Error", logmsg+"", logdata));
    };
    window.console.warn = (logmsg, ...logdata) => {
        console_warn(logmsg, ...logdata);
        $e_node.asyncConsoleLog(createLogMsg("Warn", logmsg+"", logdata));
    };
    window.console.trace = (logmsg, ...logdata) => {
        console_trace(logmsg, ...logdata);
        $e_node.asyncConsoleLog(createLogMsg("Trace", logmsg+"", logdata));
    };
    var SenderCls=null;
    function callChannel(timeout, browserWindowID, requestID, channel, ...args) {
        if (SenderCls==null) {
            let EventEmitter = require('eventemitter3');
            class _SenderCls extends EventEmitter {
                getLastWebPreferences(){
                    return {
                        enableRemoteModule: true
                    }
                }
                getOwnerBrowserWindow () {
                    return window.__electrico.browser_window[browserWindowID];
                }
            }
            SenderCls = _SenderCls;
        }
        let event = new Proxy({}, {
            get(target, prop, rec) {
                if (prop=="reply") {
                    return (response) => {
                        target.returnValue=response;
                    }
                } else if (prop=="sender") {
                    return new SenderCls();
                }
                return target[prop];
            },
            set(target, prop, value) {
                if (prop=="returnValue") {
                    target.returnValue=value;
                    return true;
                }
            }
        });
        let resp = Promise.resolve(window.__electrico.channel[channel](event, ...args));
        setTimeout(()=>{
            resp.then(function(ret) {
                let response = event.returnValue!=null?event.returnValue:ret;
                if (response==undefined) response=null;
                event.returnValue = response;
                timeout.cleared = true;
                const req = createCMDRequest(true, "Frontend.SetIPCResponse");
                req.send(JSON.stringify({"action":"SetIPCResponse", "request_id":requestID, "params": JSON.stringify(response)}));
            }).catch((e) => {
                console.error("callChannel error", e);
                window.__electrico.error = e;
            });
        }, 0);
        return event;
    }
    function exctractIPCParams(argumentsstr) {
        let sep_browserwindow = argumentsstr.indexOf("@");
        let sep_requestid = argumentsstr.indexOf("@@");
        return {
            browserWindowID: argumentsstr.substring(0, sep_browserwindow),
            requestID: argumentsstr.substring(sep_browserwindow+1, sep_requestid),
            arguments: JSON.parse(argumentsstr.substring(sep_requestid+2, argumentsstr.length))
        }
    }
    function wrapNodeInvoke(invoke) {
        return {"action":"Node", invoke:invoke};
    }
    window.__electrico={
        app_menu:{},
        module_paths: {},
        module_cache: {},
        call: (f) => {
            setTimeout(f, 0);
            return "OK";
        },
        child_process: {
            callback: {
                on_stdout: (pid) => {
                    let Buffer = require('buffer').Buffer;
                    let {r, e} = $e_node.syncGetDataBlob({"id":pid});
                    let bdata = Buffer.from(r);
                    let cb = window.__electrico.child_process[pid].stdout_on['data'];
                    if (cb!=null) {
                        cb(bdata);
                    }
                },
                on_stderr: (pid, data) => {
                    let Buffer = require('buffer').Buffer;
                    let {r, e} = $e_node.syncGetDataBlob({"id":pid});
                    let bdata = Buffer.from(r);
                    let cb = window.__electrico.child_process[pid].stderr_on['data'];
                    if (cb!=null) {
                        cb(bdata);
                    }
                },
                on_close: (pid, exit_code) => {
                    let cb = window.__electrico.child_process[pid].on['close'];
                    if (cb!=null) {
                        try {
                            cb(exit_code);
                        } catch (e) {
                            console.log("child_process.on_close", e);
                        }
                    }
                    setTimeout(()=>{
                        delete window.__electrico.child_process[pid];
                    }, 100);
                }
            }
        },
        fs_watcher: {
            on_event: (wid, eventType, filenames) => {
                let files = filenames.split(";");
                for (let file of files) {
                    window.__electrico.fs_watcher[wid].on_event(eventType, file);
                }
            }
        },
        net_server: {
            callback: {
                on_start: (hook, id) => {
                    let server = window.__electrico.net_server[hook];
                    if (server!=null) {
                        server._connection_start(id);
                    }
                },
                on_data: (id) => {
                    let connection = window.__electrico.net_server[id];
                    if (connection!=null) {
                        let Buffer = require('buffer').Buffer;
                        let {r, e} = $e_node.syncGetDataBlob({"id":id});
                        let bdata = Buffer.from(r);
                        connection.emit("data", bdata);
                    }
                },
                on_end: (id) => {
                    let connection = window.__electrico.net_server[id];
                    if (connection!=null) {
                        connection._connection_end(id);
                    }
                }
            }
        },
        net_client: {},
        app: {},
        libs: window.__electrico!=null?window.__electrico.libs:{},
        getLib: (mpath, nonce) => {
            let lib = window.__electrico.libs[mpath];
            return lib;
        },
        callback: {
            "BrowserWindowLoadfile": (id) => {
                //console.trace("BrowserWindowLoadfile done", id);
                let win = window.__electrico.browser_window[id]
                let cb = win.webContents.on['did-finish-load'];
                if (cb!=null) {
                    cb();
                }
            }
        },
        channel:{},
        browser_window: {},
        loadMain: (main) => {
            window.__dirname = window.__electrico.appPath+(main.indexOf("/")>=0?("/"+main.substring(0, main.indexOf("/"))):"");
            window.__Import_meta = {url:window.__dirname};
            if (!main.startsWith("./")) {
                main = "./"+main;
            }
            //setTimeout(()=>{
                require(main);
            //}, 1000);
        },
        callIPCChannel: (argumentsstr) => {
            let p = exctractIPCParams(argumentsstr);
            let channel = p.arguments[0];
            let resp = null;
            delete window.__electrico.error;
            let timeout = {
                "cleared": false,
                trigger: function() {
                    if (!timeout.cleared) {
                        if (resp==null && window.__electrico.error!=null) {
                            console.error("callChannel script error", channel, window.__electrico.error);
                            delete window.__electrico.error;
                            const req = createCMDRequest(true, "Frontend.SetIPCResponse");
                            req.send(JSON.stringify({"action":"SetIPCResponse", "request_id":p.requestID, "params": JSON.stringify(null)}));
                        } else {
                            setTimeout(timeout.trigger, 1000);
                        }
                    }
                }
            };
            setTimeout(timeout.trigger, 1000);
            setTimeout(()=>{
                let event = callChannel(timeout, p.browserWindowID, p.requestID, ...p.arguments);
                if (event.returnValue!=null) {
                    resp=event.returnValue;
                }
            }, 0);
            return "OK";
        },
        getIPCChannelSyncResponse: () => {
            return window.__electrico.ipcChannelSyncResponse;
        },
        callAppOn: (event, windowID) => {
            if (event == "window-close") {
                let winids = windowID!=null?{[windowID]:windowID}:window.__electrico.browser_window;
                for (let winid in winids) {
                    let closeEvent = new CustomEvent("close");
                    let prevented = false;
                    closeEvent.preventDefault=() => {
                        prevented=true;
                    }
                    window.__electrico.browser_window[winid].emit("close", closeEvent);
                    if (!prevented) {
                        const req = createCMDRequest(true, "Frontend.BrowserWindowClose");
                        req.send(JSON.stringify(window.__electrico.wrapInvoke({"command":"BrowserWindowClose", "id":winid}))); 
                    }
                }
            } else {
                const {app} = require('electron/main');
                app.emit(event);
            }
        },
        menuSelected: (menuid) => {
            let item = window.__electrico.app_menu.idmapping[menuid];
            item.click(item);
        },
        domContentLoaded: (windowID) => {
            window.__electrico.browser_window[windowID].domContentLoaded();
        }
    };
    let _process = null;
    let EventEmitter = require('eventemitter3');
    var process=new Proxy(new EventEmitter(), {
        get(target, prop, receiver) {
            if (prop=="stdout") {
                return {
                    write: (d) => {
                        console.log(d);
                    }
                }
            }
            if (prop=="argv") {
                let {r, e} = $e_node.syncGetStartArgs();
                return JSON.parse(r);
            }
            if (prop=="cwd") {
                return () => {
                    return window.__electrico.appPath;
                }
            }
            if (prop=="electronBinding") {
                //console.log("electronBinding");
                return (nodeversion) => {
                    return {
                        getHiddenValue: (w) => {
                            //console.log("getHiddenValue");
                            return "electrico";
                        },
                        isViewApiEnabled: () => {
                            true;
                        }
                    }
                }
            }
            if (_process==null) {
                let {r, e} = $e_node.syncGetProcessInfo();
                _process = JSON.parse(r);
                for (let k in _process) {
                    target[k] = _process[k];
                }
            }
            return target[prop];
        }
    });
    window.process=process;
})();

require("./node.js");
require("./electron.js");

```

</pre></span></p>


-----------------------

/src/js/backend/electron.js:
-----------------------

<p align="left"><span><pre>

```javascript
(function () {
    let EventEmitter = require('eventemitter3');
    window.__electrico = window.__electrico || {libs:{}};
    let uuidv4 = window.__uuidv4;
    function wrapInvoke(invoke) {
        return {"action":"Electron", invoke:invoke};
    }
    window.__electrico.wrapInvoke=wrapInvoke;
    function menuBuildFromTemplate(menu, idmapping) {
        let _idmapping = idmapping || {};
        for (let sub of menu) {
            sub.id = uuidv4();
            _idmapping[sub.id] = sub;
            if (sub.submenu!=null) {
                menuBuildFromTemplate(sub.submenu, _idmapping);
            }
        }
        return _idmapping;
    }
    
    class BrowserWindow extends EventEmitter {
        constructor(config) {
            super();
            if (config.x != null) config.x=Math.floor(config.x);
            if (config.y != null) config.y=Math.floor(config.y);
            this.id="browser_window_"+uuidv4();
            this.config=config;
            class WebContentsCls extends EventEmitter {
                constructor(id) {
                    super();
                    this.id=id;
                }
                openDevTools() {
                    $e_electron.asyncBrowserWindowDevTools({"params":{"id":this.id, "call": "Open"}});
                }
                closeDevTools() {
                    $e_electron.asyncBrowserWindowDevTools({"params":{"id":this.id, "call": "Close"}});
                }
                executeJavaScript (script) {
                    $e_electron.asyncExecuteJavascript({"id":this.id, "script":script});
                }
                printToPDF (options) {
                    $e_electron.syncPrintToPDF({"id":this.id});
                    return "";
                }
                send (channel, ...args) {
                    $e_electron.asyncChannelSendMessage({"id":this.id, "channel":channel, "args":JSON.stringify(args)});
                }
            }
            this.webContents = new WebContentsCls(this.id);
            this.getContentBounds = (() => {
                let {r, e} = $e_electron.syncBrowserWindowBounds({"id":this.id, "params": {"method": "Get"}});
                return JSON.parse(r);
            }).bind(this);
            this.setContentBounds = ((bounds , animate) => {
                $e_electron.asyncBrowserWindowBounds({"id":this.id, "params": {"method":"Set", "bounds":bounds}});
            }).bind(this);
            this.isMaximized = (() => {
                let {r, e} = $e_electron.syncBrowserWindowMaximized({"id":this.id, "id":this.id, "params": {"method": "Get"}});
                return r=="true";
            }).bind(this);
            this.maximize = (() => {
                $e_electron.asyncBrowserWindowMaximized({"id":this.id, "params": {"method":"Set", "maximized":true}});
            }).bind(this);
            this.unmaximize = (() => {
                $e_electron.asyncBrowserWindowMaximized({"id":this.id, "params": {"method":"Set", "maximized":false}});
            }).bind(this);
            this.isMinimized = (() => {
                let {r, e} = $e_electron.syncBrowserWindowMinimized({"id":this.id, "id":this.id, "params": {"method": "Get"}});
                return r=="true";
            }).bind(this);
            this.minimize = (() => {
                $e_electron.asyncBrowserWindowMinimized({"id":this.id, "params": {"method":"Set", "minimized":true}});
            }).bind(this);
            this.close = (() => {
                window.__electrico.callAppOn("window-close", this.id);
            }).bind(this);
            this.show = (() => {
                $e_electron.asyncBrowserWindowShow({"id":this.id, "id":this.id, "shown":true});
            }).bind(this);
            this.hide = (() => {
                $e_electron.asyncBrowserWindowShow({"id":this.id, "id":this.id, "shown":false});
            }).bind(this);
            window.__electrico.browser_window[this.id]=this;
            this.config.title = this.config.title || "Electrico Window";
            this.config.resizable = this.config.resizable!=null?this.config.resizable:true;
            this.config.modal = this.config.modal!=null?this.config.modal:false;
            this.config.show = this.config.show!=null?this.config.show:true;
            if (this.config.webPreferences==null) {
                this.config.webPreferences={};
            }
            if (this.config.webPreferences.nodeIntegration==null) {
                this.config.webPreferences.nodeIntegration=false;
            }
            if (this.config.webPreferences.contextIsolation==null) {
                this.config.webPreferences.contextIsolation=true;
            }
            let {r, e} = $e_electron.syncBrowserWindowCreate({"id":this.id, "params":{"id":this.id, "config": this.config}});
        }
        
        loadFile(file) {
            $e_electron.asyncBrowserWindowLoadfile({"params":{"id":this.id, "file":file, "config": this.config}});
        }
        loadURL(url) {
            $e_electron.asyncBrowserWindowLoadfile({"params":{"id":this.id, "file":url, "config": this.config}});
        }
        removeMenu = () => {
            console.log("BrowserWindow.removeMenu");
        }
        domContentLoaded = () => {
            this.webContents.emit("did-finish-load");
            this.webContents.emit("dom-ready");
        }
    };
    BrowserWindow.getAllWindows = () => {
        let windows = [];
        for (let id in window.__electrico.browser_window) {
            windows.push(window.__electrico.browser_window[id]);
        }
        return windows;
    };
    class AppCls extends EventEmitter {
        constructor() {
            super();
            this.commandLine = {
                appendSwitch: (...args) => {
                    console.log("commandLine.appendSwitch", args);
                },
                getSwitchValue: (k) => {
                    console.log("commandLine.getSwitchValue", k);
                    return null;
                }
            }
            this.enableSandbox = () => {
                console.log("app.enableSandbox");
            }
            this.setPath = (k, path) => {
                console.log("commandLine.setPath", k, path);
            }
            this.getPreferredSystemLanguages = () => {
                return ['en-US'];
            }
            this.getLocale = () => {
                return 'en-US';
            }
            setTimeout(()=>{
                this.emit("ready");
            }, 1000);
        }
        setName (name) {
            window.__electrico.app.name=name;
            $e_electron.asyncAppSetName({"name": name});
        }
        getName() {
            return window.__electrico.app.name;
        }
        getAppPath() {
            let {r, e} = $e_electron.syncGetAppPath();
            return r;
        }
        getPath(path) {
            let {r, e} = $e_electron.syncGetAppPath({"path":path});
            return r;
        }
        whenReady () {
            return {
                then: (cb) => {
                    cb();
                }
            };
        }
        quit() {
            $e_electron.asyncAppQuit({"exit":false});
        }
        exit() {
            $e_electron.asyncAppQuit({"exit":true});
        }
        getVersion(){
            let {r, e} = $e_electron.syncGetAppVersion();
            return r;
        }
        requestSingleInstanceLock(ad) {
            return true;
        }
    }
    let electron = {
        session: {
            defaultSession: {
                webRequest: {
                    onHeadersReceived: (handler) => {
                        //TODO not implemented
                    }
                },
                protocol: {
                    interceptFileProtocol: (schema, handler) => {
                        console.log("interceptFileProtocol", schema);
                        //TODO not implemented
                    },
                    registerFileProtocol: (schema, handler) => {
                        console.log("registerFileProtocol", schema);
                        //TODO not implemented
                    }
                }
            }
        },
        app: new AppCls(),
        ipcMain: {
            on: (channel, fun) => {
                window.__electrico.channel[channel]=fun;
            },
            handle: (channel, fun) => {
                window.__electrico.channel[channel]=fun;
            },
            off: (channel, fun) => {
                delete window.__electrico.channel[channel];
            },
        },
        BrowserWindow: BrowserWindow,
        Menu: {
            buildFromTemplate(template) {
                let idmapping = menuBuildFromTemplate(template);
                window.__electrico.app_menu.idmapping=idmapping;
                return template;
            },
            setApplicationMenu(menu) {
                window.__electrico.app_menu.menu=menu;
                $e_electron.asyncSetApplicationMenu({"menu": menu});
            },
            getApplicationMenu() {
                return window.__electrico.app_menu.menu;
            }
        },
        screen: {
            getPrimaryDisplay: () => {
                return {
                    bounds: {
                        width:window.screen.width,
                        height:window.screen.height,
                        x:0,
                        y:0
                    }
                };
            },
            getAllDisplays: () => {
                return [{
                    bounds: {
                        width:window.screen.width,
                        height:window.screen.height,
                        x:0,
                        y:0
                    }
                }];
            }
        },
        dialog: {
            showOpenDialogSync: (win, options) => {
                if (options==null) {
                    options=win;
                    win=null;
                }
                let {r, e} = $e_electron.syncShowOpenDialogSync({options:options});
                return JSON.parse(r);
            },
            showOpenDialog: (win, options) => {
                if (options==null) {
                    options=win;
                    win=null;
                }
                return new Promise(resolve => {
                    $e_electron.asyncShowOpenDialog({"window_id": win!=null?win.id:null, options:options}).then((e, r)=>{
                        if (e!=null) {
                            let res = JSON.parse(r);
                            resolve({"canceled": res==null, "filePaths":res});
                        } else throw "showOpenDialog failed: "+e;
                    });
                });
            },
            showSaveDialogSync: (win, options) => {
                if (options==null) {
                    options=win;
                    win=null;
                }
                let {r, e} = $e_electron.syncShowSaveDialogSync({options:options});
                JSON.parse(r);
            },
            showSaveDialog: (win, options) => {
                if (options==null) {
                    options=win;
                    win=null;
                }
                return new Promise(resolve => {
                    $e_electron.asyncShowSaveDialog({"window_id": win!=null?win.id:null, options:options}).then((e, r)=>{
                        if (e!=null) {
                            let res = JSON.parse(r);
                            resolve({"canceled": res==null, "filePaths":res});
                        } else throw "showOpenDialog failed: "+e;
                    });
                });
            },
            showMessageBoxSync: (win, options) => {
                if (options==null) {
                    options=win;
                    win=null;
                }
                let {r, e} = $e_electron.syncShowMessageBoxSync({options:options});
                JSON.parse(r);
            }
        },
        shell: {
            openExternal: (url, options) => {
                $e_electron.asyncShellOpenExternal({url:url});
            },
            openPath: (path, options) => {
                $e_electron.asyncShellOpenExternal({url:path});
            }
        },
        protocol: {
            registerSchemesAsPrivileged: (customSchemes) => {
                console.log("registerSchemesAsPrivileged", customSchemes);
            }
        },
        crashReporter: {

        },
        contentTracing: {

        }
    };

    window.__electrico.libs["electron/main"]=electron;
    window.__electrico.libs["electron"]=electron;

    var {Buffer} = require("buffer");
    window.Buffer=Buffer;

    let {r, e} = $e_electron.syncGetAppPath();
    window.__electrico.appPath = r;
})();
```

</pre></span></p>


-----------------------

/src/js/backend/node.js:
-----------------------

<p align="left"><span><pre>

```javascript
(function () {
    //let path = require('path');
    let uuidv4 = window.__uuidv4;
    var global = window;
    window.global = global;
    let Buffer = require('buffer').Buffer;
    let EventEmitter = require('eventemitter3');
    let {queryString} = require('query-string');
    window.__electrico.libs.util = {};
    let inherits = require('inherits');
    window.__electrico.libs["node:inherits"] = inherits;
    window.__electrico.libs.inherits = inherits;
    window.__electrico.libs.util = null;
    let util = require('util');
    let _fd=0;
    util.promisify = (f) => {
        return function(...args) {
            return new Promise((resolve, reject) => {
                f(...args, (err, ...value) => {
                    if (err!=null) {
                        reject(err);
                    } else {
                        resolve(...value);
                    }
                });
            })
        }
    }
    let path = require('path');
    
    window.__electrico = window.__electrico || {libs:{}};
    function wrapInvoke(invoke) {
        return {"action":"Node", invoke:invoke};
    }
    let node = {
        path:null,
        path: path, 
        fs: {
            constants: {
                "F_OK": 1,
                "R_OK": 2,
                "W_OK": 4,
                "X_OK": 8,
            },
            accessSync(path, mode) {
                let {r, e} = $e_node.syncFSAccess({"path":path, "mode": mode!=null?mode:1});
                if (e!=null) throw "file access failed: "+path;
            },
            access(path, mode, cb) {
                if (cb==null) {
                    cb = mode;
                    mode=null;
                }
                $e_node.asyncFSAccess({"path":path, "mode": mode!=null?mode:1}).then((e, r)=>{
                    if (e!=null) {
                        cb("file access failed: "+path);
                    } else {
                        cb();
                    }
                });
            },
            lstatSync(path) {
                let {r, e} = $e_node.syncFSLstat({"path":path});
                if (e!=null) throw "lstat failed: "+path;
                let resp = JSON.parse(r);
                return {
                    isDirectory: () => {
                        return resp.isDirectory
                    },
                    isFile: () => {
                        return !resp.isDirectory
                    },
                    birthtime: resp.birthtime!=null?new Date(resp.birthtime.secs_since_epoch*1000):null,
                    mtime: resp.mtime!=null?new Date(resp.mtime.secs_since_epoch*1000):null
                };
            },
            existsSync(path) {
                let {r, e} = $e_node.syncFSAccess({"path":path, "mode": 1});
                return r=="OK";
            },
            exists(path, mode, cb) {
                if (cb==null) {
                    cb = mode;
                    mode=null;
                }
                $e_node.asyncFSAccess({"path":path, "mode": 1}).then((e, r)=>{
                    if (r=="OK") {
                        cb(true);
                    } else {
                        cb(false);
                    }
                });
            },
            mkdirSync(path, options) {
                if (options!=null && typeof options != 'object') options = {recursive: options};
                let {r, e} = $e_node.syncFSMkdir({"path":path, options:options});
                if (e!=null) throw "mkdir failed: "+path;
                return r;
            },
            mkdir(path, options, cb) {
                if (cb==null) {
                    cb = options;
                    options=null;
                }
                if (options!=null && typeof options != 'object') options = {recursive: options};
                $e_node.asyncFSMkdir({"path":path, options:options}).then((e, r)=>{
                    if (e!==null) {
                        throw "mkdir failed: "+path;
                    } else {
                        cb(r);
                    }
                });
            },
            writeFileSync(path, data, options) {
                if (options!=null && typeof options != 'object') options = {encoding: options};
                let {r, e} = $e_node.syncFSWriteFile({"path":path, options:options}, data);
                if (e!=null) throw "writeFileSync failed: "+path;
            },
            writeFile(path, data, options, cb) {
                if (cb==null) {
                    cb = options;
                    options=null;
                }
                if (options!=null && typeof options != 'object') options = {encoding: options};
                $e_node.asyncFSWriteFile({"path":path, "data": data, options:options}, data).then((e, r)=>{
                    if (e!==null) {
                        throw "writeFile failed: "+path;
                    } else {
                        cb();
                    }
                });
            },
            readFileSync(path, options) {
                if (options!=null && typeof options != 'object') options = {encoding: options};
                let {r, e} = $e_node.syncFSReadFile({"path":path, options:options});
                if (e!=null) throw "readFileSync failed: "+path;
                if (options==null || options.encoding==null) {
                    return Buffer.from(r);
                }
                return r;
            },
            readFile(path, options, cb) {
                if (cb==null) {
                    cb = options;
                    options=null;
                }
                if (options!=null && typeof options != 'object') options = {encoding: options};
                $e_node.asyncFSReadFile({"path":path, options:options}).then((e, r)=>{
                    if (e!==null) {
                        cb(e);
                    } else {
                        if (options==null || options.encoding==null) {
                            cb(null, Buffer.from(r));
                        } else {
                            cb(null, r);
                        }
                    }
                });
            },
            readdirSync(path, options) {
                if (options!=null && typeof options != 'object') options = {encoding: options};
                let {r, e} = $e_node.syncFSReadDir({"path":path, options:options});
                if (e!=null) throw "readdirSync failed: "+path;
                let dirents = JSON.parse(r);
                if (options==null || !options.withFileTypes) {
                    let names = [];
                    for (let de of dirents) {
                        names.push(de.name);
                    }
                    return names;
                }
                return dirents;
            },
            open(path, flags, mode, cb) {
                if (cb==null) {
                    if (mode!=null) {
                        cb=mode; mode=null;
                    } else {
                        cb=flags; flags=null;
                    }
                }
                if (mode==null) mode="0o666";
                if (flags==null) flags="r";
                _fd++;
                $e_node.asyncFSOpen({fd:_fd, "path":path, "flags":flags.toLowerCase(), "mode":mode}).then((e, r)=>{
                    if (e!==null) {
                        cb(e);
                    } else {
                        cb(null, r*1);
                    }
                });
            },
            close(fd, cb) {
                $e_node.asyncFSClose({"fd":fd}).then((e, r)=>{
                    if (e!==null) {
                        cb(e);
                    }
                });
            },
            read(fd, ...args) {
                let buffer, offset=0, length, position, cb;
                if (args.length==5) {
                    buffer=args[0]; offset=args[1]; length=args[2]; position=args[3]; cb=args[4]; 
                } else {
                    let options=null;
                    if (args.length==3) {
                        buffer=args[0];
                        options=args[1];
                        cb=args[2];
                    } else if (Buffer.isBuffer(args[0])) {
                        buffer=args[0];
                        cb=args[1];
                    } else {
                        options=args[0];
                        cb=args[1];
                    }
                    if (options!=null) {
                        offset=options.offset || offset; length=options.length || length; position=options.position || position;
                        if (buffer==null && options.buffer!=null) buffer=options.buffer;
                    }
                    length = buffer.byteLength-offset;
                }
                $e_node.asyncFSRead({"fd":fd, "offset":offset, "length":length, "position":position}).then((e, r)=>{
                    if (e!==null) {
                        cb(e);
                    } else {
                        let br = Buffer.from(r);
                        let bytesRead = Math.min(br.byteLength, buffer.byteLength);
                        br.copy(buffer, 0, 0, bytesRead);
                        cb(null, bytesRead, buffer);
                    }
                });
            },
            write(fd, ...args) {
                let buffer, offset=0, length, position, cb;
                let options=null;
                if (Buffer.isBuffer(args[0])) {
                    buffer=args[0];
                    if (args.length>2) {
                        if (typeof (args[1] === 'object')) {
                            options=args[1];
                        } else {
                            offset=args[1];
                        }
                        if (args.length>3) {
                            length = args[2];
                            if (args.length>4) {
                                position = args[3];
                            }
                        }
                    }
                } else {
                    buffer=Buffer.from(args[0], args.length==4?args[2]:(args.length==3?args[1]:'utf-8'));
                    if (args.length==4) position=args[1];
                }
                cb=args[args.length-1];
                if (options!=null) {
                    offset=options.offset || offset; length=options.length || length; position=options.position || position;
                }
                length = buffer.byteLength-offset;
                $e_node.asyncFSWrite({"fd":fd,"offset":offset, "length":length, "position":position}, buffer).then((e, r)=>{
                    if (e!==null) {
                        cb(e);
                    } else {
                        let written = r*1;
                        cb(null, written, args[0]);
                    }
                });
            },
            realpath: (path, options, cb) => {
                if (cb==null) {
                    cb = options;
                    options=null;
                }
                let {r, e} = $e_node.syncFSRealPath({"path":path});
                if (e!=null) throw "realpath failed: "+path;
                cb(null, r);
            },
            fdatasync: (fd, cb) => {
                $e_node.asyncFSFdatasync({"fd":fd});
            },
            watch(path, options, cb) {
                let wid = uuidv4();
                let {r, e} = $e_node.syncFSWatch({wid:wid, "path":path, options:options});
                if (e!=null) {
                    throw "fs.watch error: "+e;
                }
                class WatcherCls extends EventEmitter {
                    constructor() {
                        super();
                        this.on_event = (eventType, filename) => {
                            let mEventType = null;
                            if (eventType.startsWith("Modify(Name(")) {
                                mEventType = "rename";
                            } else if (eventType.startsWith("Create(")) {
                                mEventType = "change";
                            } else if (eventType.startsWith("Modify(Data(")) {
                                mEventType = "change";
                            } else if (eventType.startsWith("Modify(Any)")) {
                                mEventType = "change";
                            } else if (eventType.startsWith("Modify(Metadata(Extended))")) {
                                mEventType = "change";
                            }
                            if (mEventType!=null) {
                                this.emit("change", mEventType, filename);
                                if (cb!=null) {
                                    cb(eventType, filename);
                                }
                            }
                        }
                        this.close = () => {
                            $e_node.asyncFSWatchClose({wid:wid});
                        }
                    }
                }
                let watcher = new WatcherCls();
                window.__electrico.fs_watcher[wid] = watcher;
                return watcher;
            },
            promises: {
                stat: (path) => {
                    return new Promise((resolve, reject)=>{
                        resolve(node.fs.lstatSync(path));
                    });
                },
                readdir: (path) => {
                    return new Promise((resolve, reject)=>{
                        resolve(node.fs.readdirSync(path));
                    });
                },
                mkdir: (path, options) => {
                    return new Promise((resolve, reject)=>{
                        resolve(node.fs.mkdirSync(path, options));
                    });
                },
                readFile: (path, options) => {
                    return new Promise((resolve, reject)=>{
                        resolve(node.fs.readFileSync(path, options));
                    });
                }
            }
        },
        http: {
            request(options, cb) {
                let req_events =  {};
                let resp_events =  {};
                let req = createCMDRequest(true);
                req.onreadystatechange = function() {
                    if (this.readyState == 4) {
                        if (cb!=null) {
                            cb({
                                statusCode: req.status,
                                headers: req.getAllResponseHeaders().split("\r\n"),
                                on: (event, cb) => {
                                    resp_events[event] = cb;
                                }
                            });
                        }
                        if (req.status == 200) {
                            if (resp_events["data"]!=null) {
                                resp_events["data"](req.response);
                            }
                        } else if (req_events["error"]!=null) {
                            req_events["error"]("error status "+req.status);
                        }
                    }
                };
                req.error = function(e) {
                    if (req_events["error"]!=null) {
                        req_events["error"]("error "+e);
                    }
                }
                return {
                    on: (event, cb) => {
                        req_events[event] = cb;
                    },
                    end: () => {
                        req.send(JSON.stringify(wrapInvoke({"command":"HTTPRequest", options:options})));
                    }
                }
            }
        },
        child_process: {
            spawn: function(cmd, args, options) {
                let {r, e} = $e_node.syncChildProcessSpawn({cmd:cmd, args:args});
                if (e!=null) {
                    throw "child_process.spawn error: "+e;
                }
                let pid = r;
                let proc = {
                    pid: pid,
                    on: {},
                    stdout_on: {},
                    stderr_on: {},
                    stdin: {
                        write: (data) => {
                            let {r, e} = $e_node.syncChildProcessStdinWrite({pid: pid}, data);
                            if (e!=null) {
                                throw "child_process.stdin.write error: "+e;
                            }
                        }
                    },
                    stdout: {
                        on: (event, cb) => {
                            proc.stdout_on[event] = cb;
                        }
                    },
                    stderr: {
                        on: (event, cb) => {
                            proc.stderr_on[event] = cb;
                        }
                    },
                    on: (event, cb) => {
                        proc.on[event] = cb;
                    },
                    disconnect: () => {
                        let {r, e} = $e_node.syncChildProcessDisconnect({pid: pid});
                        if (e!=null) {
                            throw "child_process.disconnect error: "+e;
                        }
                    }
                };
                window.__electrico.child_process[pid] = proc;
                return proc;
            }
        },
        os: {
            homedir: () => {
                if (window.__electrico.homedir==null) {
                    let {r, e} = $e_electron.syncGetAppPath({ "path":"userHome"});
                    window.__electrico.homedir = r;
                }
                return window.__electrico.homedir;
            },
            tmpdir: () => {
                if (window.__electrico.tmpdir==null) {
                    let {r, e} = $e_electron.syncGetAppPath({ "path":"temp"});
                    window.__electrico.tmpdir = r;
                }
                return window.__electrico.tmpdir;
            }
        },
        querystring: queryString,
        util: util,
        events: EventEmitter,
        url: {
            fileURLToPath: (file) => {
                return file;
            }
        },
        module: {
            createRequire: (file) => {
                return require;
            },
            register: (script, path) => {

            }
        },
        crypto: {
            createHash: (alg) => {
                if (alg=="sha256") {
                    let SHA256 = require("crypto-js/sha256");
                    return {
                        update: (text) => {
                            let hash = SHA256(text);
                            return {
                                digest: (d) => {
                                    if (d=="hex") {
                                        return hash.toString();
                                    } else {
                                        throw "createHash - unknown digest: "+d;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    throw "createHash - unknown algorithm: "+alg;
                }
            }
        },
        net: {
            createServer: function(options, listener) {
                if (listener==null) {
                    listener=options;
                    options=null;
                }
                class ServerCls extends EventEmitter {
                    constructor() {
                        super();
                        this._connections={};
                        this.listen = ((hook, cb) => {
                            if (cb!=null) {
                                this.on("listening", cb);
                            }
                            let {r, e} = $e_node.syncNETCreateServer({"hook":hook, "options":options});
                            if (e==null) {
                                window.__electrico.net_server[hook]=this;
                                this.id=r;
                                this.emit("listening");
                            } else {
                                this.emit("error", e);
                            }
                        }).bind(this);
                        this.close = ((cb) => {
                            for (let cid in this._connections) {
                                let {r, e} = $e_node.syncNETCloseConnection({"id":cid});
                                this._connections[cid].emit("close");
                                delete window.__electrico.net_server[cid];
                            }
                            let {r, e} = $e_node.syncNETCloseServer({"id":this.id});
                            this._connections={};
                            for (let id in window.__electrico.net_server) {
                                if (window.__electrico.net_server[id]==this) {
                                    delete window.__electrico.net_server[id];
                                }
                            }
                            this.emit("close");
                            if (cb!=null) cb();
                        }).bind(this);
                        this._connection_start = (id => {
                            class ConnectionCls extends EventEmitter {
                                constructor(server) {
                                    super();
                                    server._connections[id] = this;
                                    this.write = ((data, encoding, cb) => {
                                        if (cb==null) {
                                            cb=encoding;
                                            encoding=null;
                                        }
                                        encoding = encoding || 'utf-8';
                                        if (!Buffer.isBuffer(data)) {
                                            data=Buffer.from(data, encoding);
                                        }
                                        $e_node.asyncNETWriteConnection({"id":id}, data).then((e, r)=>{
                                            if (cb!=null) cb(e==null);
                                        });
                                    }).bind(this);
                                    this.end = ((data, encoding, cb) => {
                                        cb = cb || encoding;
                                        let end = () => {
                                            setTimeout(()=>{
                                                let {r, e} = $e_node.syncNETCloseConnection({"id":id});
                                            }, 100);
                                        };
                                        if (data!=null) {
                                            this.write(data, encoding, ()=>{
                                                end();
                                            });
                                        } else {
                                            end();
                                        }
                                    }).bind(this);
                                    this._connection_end = (id => {
                                        this.emit("end");
                                        delete server._connections[id];
                                        delete window.__electrico.net_server[id];
                                    }).bind(this);
                                }
                            }
                            let connection = new ConnectionCls(this);
                            window.__electrico.net_server[id] = connection;
                            this.emit("connection", connection);
                        }).bind(this);
                    }
                }
                let server = new ServerCls();
                if (listener!=null) {
                    server.on("connection", listener);
                }
                return server;
            },
            createConnection: function (hook, listener) {
                class ConnectionCls extends EventEmitter {
                    constructor() {
                        super();
                        this.write = ((data, encoding, cb) => {
                            if (cb==null) {
                                cb=encoding;
                                encoding=null;
                            }
                            encoding = encoding || 'utf-8';
                            if (!Buffer.isBuffer(data)) {
                                data=Buffer.from(data, encoding);
                            }
                            $e_node.asyncNETWriteConnection({"id":id}, data).then((e, r)=>{
                                if (cb!=null) cb(e==null);
                            });
                        }).bind(this);
                        this.end = ((data, encoding, cb) => {
                            cb = cb || encoding;
                            let end = () => {
                                setTimeout(()=>{
                                    let {r, e} = $e_node.syncNETCloseConnection({"id":id});
                                }, 100);
                            };
                            if (data!=null) {
                                this.write(data, encoding, ()=>{
                                    end();
                                });
                            } else {
                                end();
                            }
                        }).bind(this);
                        this._connection_end = (id => {
                            this.emit("end");
                            delete window.__electrico.net_server[id];
                        }).bind(this);
                    }
                }
                let id = uuidv4();
                let connection = new ConnectionCls();
                if (listener!=null) {
                    connection.on("connect", listener);
                }
                window.__electrico.net_server[id] = connection;
                let {r, e} = $e_node.syncNETCreateConnection({"id":id, "hook":hook});
                if (e!=null) {
                    console.error("createConnection error: ", e);
                    setTimeout(()=>{
                        connection.emit("error", e);
                    }, 0);
                } else {
                    setTimeout(()=>{
                        connection.emit("connect");
                    }, 0);
                }
                return connection;
            }
        },
        zlib :{
            createDeflateRaw: {

            },
            createInflateRaw: {

            }
        }
    };
    window.__electrico.libs["node:path"] = node.path;
    window.__electrico.libs.path = node.path;
    window.__electrico.libs["node:fs"] = node.fs;
    window.__electrico.libs.fs = node.fs;
    window.__electrico.libs["node:child_process"] = node.child_process;
    window.__electrico.libs.child_process = node.child_process;
    window.__electrico.libs["node:https"] = node.http;
    window.__electrico.libs.https = node.http;
    window.__electrico.libs["node:http"] = node.http;
    window.__electrico.libs.http = node.http;
    window.__electrico.libs["node:os"] = node.os;
    window.__electrico.libs.os = node.os;
    window.__electrico.libs["node:querystring"] = node.querystring;
    window.__electrico.libs.querystring = node.querystring;
    window.__electrico.libs["node:util"] = node.util;
    window.__electrico.libs.util = node.util;
    window.__electrico.libs["node:events"] = node.events;
    window.__electrico.libs.events = node.events;
    window.__electrico.libs["node:url"] = node.url;
    window.__electrico.libs.url = node.url;
    window.__electrico.libs["node:module"] =node.module;
    window.__electrico.libs.module = node.module;
    window.__electrico.libs["node:crypto"] =node.crypto;
    window.__electrico.libs.crypto = node.crypto;
    window.__electrico.libs["node:net"] =node.net;
    window.__electrico.libs.net = node.net;
    window.__electrico.libs["node:zlib"] =node.zlib;
    window.__electrico.libs.zlib = node.zlib;
})();
```

</pre></span></p>


-----------------------

/src/js/backend/package-lock.json:
-----------------------

<p align="left"><span><pre>

```json
{
  "name": "electrico-backend",
  "version": "0.5.0",
  "lockfileVersion": 3,
  "requires": true,
  "packages": {
    "": {
      "name": "electrico-backend",
      "version": "0.5.0",
      "dependencies": {
        "buffer": "^6.0.3",
        "crypto-js": "4.2.0",
        "eventemitter3": "^5.0.1",
        "path": "^0.12.7",
        "query-string": "^9.1.0"
      }
    },
    "node_modules/base64-js": {
      "version": "1.5.1",
      "resolved": "https://registry.npmjs.org/base64-js/-/base64-js-1.5.1.tgz",
      "integrity": "sha512-AKpaYlHn8t4SVbOHCy+b5+KKgvR4vrsD8vbvrbiQJps7fKDTkjkDry6ji0rUJjC0kzbNePLwzxq8iypo41qeWA==",
      "funding": [
        {
          "type": "github",
          "url": "https://github.com/sponsors/feross"
        },
        {
          "type": "patreon",
          "url": "https://www.patreon.com/feross"
        },
        {
          "type": "consulting",
          "url": "https://feross.org/support"
        }
      ]
    },
    "node_modules/buffer": {
      "version": "6.0.3",
      "resolved": "https://registry.npmjs.org/buffer/-/buffer-6.0.3.tgz",
      "integrity": "sha512-FTiCpNxtwiZZHEZbcbTIcZjERVICn9yq/pDFkTl95/AxzD1naBctN7YO68riM/gLSDY7sdrMby8hofADYuuqOA==",
      "funding": [
        {
          "type": "github",
          "url": "https://github.com/sponsors/feross"
        },
        {
          "type": "patreon",
          "url": "https://www.patreon.com/feross"
        },
        {
          "type": "consulting",
          "url": "https://feross.org/support"
        }
      ],
      "dependencies": {
        "base64-js": "^1.3.1",
        "ieee754": "^1.2.1"
      }
    },
    "node_modules/crypto-js": {
      "version": "4.2.0",
      "resolved": "https://registry.npmjs.org/crypto-js/-/crypto-js-4.2.0.tgz",
      "integrity": "sha512-KALDyEYgpY+Rlob/iriUtjV6d5Eq+Y191A5g4UqLAi8CyGP9N1+FdVbkc1SxKc2r4YAYqG8JzO2KGL+AizD70Q=="
    },
    "node_modules/decode-uri-component": {
      "version": "0.4.1",
      "resolved": "https://registry.npmjs.org/decode-uri-component/-/decode-uri-component-0.4.1.tgz",
      "integrity": "sha512-+8VxcR21HhTy8nOt6jf20w0c9CADrw1O8d+VZ/YzzCt4bJ3uBjw+D1q2osAB8RnpwwaeYBxy0HyKQxD5JBMuuQ==",
      "engines": {
        "node": ">=14.16"
      }
    },
    "node_modules/eventemitter3": {
      "version": "5.0.1",
      "resolved": "https://registry.npmjs.org/eventemitter3/-/eventemitter3-5.0.1.tgz",
      "integrity": "sha512-GWkBvjiSZK87ELrYOSESUYeVIc9mvLLf/nXalMOS5dYrgZq9o5OVkbZAVM06CVxYsCwH9BDZFPlQTlPA1j4ahA=="
    },
    "node_modules/filter-obj": {
      "version": "5.1.0",
      "resolved": "https://registry.npmjs.org/filter-obj/-/filter-obj-5.1.0.tgz",
      "integrity": "sha512-qWeTREPoT7I0bifpPUXtxkZJ1XJzxWtfoWWkdVGqa+eCr3SHW/Ocp89o8vLvbUuQnadybJpjOKu4V+RwO6sGng==",
      "engines": {
        "node": ">=14.16"
      },
      "funding": {
        "url": "https://github.com/sponsors/sindresorhus"
      }
    },
    "node_modules/ieee754": {
      "version": "1.2.1",
      "resolved": "https://registry.npmjs.org/ieee754/-/ieee754-1.2.1.tgz",
      "integrity": "sha512-dcyqhDvX1C46lXZcVqCpK+FtMRQVdIMN6/Df5js2zouUsqG7I6sFxitIC+7KYK29KdXOLHdu9zL4sFnoVQnqaA==",
      "funding": [
        {
          "type": "github",
          "url": "https://github.com/sponsors/feross"
        },
        {
          "type": "patreon",
          "url": "https://www.patreon.com/feross"
        },
        {
          "type": "consulting",
          "url": "https://feross.org/support"
        }
      ]
    },
    "node_modules/inherits": {
      "version": "2.0.3",
      "resolved": "https://registry.npmjs.org/inherits/-/inherits-2.0.3.tgz",
      "integrity": "sha512-x00IRNXNy63jwGkJmzPigoySHbaqpNuzKbBOmzK+g2OdZpQ9w+sxCN+VSB3ja7IAge2OP2qpfxTjeNcyjmW1uw=="
    },
    "node_modules/path": {
      "version": "0.12.7",
      "resolved": "https://registry.npmjs.org/path/-/path-0.12.7.tgz",
      "integrity": "sha512-aXXC6s+1w7otVF9UletFkFcDsJeO7lSZBPUQhtb5O0xJe8LtYhj/GxldoL09bBj9+ZmE2hNoHqQSFMN5fikh4Q==",
      "dependencies": {
        "process": "^0.11.1",
        "util": "^0.10.3"
      }
    },
    "node_modules/process": {
      "version": "0.11.10",
      "resolved": "https://registry.npmjs.org/process/-/process-0.11.10.tgz",
      "integrity": "sha512-cdGef/drWFoydD1JsMzuFf8100nZl+GT+yacc2bEced5f9Rjk4z+WtFUTBu9PhOi9j/jfmBPu0mMEY4wIdAF8A==",
      "engines": {
        "node": ">= 0.6.0"
      }
    },
    "node_modules/query-string": {
      "version": "9.1.0",
      "resolved": "https://registry.npmjs.org/query-string/-/query-string-9.1.0.tgz",
      "integrity": "sha512-t6dqMECpCkqfyv2FfwVS1xcB6lgXW/0XZSaKdsCNGYkqMO76AFiJEg4vINzoDKcZa6MS7JX+OHIjwh06K5vczw==",
      "dependencies": {
        "decode-uri-component": "^0.4.1",
        "filter-obj": "^5.1.0",
        "split-on-first": "^3.0.0"
      },
      "engines": {
        "node": ">=18"
      },
      "funding": {
        "url": "https://github.com/sponsors/sindresorhus"
      }
    },
    "node_modules/split-on-first": {
      "version": "3.0.0",
      "resolved": "https://registry.npmjs.org/split-on-first/-/split-on-first-3.0.0.tgz",
      "integrity": "sha512-qxQJTx2ryR0Dw0ITYyekNQWpz6f8dGd7vffGNflQQ3Iqj9NJ6qiZ7ELpZsJ/QBhIVAiDfXdag3+Gp8RvWa62AA==",
      "engines": {
        "node": ">=12"
      },
      "funding": {
        "url": "https://github.com/sponsors/sindresorhus"
      }
    },
    "node_modules/util": {
      "version": "0.10.4",
      "resolved": "https://registry.npmjs.org/util/-/util-0.10.4.tgz",
      "integrity": "sha512-0Pm9hTQ3se5ll1XihRic3FDIku70C+iHUdT/W926rSgHV5QgXsYbKZN8MSC3tJtSkhuROzvsQjAaFENRXr+19A==",
      "dependencies": {
        "inherits": "2.0.3"
      }
    }
  }
}
```

</pre></span></p>


-----------------------

/src/js/backend/package.json:
-----------------------

<p align="left"><span><pre>

```json
{
  "name": "electrico-backend",
  "version": "0.5.0",
  "description": "Electrico Backend JS",
  "author": "Thomas Tschurtschenthaler",
  "dependencies": {
    "buffer": "^6.0.3",
    "eventemitter3": "^5.0.1",
    "path": "^0.12.7",
    "query-string": "^9.1.0",
    "crypto-js": "4.2.0"
  }
}
```

</pre></span></p>


-----------------------

/src/js/frontend/electrico.js:
-----------------------

<p align="left"><span><pre>

```javascript
(function() {
    let initscript = function(document) {
        let ipcRenderer = null;
        let _XMLHttpRequest = XMLHttpRequest;
        var window=document.window;
        __init_shared(window);
        window.alert = (msg) => {
            const req = new XMLHttpRequest();
            req.open("POST", window.__create_protocol_url("ipc://ipc/send"), false);
            req.send(JSON.stringify({"action": "Alert", "message": msg}));
        }
        function sendIPC(request_id, nonce, async, ...args) {
            const req = new _XMLHttpRequest();
            req.open("POST", window.__create_protocol_url("ipc://ipc/send"), async);
            req.send(JSON.stringify({"action":"PostIPC", "request_id":request_id, "nonce": nonce, "params":JSON.stringify(args)}));
            return req;
        }
        let uuidv4 = window.__uuidv4;
        function processi(nonce) {
            let _processInfo=null;
            return new Proxy({}, {
                get(target, prop, receiver) {
                    if (_processInfo==null) {
                        if (nonce!=null) {
                            const req = new _XMLHttpRequest();
                            req.open("POST", window.__create_protocol_url("ipc://ipc/send"), false);
                            req.send(JSON.stringify({"action":"GetProcessInfo", "nonce":nonce}));
                            _processInfo = JSON.parse(req.responseText);
                        } else {
                            _processInfo = {};
                        }
                    }
                    if (prop=="on") {
                        return (event, f) => {
                            //console.log("process on", event, f);
                        }
                    } else if (prop=="electronBinding") {
                        //console.log("electronBinding");
                        return (nodeversion) => {
                            return {
                                getHiddenValue: (w) => {
                                    //console.log("getHiddenValue");
                                    return "electrico";
                                },
                                isViewApiEnabled: () => {
                                    true;
                                }
                            }
                        }
                    } else if (prop=="argv") {
                        return [];
                    }
                    return _processInfo[prop];
                }
            });
        }
        window.process = processi(__electrico_nonce);
       
        let _electron_i = {};
        
        let _electron = function(nonce) {
            if (_electron_i[nonce]!=null) {
                return _electron_i[nonce];
            }
            let EventEmitter = require('eventemitter3');
            class IpcRendererCls extends EventEmitter {
                constructor(nonce) {
                    super();
                    this.nonce=nonce;
                }
                send(...args) {
                    sendIPC(uuidv4(), this.nonce, true, ...args);
                }
                sendSync(...args) {
                    window.__electrico.ipcSyncResponse=null;
                    let req = sendIPC(uuidv4(), this.nonce, false, ...args);
                    if (req.readyState == 4 && req.status == 200) {
                        return JSON.parse(req.responseText);
                    }
                    console.error("sendSync request failed - timeout");
                    return null;
                }
                invoke(...args) {
                    return new Promise(resolve => {
                        let req = sendIPC(uuidv4(), this.nonce, true, ...args);
                        req.onreadystatechange = function() {
                            if (this.readyState == 4) {
                                if (req.status == 200) {
                                    resolve(JSON.parse(req.responseText));
                                } else {
                                    console.error("invoke async response failed - timeout");
                                    resolve(null);
                                }
                            }
                        };
                    });
                }
            }
            let _ipcRenderer = new IpcRendererCls(nonce);
            if (nonce!=null && nonce.length>0 && ipcRenderer==null) {
                ipcRenderer = _ipcRenderer
            }
            _electron_i[nonce] = {
                ipcRenderer: _ipcRenderer,
                contextBridge: {
                    exposeInMainWorld: (method, fun) => {
                        window[method] = fun;
                    }
                }
            }
            return _electron_i[nonce];
        };
        electron = {
            __init_electrico_nonce: (nonce) => {
                return _electron(nonce);
            }
        }
        window.__electrico={
            module_paths: {},
            module_cache: {},
            channel: {},
            libs: {
                "electron":electron,
            },
            replaceImports: (script) => {
                return script.replaceAll(/\import  *([^ ]*) *from *([^{ ,;,\r, \n}]*)/g, "var $1 = __import($2)");
            },
            getLib: (mpath, nonce) => {
                let lib = window.__electrico.libs[mpath];
                if (lib!=null && nonce!=null && lib.__init_electrico_nonce!=null) {
                    lib = lib.__init_electrico_nonce(nonce);
                }
                return lib;
            },
            sendChannelMessage: (argumentsstr) => {
                setTimeout(()=>{
                    let sep_channel = argumentsstr.indexOf("@");
                    let channel = argumentsstr.substring(0, sep_channel);
                    let args = JSON.parse(argumentsstr.substring(sep_channel+1, argumentsstr.length));
                    ipcRenderer.emit(channel, {}, ...args);
                }, 0);
                return "OK";
            }
        };
        let _addEventListener = window.addEventListener;
        window.__electrico_preload(document, {
            before: (nonce) => {
                window.addEventListener = (e, h) => {
                    _addEventListener(e, (e)=>{
                        let process=processi(nonce);
                        let he = "("+h.toString()+")(e)";
                        eval(he);
                    })
                };
            },
            after: () => {
                window.addEventListener=_addEventListener;
                window.process=processi(null);
            }
        });
        //setTimeout(()=>{window.__electrico_preload(document);}, 1000);
        
        let start = (new Date()).getTime();
        let init_iframes = (nonce)=>{
            let iframes = document.querySelectorAll("iframe");
            if (iframes.length>0) {
                for (let i=0; i<iframes.length; i++) {
                    try {
                        let framewindow = iframes[i].contentWindow;
                        framewindow.initscript=initscript;
                        framewindow.__electrico_preload=window.__electrico_preload;
                        framewindow.__init_require=window.__init_require;
                        let _addEventListener = framewindow.addEventListener;
                        let domLoadedHandlers = [];
                        framewindow.addEventListener = (event, handler)=>{
                            if (event=="DOMContentLoaded") {
                                domLoadedHandlers.push(handler);
                            } else {
                                _addEventListener(event, handler);
                            }
                        }
                        __electrico_nonce=nonce;
                        framewindow.eval("window.document.window=window; window.initscript(window.document);");
                        __electrico_nonce='';
                        setTimeout(()=>{
                            console.trace("calling iframe domLoadedHandlers preload handlers", framewindow.document.documentElement);
                            for (let h of domLoadedHandlers) {
                                h();
                            }
                        }, 0);
                    } catch (e) {
                        console.error("electrico frame init error", e);
                    }
                }
            } else if ((new Date()).getTime()-start<2000) {
                setTimeout(()=>{init_iframes(nonce);}, 200);
            }
        };
        init_iframes(__electrico_nonce);
    }
    document.window=window;
    initscript(document);
    var {Buffer} = require("buffer");
    window.Buffer=Buffer;
    window.addEventListener("DOMContentLoaded", ()=>{
        const req = new XMLHttpRequest();
        req.open("POST", window.__create_protocol_url("ipc://ipc/send"), true);
        req.send(JSON.stringify({"action": "DOMContentLoaded", "title": document.title}));
    })
})();
```

</pre></span></p>


-----------------------

/src/js/frontend/package-lock.json:
-----------------------

<p align="left"><span><pre>

```json
{
  "name": "electrico-frontend",
  "version": "0.5.0",
  "lockfileVersion": 3,
  "requires": true,
  "packages": {
    "": {
      "name": "electrico-frontend",
      "version": "0.5.0",
      "dependencies": {
        "buffer": "^6.0.3",
        "eventemitter3": "^5.0.1"
      }
    },
    "node_modules/base64-js": {
      "version": "1.5.1",
      "resolved": "https://registry.npmjs.org/base64-js/-/base64-js-1.5.1.tgz",
      "integrity": "sha512-AKpaYlHn8t4SVbOHCy+b5+KKgvR4vrsD8vbvrbiQJps7fKDTkjkDry6ji0rUJjC0kzbNePLwzxq8iypo41qeWA==",
      "funding": [
        {
          "type": "github",
          "url": "https://github.com/sponsors/feross"
        },
        {
          "type": "patreon",
          "url": "https://www.patreon.com/feross"
        },
        {
          "type": "consulting",
          "url": "https://feross.org/support"
        }
      ]
    },
    "node_modules/buffer": {
      "version": "6.0.3",
      "resolved": "https://registry.npmjs.org/buffer/-/buffer-6.0.3.tgz",
      "integrity": "sha512-FTiCpNxtwiZZHEZbcbTIcZjERVICn9yq/pDFkTl95/AxzD1naBctN7YO68riM/gLSDY7sdrMby8hofADYuuqOA==",
      "funding": [
        {
          "type": "github",
          "url": "https://github.com/sponsors/feross"
        },
        {
          "type": "patreon",
          "url": "https://www.patreon.com/feross"
        },
        {
          "type": "consulting",
          "url": "https://feross.org/support"
        }
      ],
      "dependencies": {
        "base64-js": "^1.3.1",
        "ieee754": "^1.2.1"
      }
    },
    "node_modules/eventemitter3": {
      "version": "5.0.1",
      "resolved": "https://registry.npmjs.org/eventemitter3/-/eventemitter3-5.0.1.tgz",
      "integrity": "sha512-GWkBvjiSZK87ELrYOSESUYeVIc9mvLLf/nXalMOS5dYrgZq9o5OVkbZAVM06CVxYsCwH9BDZFPlQTlPA1j4ahA=="
    },
    "node_modules/ieee754": {
      "version": "1.2.1",
      "resolved": "https://registry.npmjs.org/ieee754/-/ieee754-1.2.1.tgz",
      "integrity": "sha512-dcyqhDvX1C46lXZcVqCpK+FtMRQVdIMN6/Df5js2zouUsqG7I6sFxitIC+7KYK29KdXOLHdu9zL4sFnoVQnqaA==",
      "funding": [
        {
          "type": "github",
          "url": "https://github.com/sponsors/feross"
        },
        {
          "type": "patreon",
          "url": "https://www.patreon.com/feross"
        },
        {
          "type": "consulting",
          "url": "https://feross.org/support"
        }
      ]
    }
  }
}
```

</pre></span></p>


-----------------------

/src/js/frontend/package.json:
-----------------------

{
  "name": "electrico-frontend",
  "version": "0.5.0",
  "description": "Electrico Frontend JS",
  "author": "Thomas Tschurtschenthaler",
  "dependencies": {
    "buffer": "^6.0.3",
    "eventemitter3": "^5.0.1"
  }
}


-----------------------

/src/js/shared/require.js:
-----------------------

<p align="left"><span><pre>

```javascript
(function() {
    window.__init_require = function (window) {
        //console.trace("__init_require call", window);
        function fromCache(expanded_path) {
            return window.__electrico.module_cache[expanded_path];
        }
        function normalize(path) {
            let npath=[];
            for (let p of path.split("/")) {
                if (p==".") continue;
                if (p==".." && npath.length>0) {
                    npath.pop();
                } else {
                    npath.push(p);
                }
            }
            let rpath = npath.join("/");
            if (rpath.startsWith("/")) rpath = rpath.substring(1);
            //console.log(path, rpath);
            return rpath;
        }
        function loadModule(_this, mpath, cache) {
            let lib = window.__electrico.getLib(mpath, __electrico_nonce);
            if (lib!=null) {
                return lib;
            }
            var module = {}; var exports = {__electrico_deferred:[]};
            let module_path = _this!=null?_this.__import_mpath:"";
            let expanded_path = module_path;
            if (mpath.startsWith(".")) {
                expanded_path+="/"+mpath;
            } else {
                expanded_path="node_modules/"+mpath;
            }
            expanded_path = normalize(expanded_path);
            let cache_path = expanded_path;
            let cached = fromCache(cache_path);
            if (cached!=null && cached!="" && cache) {
               return cached;
            }
            let script=null; let req={};
            if (cached!="") {
                let jsfilepath = window.__create_protocol_url("fil://mod/"+((expanded_path.lastIndexOf(".")<expanded_path.lastIndexOf("/"))?expanded_path+".js":expanded_path));
                req = new XMLHttpRequest();
                req.open("GET", jsfilepath, false);
                req.send();
            }
            if (cached=="" || req.status==301) {
                //console.trace("js file not found", expanded_path);
                let package_path = window.__create_protocol_url("fil://mod/"+expanded_path+"/package.json");
                const preq = new XMLHttpRequest();
                preq.open("GET", package_path, false);
                preq.send();
                if (preq.status==301) {
                    console.error("js file not found - no package.json", package_path);
                    return null;
                }
                let package = JSON.parse(preq.responseText);
                let mainjs = package.main!=null?package.main:(package.exports!=null?(package.exports.default!=null?package.exports.default:package.exports):package.files[0]);
                expanded_path = expanded_path+"/"+mainjs;
                
                if (!expanded_path.endsWith("js")) expanded_path+=".js";
                expanded_path = normalize(expanded_path);
                
                const req2 = new XMLHttpRequest();
                let jsfilepath = window.__create_protocol_url("fil://mod/"+expanded_path);
                req2.open("GET", jsfilepath, false);
                req2.send();
                if (req2.status==404) {
                    console.error("js file not found", jsfilepath);
                    return null;
                }
                script=req2.responseText;
            } else {
                script=req.responseText;
            }
            let exported = null;
            if (mpath.endsWith(".json")) {
                exported = JSON.parse(script);
            } else {
                let _this = {"__import_mpath":expanded_path.substring(0, expanded_path.lastIndexOf("/"))};
                let sourceURL = "//# sourceURL="+expanded_path+"\n";
                script = window.__replaceImports(script);
                script = sourceURL+"{\nlet __require_this=_this;"+script+"\n}";
                try {
                    eval(script);
                } catch (e) {
                    //console.log("require error", expanded_path, script, e);
                    throw e;
                }
                if (exports.__electrico_deferred!=null) {
                    for (let def of exports.__electrico_deferred) {
                        def();
                    }
                    delete exports.__electrico_deferred;
                }
                exported = module.exports || exports;
            }
            if (cache) {
                window.__electrico.module_cache[cache_path]=exported;
            }
            return exported;
        }
        window.__Import=function(_this, selector, mpath, doExport, exports) {
            //console.log("__import", mpath, selector);
            let mod = loadModule(_this, mpath, true);
            let toEval="";
            if (selector!=null) {
                selector = selector.trim();
                let vlnames = false;
                if (selector.startsWith("{") && selector.endsWith("}")) {
                    vlnames = true;
                    selector = selector.substring(1, selector.length-1);
                }
                let parts = selector.split(",");
                for (let i=0; i<parts.length; i++) {
                    let part = parts[i];
                    let vparts = part.split(" as ");
                    let vname = null; let vlname = null;
                    if (vparts.length>1) {
                        vlname =  vparts[0];
                        vname = vparts[1];
                    } else {
                        vname = vparts[0];
                        vlname = vparts[0];
                        if (!vlnames && parts.length==1) {
                            if (mod==null) {
                                console.warn("mod null:", mpath);
                            } else if (Object.keys(mod).length==1) {
                                vlname =  Object.keys(mod)[0];
                                vlnames = true;
                            }
                        }
                    }
                    vlname=vlname.trim();
                    vname=vname.trim();
                    if (vname.length==0) {
                        console.warn("__Import vlname empty", mpath, selector)
                        continue;
                    }
                    if (doExport) {
                        vname = "exports['"+vname+"']";
                    }
                    if (vlnames) {
                        toEval+=((doExport?"":"var ")+vname+"="+"__electrico_import.mod['"+vlname+"'];");
                    } else {
                        toEval+=((doExport?"":"var ")+vname+"="+"__electrico_import.mod;");
                    }
                }
            }
            return {mod:mod, toEval:toEval}; 
        }
        window.__importinline=function(__require_this, mpath) {
            return new Promise((resolve, reject) => {
                let mod = loadModule(__require_this, mpath, true);
                if (mod==null) {
                    reject();
                } else {
                    resolve(mod);
                }
            });
        }
        window.require=function(__require_this, mpath) {
            if (mpath==null) {
                mpath = __require_this;
                __require_this=null;
            }
            return loadModule(__require_this, mpath, true);
        }
        window.__replaceImports = (script) => {
            script = ("\n"+script+"\n").replaceAll(/([;,\r,\n])import (.*) from [',"](.*)[',"][;,\r,\n]/g, "$1var __electrico_import=__Import(__require_this, '$2', '$3');eval(__electrico_import.toEval);");
            script = script.replaceAll(/([;,\r,\n])import [',"](.*)[',"][;,\r,\n]/g, ";$1var __electrico_import=__Import(__require_this, null, '$2');eval(__electrico_import.toEval);");
            script = script.replaceAll("import.meta", "__Import_meta");
            script = script.replaceAll("import(", "__importinline(__require_this, ");
            script = script.replaceAll("require(", "require(__require_this, ");
            script = script.replaceAll(/([;,\r,\n])export (.*) from [',"](.*)[',"][;,\r,\n]/g, ";$1var __electrico_import=__Import(__require_this, '$2', '$3', true, exports);eval(__electrico_import.toEval);");
            
            script = script.replaceAll(/\export +{ *([^{ ,;,\n,}}]*) *} *;/g, "exports['$1']=$1;");
            let export_try_deferred = "var $3={}; try {exports['$3']=$3$4;} catch (e) {exports.__electrico_deferred.push(function(){exports['$3']=$3$4;});};";

            script = script.replaceAll(/\export +(var ) *(([^{ ,;,\n}]*))(.*);/g, export_try_deferred);
            script = script.replaceAll(/\export +(let ) *(([^{ ,;,\n}]*))(.*);/g, export_try_deferred);
            script = script.replaceAll(/\export +(const ) *(([^{ ,;,\n}]*))(.*);/g, export_try_deferred);
            script = script.replaceAll(/\export +(default )?(const )?(var )?(let )? *(([^{ ,;,\n}]*))(.*);/g, "exports['$6']=$6$7;");

            script = script.replaceAll(/\export +(default)?(const)? *((async +function)?(function)?(function\*)?(async +function\*)?(class)? +([^{ ,(,;,\n}]*))/g, "exports['$9']=$9=$3");
            script = script.replaceAll('"use strict"', "");
            script = script.replaceAll(/[\r,\n] *}[\r,\n] *\(function/g, "\n};\n(function"); // Color

            let sourcemapspattern = "sourceMappingURL=data:application/json;base64,";
            let smix = script.indexOf(sourcemapspattern);
            if (smix>=0) {
                try {
                    let sourcemaps = JSON.parse(atob(script.substring(smix+sourcemapspattern.length)));
                    if (sourcemaps.sourceRoot!=null && sourcemaps.sourceRoot.startsWith("file://")) {
                        sourcemaps.sourceRoot = window.__create_protocol_url("fil://mod/"+sourcemaps.sourceRoot.substring(7));
                        script = script.substring(0, smix+sourcemapspattern.length)+btoa(JSON.stringify(sourcemaps));
                    }
                } catch (e) {}
            }
            return script;
        }
    };
})();
```

</pre></span></p>


-----------------------

/src/js/shared/shared.js:
-----------------------

<p align="left"><span><pre>

```javascript
(function() {
    function init_shared (window) {
        function getCircularReplacer() {
            const ancestors = [];
            return function (key, value) {
              if (typeof value !== "object" || value === null) {
                return value;
              }
              while (ancestors.length > 0 && ancestors.at(-1) !== this) {
                ancestors.pop();
              }
              if (ancestors.includes(value)) {
                return null;
              }
              ancestors.push(value);
              return value;
            };
        }
        let _stringify = JSON.stringify;
        JSON.stringify = (obj, r) => {
            return _stringify(obj, r!=null?r:getCircularReplacer());
        };
        window.__create_protocol_url = (url) => {
            if (window.__is_windows) {
                let ix = url.indexOf(":");
                url = "http://"+url.substring(0,ix)+"."+url.substring(ix+3);
            }
            return url;
        };
        window.__uuidv4 = function() {
          return "10000000-1000-4000-8000-100000000000".replace(/[018]/g, c =>
              (+c ^ window.crypto.getRandomValues(new Uint8Array(1))[0] & 15 >> +c / 4).toString(16)
          );
        };
        window.__init_require(window);
    }
    window.__init_shared = init_shared;
})();
```

</pre></span></p>


-----------------------

/src/main.rs:
-----------------------

<p align="left"><span><pre>

```rust
mod common;
mod types;
mod ipcchannel;
mod backend;
mod frontend;
mod node;
mod electron;
use std::{collections::HashMap, fs, path::{Path, PathBuf}, str::FromStr, sync::mpsc::{self, Receiver, Sender}, time::{Duration, SystemTime}};
use electron::electron::process_electron_command;
use env_logger::Env;
use json_comments::StripComments;
use muda::{Menu, MenuEvent};
use serde_json::Error;
use backend::Backend;
use frontend::Frontend;
use ipcchannel::{IPCChannel, IPCMsg};
use log::{debug, error, trace, warn};
use node::node::{process_node_command, AppEnv};
use reqwest::StatusCode;
use tao::event_loop::EventLoopBuilder;
use common::{build_file_map, handle_file_request, respond_ok, respond_status, CONTENT_TYPE_HTML, CONTENT_TYPE_JSON, JS_DIR_FRONTEND};
use types::{Command, ElectricoEvents, Package, Resources};
use tao::{event::{Event, StartCause, WindowEvent},event_loop::{ControlFlow, EventLoop}};

fn main() -> wry::Result<()> {
  let env = Env::default()
        .filter_or("LOG_LEVEL", "debug")
        .write_style_or("LOG_STYLE", "always");

  env_logger::init_from_env(env);

  let tokio_runtime = tokio::runtime::Builder::new_multi_thread().worker_threads(30).enable_io().enable_time().build().unwrap();

  let mut rsrc_dir = std::env::current_exe()
    .expect("Can't find path to executable");

  while rsrc_dir.pop() {
    let rsrc_link_dir = rsrc_dir.join("ResourcesLink.json");
    let rsrc_link = Path::new(&rsrc_link_dir);
    if rsrc_link.exists() {
      if let Ok(rsrc_link_str) = fs::read_to_string(rsrc_link) {
        let rsrc_link_json = StripComments::new(rsrc_link_str.as_bytes());
        let res:Result<Resources, Error> = serde_json::from_reader(rsrc_link_json);
        if let Ok(res) = res {
          if let Some(link) = res.link {
            trace!("link {}", link);
            rsrc_dir = PathBuf::from_str(link.as_str()).unwrap();
            break;
          }
        }
      }
    }
    rsrc_dir.push("Resources");
    if rsrc_dir.exists() && rsrc_dir.is_dir() {
      break;
    }
    rsrc_dir.pop();
  }

  let pgk_file = rsrc_dir.join("package.json");
  trace!("package.json path: {}", pgk_file.as_path().as_os_str().to_str().unwrap());
  
  let packagetxt = std::fs::read_to_string(pgk_file).expect("Can't find package.json");
  let package:Package = serde_json::from_str(packagetxt.as_str()).expect("Can't deserialize package.json");
  trace!("package.json main js: {}", package.main.as_str());

  let frontend_js_files = build_file_map(&JS_DIR_FRONTEND);

  let event_loop:EventLoop<ElectricoEvents> = EventLoopBuilder::with_user_event().build();
  let proxy: tao::event_loop::EventLoopProxy<ElectricoEvents> = event_loop.create_proxy();
  
  let mut backend = Backend::new(rsrc_dir.clone(), &package, &event_loop, proxy.clone());
  let mut frontend = Frontend::new(rsrc_dir.clone());
  let mut ipc_channel = IPCChannel::new();
  
  let menu_channel = MenuEvent::receiver();
  
  let mut _main_menu_hold:Option<Menu> = None;
  let mut app_env = AppEnv::new(rsrc_dir.as_os_str().to_str().unwrap().to_string());

  event_loop.run( move |event, event_loop, control_flow| {
    *control_flow = ControlFlow::Wait;
    if let Ok(event) = menu_channel.try_recv() {
      if event.id == "quit" {
        backend.window_close(&None);
        return;
      } else if event.id == "toggleDevTools" {
        frontend.toggle_dev_tools();
        return;
      }
      backend.menu_selected(event.id);
    }
    backend.process_commands();
    match event {
      Event::NewEvents(StartCause::Init) => {
        
      },
      Event::Opened{ urls } => {
        for url in urls{
          app_env.add_arg(url.as_str().to_string());
        }
      }
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        window_id,
        ..
      } => {
        if let Some(w_id) = frontend.get_id(&window_id) {
          backend.window_close(&Some(w_id.to_string()));
        }
      },
      Event::UserEvent(ElectricoEvents::FrontendNavigate{browser_window_id, page, preload}) => {
        
      },
      Event::UserEvent(ElectricoEvents::IPCCallRetry{browser_window_id, request_id, params, sender}) => {
        backend.call_ipc_channel(&browser_window_id, &request_id, params, sender);
      }
      Event::UserEvent(ElectricoEvents::ExecuteCommand{command, responder, data_blob}) => {
        trace!("backend ExecuteCommand call");
        match command {
          Command::PostIPC { browser_window_id, request_id, params} => {
            trace!("PostIPC {} {} {}", browser_window_id, request_id, params);
            let r_request_id = request_id.clone();
            
            let (sender, receiver): (Sender<IPCMsg>, Receiver<IPCMsg>) = mpsc::channel();
            ipc_channel.start(request_id.clone(), sender.clone());

            let callipc_proxy = proxy.clone();
            let callipc_browser_window_id = browser_window_id.clone();
            let callipc_request_id = request_id.clone();
            let callipc_params = params.clone();
            let callipc_sender = sender.clone();
            let started = SystemTime::now();
            let mut called = false;
            tokio_runtime.spawn(
              async move {
                while !called {
                  let called_response = receiver.recv_timeout(Duration::from_millis(100));
                  match called_response {
                    Ok (response) => {
                      match response {
                        IPCMsg::Called => {
                          called = true;
                        },
                        IPCMsg::Response { params } => {
                          respond_status(StatusCode::OK, CONTENT_TYPE_JSON.to_string(), params.to_string().into_bytes(), responder);
                          return;
                        }
                      }
                    },
                    Err (_e) => {
                      match started.elapsed() {
                        Ok(elapsed) => {
                            if elapsed.as_secs()>600 {
                                warn!("PostIPC Call Expired {}", callipc_request_id.clone());
                                respond_status(StatusCode::GONE, CONTENT_TYPE_JSON.to_string(), "call expired (timeout)".to_string().into_bytes(), responder);
                                return;
                            }
                        },
                        Err(e) => {
                            error!("PostIPC SystemTimeError {}", e.to_string());
                        }
                      }
                      let _ = callipc_proxy.send_event(
                        ElectricoEvents::IPCCallRetry { browser_window_id:callipc_browser_window_id.clone(), request_id:callipc_request_id.clone(), params:callipc_params.clone(), sender:callipc_sender.clone() }
                      );
                    }
                  }
                }
                let ipc_response = receiver.recv_timeout(Duration::from_secs(600));
                match ipc_response {
                  Ok (response) => {
                    match response {
                      IPCMsg::Response { params } => {
                        trace!("PostIPC Response {}", params);
                        respond_status(StatusCode::OK, CONTENT_TYPE_JSON.to_string(), params.to_string().into_bytes(), responder);
                      },
                      _ => ()
                    }
                  },
                  Err (_e) => {
                    warn!("PostIPC request expired (timeout): {}", r_request_id.clone());
                    respond_status(StatusCode::GONE, CONTENT_TYPE_JSON.to_string(), "expired (timeout)".to_string().into_bytes(), responder);
                  }
                }
              }
            );
            backend.call_ipc_channel(&browser_window_id, &request_id, params, sender);
          },
          Command::SetIPCResponse {request_id, params} => {
            trace!("backend ExecuteCommand call SetIPCResponse {} {}", request_id, params);
            match ipc_channel.get(&request_id) {
              Some(sender) => {
                let _ = sender.send(IPCMsg::Response { params });
                ipc_channel.end(&request_id);
              },
              None => {
                warn!("ipc_channel - backend ExecuteCommand call SetIPCResponse request expired (timeout): {}", request_id);
              }
            }
            
            respond_ok(responder);
          },
          Command::BrowserWindowReadFile { browser_window_id, file_path, module } => {
              trace!("BrowserWindowReadFile {} {}", browser_window_id, file_path);
              match frontend.get_client_path_base(&browser_window_id) {
                  Some(client_path_base) => {
                    let file = rsrc_dir.join(file_path.clone());
                    if !file.starts_with(client_path_base) {
                        error!("browser client access to file forbidden: {} {}", file.as_os_str().to_str().unwrap(), client_path_base.as_os_str().to_str().unwrap());
                        respond_status(StatusCode::FORBIDDEN, CONTENT_TYPE_HTML.to_string(), "forbidden".to_string().into_bytes(), responder);
                        return;
                    }  
                    handle_file_request(&tokio_runtime, module, file_path, file, &frontend_js_files, responder);
                  },
                  None => {
                      error!("browser client access to file forbidden - no client_path_base: {}", file_path);
                      respond_status(StatusCode::FORBIDDEN, CONTENT_TYPE_HTML.to_string(), "forbidden".to_string().into_bytes(), responder);
                  }
              }
          },
          Command::DOMContentLoaded { browser_window_id, title} => {
            backend.dom_content_loaded(&browser_window_id);
            frontend.dom_content_loaded(&browser_window_id, title);
          }
          Command::Electron { invoke } => {
            let menu_ret = process_electron_command(&tokio_runtime, event_loop, proxy.clone(), 
              &mut app_env, &rsrc_dir, &package,
              &mut frontend, &mut backend, invoke, responder);
            if let Some(menu) = menu_ret.clone() {
              _main_menu_hold = menu_ret;
            }
          }
          Command::Node { invoke } => {
            process_node_command(&tokio_runtime, &app_env, proxy.clone(), &mut backend, invoke, responder, data_blob);
          }
        }
      },
      Event::UserEvent(ElectricoEvents::SendChannelMessageRetry { browser_window_id, channel, args }) => {
        trace!("SendChannelMessageRetry");
        frontend.send_channel_message(proxy.clone(), browser_window_id, channel, args);
      },
      Event::UserEvent(ElectricoEvents::Exit) => {
        backend.shutdown();
        *control_flow = ControlFlow::Exit;
      },
      _ => (),
    }
  });
}
```

</pre></span></p>


-----------------------

/src/node/common.rs:
-----------------------

<p align="left"><span><pre>

```rust
use tao::event_loop::EventLoopProxy;
use std::sync::mpsc::Sender;

use crate::types::{BackendCommand, ElectricoEvents};

pub fn send_command(proxy:&EventLoopProxy<ElectricoEvents>, command_sender:&Sender<BackendCommand>, command:BackendCommand) {
    let _ = command_sender.send(command);
    let _ = proxy.send_event(ElectricoEvents::Noop);
}
```

</pre></span></p>


-----------------------

/src/node/ipc.rs:
-----------------------

<p align="left"><span><pre>

```rust
use std::{fs, path::Path, sync::mpsc::{self, Receiver, Sender}, time::Duration};

use interprocess::local_socket::{ToFsName, traits::tokio::{Listener, Stream}, GenericFilePath, ListenerOptions};
use reqwest::StatusCode;
use tao::event_loop::EventLoopProxy;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, runtime::Runtime, time::{timeout, sleep}};
use log::{debug, error, trace};
use uuid::Uuid;
use wry::RequestAsyncResponder;

use crate::{common::{respond_404, respond_ok, respond_status, CONTENT_TYPE_TEXT}, node::common::send_command, types::{BackendCommand, ElectricoEvents, NETConnection, NETServer}};

pub fn ipc_server(
        hook:String, 
        tokio_runtime:&Runtime, 
        proxy: EventLoopProxy<ElectricoEvents>, 
        command_sender: Sender<BackendCommand>,
        responder:RequestAsyncResponder) {
    if let Ok(name) = hook.clone().to_fs_name::<GenericFilePath>() {
        let lo = ListenerOptions::new().name(name);
        let s_hook = hook.clone();
        let s_proxy = proxy.clone();
        let s_command_sender = command_sender.clone();

        #[cfg(unix)] {
            if Path::new(hook.as_str()).exists() {
                trace!("removing socket file {}", hook);
                let _ = fs::remove_file(hook);
            }
        }
        tokio_runtime.spawn(async move {
           match lo.create_tokio() {
                Ok(l) => {
                    let id = Uuid::new_v4().to_string();
                    let (sender, receiver): (Sender<NETServer>, Receiver<NETServer>) = mpsc::channel();
                    let _ = send_command(&s_proxy, &s_command_sender, BackendCommand::NETServerStart { id: id.clone(), sender:sender });
                    respond_status(StatusCode::OK, CONTENT_TYPE_TEXT.to_string(), id.into_bytes(), responder);
                    loop {
                        match timeout(Duration::from_secs(5), l.accept()).await {
                            Ok(rc) => {
                                match rc {
                                    Ok(c) => {
                                        let id = Uuid::new_v4().to_string();
                                        trace!("ipc listener connection start id {}", id);
                                        let (sender, receiver): (Sender<NETConnection>, Receiver<NETConnection>) = mpsc::channel();
                                        let _ = send_command(&s_proxy, &s_command_sender, BackendCommand::NETServerConnStart { hook: s_hook.clone(), id:id.clone(), sender:sender});
                                        ipc_connect(&id, c, receiver, proxy.clone(), command_sender.clone());
                                    },
                                    Err(e) => {
                                        error!("ipc listener error {}", e);
                                    }
                                }
                            },
                            Err(_t) => {
                                if let Ok(c) = receiver.try_recv() {
                                    match c {
                                        NETServer::Close => {
                                            trace!("NETServer close");
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    trace!("NETServer closed");
                },
                Err(e) => {
                    error!("NETCreateServer Error {}", e);
                    respond_status(StatusCode::INTERNAL_SERVER_ERROR, CONTENT_TYPE_TEXT.to_string(), format!("Error: {}", e).to_string().into_bytes(), responder); 
                }
            }
        });
    } else {
        respond_404(responder);
    }
}

pub fn ipc_connection(
        hook:String,
        id:String,
        tokio_runtime:&Runtime, 
        proxy: EventLoopProxy<ElectricoEvents>, 
        command_sender: Sender<BackendCommand>,
        responder:RequestAsyncResponder) {
    if let Ok(name) = hook.clone().to_fs_name::<GenericFilePath>() {
        let c_proxy = proxy.clone();
        let c_command_sender = command_sender.clone();
        let (sender, receiver): (Sender<NETConnection>, Receiver<NETConnection>) = mpsc::channel();
        let _ = send_command(&c_proxy, &c_command_sender, BackendCommand::NETClientConnStart { id:id.clone(), sender:sender});
        tokio_runtime.spawn(async move {
            match interprocess::local_socket::tokio::Stream::connect(name).await {
                Ok(c) => {
                    respond_ok(responder);
                    ipc_connect(&id, c, receiver, proxy, command_sender);
                },
                Err(e) => {
                    respond_status(StatusCode::INTERNAL_SERVER_ERROR, CONTENT_TYPE_TEXT.to_string(), format!("Error: {}", e).to_string().into_bytes(), responder);
                }
            }
        });
    } else {
        respond_404(responder);
    }
}

fn ipc_connect(id:&String, c:interprocess::local_socket::tokio::Stream, 
            receiver:Receiver<NETConnection>,
            proxy: EventLoopProxy<ElectricoEvents>,
            command_sender: Sender<BackendCommand>) {

    let r_proxy = proxy.clone();
    let r_command_sender = command_sender.clone();
    let r_id=id.clone();

    let (mut reader, mut writer) = c.split();
    let w_proxy = proxy.clone();
    let w_command_sender = command_sender.clone();
    let w_id=id.clone();
    let (timeout_sender, timeout_receiver): (Sender<bool>, Receiver<bool>) = mpsc::channel();
                                        
    tokio::spawn(async move {
        loop {
            trace!("NETConnection::write loop {}", w_id);
            match receiver.recv_timeout(Duration::from_secs(300)) {
                Ok(r) => {
                    match r {
                        NETConnection::Write { data } => {
                            trace!("NETConnection::Write {}", w_id);
                            let _ = writer.write(&data.to_vec()).await;
                        },
                        NETConnection::Disconnect => {
                            trace!("NETConnection::Disconnect {}", w_id);
                            break;
                        },
                        NETConnection::EndConnection => {
                            trace!("NETConnection::EndConnection {}", w_id);
                            let _ = send_command(&w_proxy, &w_command_sender, BackendCommand::NETConnectionEnd { id:w_id.clone() });
                        }
                    }
                },
                Err(_e) => {
                    trace!("NETConnection::receive_timeout {}", w_id);
                    let _ = timeout_sender.send(true);
                }
            }
        }
        trace!("NETConnection write end {}", w_id);
    });
    let mut buffer:Vec<u8> = vec![0; 1024];
    tokio::spawn(async move {
        loop {
            trace!("NETConnection::read loop {}", r_id);
            match timeout(Duration::from_secs(300),  reader.read(&mut buffer)).await {
                Ok(r) => {
                    match r {
                        Ok(read) => {
                            trace!("send data for NETConnection read {}", r_id);
                            if read>0 {
                                trace!("send data for NETConnection stream {}", r_id);
                                let _ = send_command(&r_proxy, &r_command_sender, BackendCommand::NETConnectionData {id:r_id.clone(), data: Some(buffer[0..read].to_vec()) });
                            } else {
                                trace!("NETConnection stream end {}", r_id);
                                sleep(Duration::from_millis(100)).await;
                                let _ = send_command(&r_proxy, &r_command_sender, BackendCommand::NETConnectionEnd { id:r_id.clone() });
                                break;
                            }
                        },
                        Err(e) => {
                            error!("ipc NETConnection stream read error {}", e);
                        }
                    }
                },
                Err(_e) => {
                    trace!("NETConnection::read_timeout {}", r_id);
                    if let Ok(_t) = timeout_receiver.try_recv() {
                        trace!("read and write timeout");
                        let _ = send_command(&r_proxy, &r_command_sender, BackendCommand::NETConnectionEnd { id:r_id.clone() });
                        break;
                    }
                }
            }
            if let Ok(_t) = timeout_receiver.try_recv() {
                trace!("write timeout, but no read timeout")
            }
        }
        trace!("NETConnection read end {}", r_id);
    });
}
```

</pre></span></p>

-----------------------

/src/node/mod.rs:
-----------------------


pub mod node;
pub mod process;
pub mod ipc;
pub mod common;
pub mod types;


-----------------------

/src/node/node.rs:
-----------------------

<p align="left"><span><pre>

```rust
use std::{fs::{self, OpenOptions}, io::{Read, Seek, SeekFrom, Write}, path::Path, time::SystemTime};
use log::{debug, error, info, trace, warn};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use reqwest::{header::{ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE}, Method, Request, StatusCode, Url};
use tao::event_loop::EventLoopProxy;
use tokio::runtime::Runtime;
use wry::{http::Response, webview_version, RequestAsyncResponder};

use crate::{backend::Backend, common::{respond_404, respond_client_error, respond_ok, respond_status, CONTENT_TYPE_BIN, CONTENT_TYPE_JSON, CONTENT_TYPE_TEXT}, node::{common::send_command, ipc::{ipc_connection, ipc_server}, types::{FSDirent, Process, ProcessEnv, ProcessVersions}}, types::{BackendCommand, ElectricoEvents}};
use super::{process::child_process_spawn, types::{ConsoleLogLevel, FSStat, NodeCommand}};

pub struct AppEnv {
    pub start_args: Vec<String>,
    pub app_name:Option<String>,
    pub resources_path:String
}

impl AppEnv {
    pub fn new(resources_path:String) -> AppEnv {
        let mut args = Vec::new();
        for arg in std::env::args() {
            args.push(arg);
        }
        AppEnv {
            start_args:args,
            app_name:None,
            resources_path:resources_path
        }
    }
    pub fn add_arg(&mut self, arg:String) {
        self.start_args.push(arg);
    }
}

pub fn process_node_command(tokio_runtime:&Runtime, app_env:&AppEnv,
        proxy:EventLoopProxy<ElectricoEvents>,
        backend:&mut Backend,
        command:NodeCommand,
        responder:RequestAsyncResponder,
        data_blob:Option<Vec<u8>>)  {
    let command_sender = backend.command_sender();
    match command {
        NodeCommand::ConsoleLog { params } => {
            match params.level {
                ConsoleLogLevel::Info => {
                    info!("{} {}", params.logmsg, params.logdata.or(Option::Some("".to_string())).unwrap());
                },
                ConsoleLogLevel::Debug => {
                    debug!("{} {}", params.logmsg, params.logdata.or(Option::Some("".to_string())).unwrap());
                },
                ConsoleLogLevel::Warn => { 
                    warn!("{} {}", params.logmsg, params.logdata.or(Option::Some("".to_string())).unwrap());
                },
                ConsoleLogLevel::Error => {
                    error!("{} {}", params.logmsg, params.logdata.or(Option::Some("".to_string())).unwrap());
                },
                ConsoleLogLevel::Trace => {
                    trace!("{} {}", params.logmsg, params.logdata.or(Option::Some("".to_string())).unwrap());
                }
            }
            respond_ok(responder);
        },
        NodeCommand::GetProcessInfo => {
            trace!("node process_info");
            const ELECTRICO_VERSION: &str = env!("CARGO_PKG_VERSION");
            let webview_version:String = webview_version().expect("webview_version failed");

            let chrome = format!("WebView-{}", webview_version);
            let node = format!("Electrico-{}/{}", ELECTRICO_VERSION, webview_version);
            let electron = format!("Electrico-{}", ELECTRICO_VERSION);

            let mut platform = "win32";
            #[cfg(target_os = "macos")] {
                platform = "darwin";
            }
            #[cfg(target_os = "linux")] {
                platform = "linux";
            }
            let mut node_env="production".to_string();
            let mut electron_is_dev="0".to_string();

            let mut home = "".to_string();
            if let Some(user_dirs) = directories::UserDirs::new() {
                home = user_dirs.home_dir().as_os_str().to_str().unwrap().to_string();
            }

            #[cfg(debug_assertions)] {
                node_env="development".to_string();
                electron_is_dev="1".to_string();
            }

            let process_info = Process::new(platform.to_string(), 
                ProcessVersions::new(node, chrome, electron), 
                ProcessEnv::new(node_env, electron_is_dev, home),
                app_env.resources_path.clone());
            match serde_json::to_string(&process_info) {
                Ok(json) => {
                    respond_status(StatusCode::OK, CONTENT_TYPE_JSON.to_string(), json.into_bytes(), responder);
                },
                Err(e) => {
                    respond_status(StatusCode::INTERNAL_SERVER_ERROR, CONTENT_TYPE_TEXT.to_string(), format!("GetProcessInfo json serialization error: {}", e).into_bytes(), responder);
                }
            }
        },
        NodeCommand::GetStartArgs => {
            trace!("node GetStartArgs");
            match serde_json::to_string(&app_env.start_args) {
                Ok(json) => {
                    respond_status(StatusCode::OK, CONTENT_TYPE_JSON.to_string(), json.into_bytes(), responder);
                },
                Err(e) => {
                    respond_status(StatusCode::INTERNAL_SERVER_ERROR, CONTENT_TYPE_TEXT.to_string(), format!("GetStartArgs json serialization error: {}", e).into_bytes(), responder);
                }
            }
        },
        NodeCommand::FSAccess { path, mode } => {
            if mode & 1 !=0 && !Path::new(&path).exists() {
                responder.respond(Response::builder().header(ACCESS_CONTROL_ALLOW_ORIGIN, "*").header(CONTENT_TYPE, CONTENT_TYPE_TEXT).body(Vec::from("NOK".to_string().as_bytes())).unwrap());
                return;
            }
            match fs::metadata(path.as_str()) {
                Ok (meta) => {
                    if mode & 4 !=0 && meta.permissions().readonly() {
                        responder.respond(Response::builder().header(ACCESS_CONTROL_ALLOW_ORIGIN, "*").header(CONTENT_TYPE, CONTENT_TYPE_TEXT).body(Vec::from("NOK".to_string().as_bytes())).unwrap());
                        return;
                    }
                    respond_ok(responder);
                },
                Err (e) => {
                    error!("FSAccess error: {}", e);
                    respond_404(responder);
                }
            }
        },
        NodeCommand::FSLstat { path} => {
            let p = Path::new(&path);
            if !p.exists() {
                respond_404(responder);
            } else {
                let mut created:Option<SystemTime>=None;
                let mut modified:Option<SystemTime>=None;
                if let Ok(meta) = p.metadata() {
                    if let Ok(c) = meta.created() {
                        created = Some(c);
                    }
                    if let Ok(m) = meta.modified() {
                        modified = Some(m);
                    }
                }
                let stat = FSStat::new(p.is_dir(), created, modified);
                match serde_json::to_string(&stat) {
                    Ok(json) => {
                        respond_status(StatusCode::OK, CONTENT_TYPE_JSON.to_string(), json.into_bytes(), responder);
                    },
                    Err(e) => {
                        respond_status(StatusCode::INTERNAL_SERVER_ERROR, CONTENT_TYPE_TEXT.to_string(), format!("FSLstat json serialization error: {}", e).into_bytes(), responder);
                    }
                }
            }
        },
        NodeCommand::FSMkdir { path, options } => {
            if Path::new(&path).exists() {
                trace!("FSMkdir path exists {}", path);
                respond_status(StatusCode::OK, CONTENT_TYPE_TEXT.to_string(), path.into_bytes(), responder);
                return;
            }
            if let Some(options) = options {
                if let Some (recursive) = options.recursive {
                    if recursive {
                        match fs::create_dir_all(path.as_str()) {
                            Ok (_) => {
                                respond_status(StatusCode::OK, CONTENT_TYPE_TEXT.to_string(), path.into_bytes(), responder);
                                return;
                            },
                            Err (e) => {
                                error!("FSMkdir create_dir_all error: {} {}", path.as_str(), e);
                                respond_404(responder);
                                return;
                            }
                        }
                    }
                }
            }
            match fs::create_dir(path.as_str()) {
                Ok (_) => {
                    respond_status(StatusCode::OK, CONTENT_TYPE_TEXT.to_string(), path.into_bytes(), responder);
                return;
                },
                Err (e) => {
                    error!("FSMkdir create_dir error: {} {}", path.as_str(), e);
                    respond_404(responder);
                return;
                }
            }
        },
        NodeCommand::FSReadDir { path, options } => {
            let mut recursive = false;
            if let Some(options) = options {
                if let Some(rec) = options.recursive {
                    recursive=rec;
                }
            }
            let mut entries:Vec<FSDirent> = Vec::new();
            fn read_dir(path:String, entries:&mut Vec<FSDirent>, recursive:bool) -> Option<std::io::Error> {
                match fs::read_dir(path) {
                    Ok(rd) => {
                        for e in rd {
                            if let Ok(e) = e {
                                let path = e.path().as_os_str().to_str().unwrap().to_string();
                                if recursive && e.path().is_dir() {
                                    if let Some(error) = read_dir(path.clone(), entries, recursive) {
                                        return Some(error);
                                    }
                                }
                                entries.push(FSDirent::new(path, e.path().is_dir()));
                            }
                        }
                        return None;
                    },
                    Err(e) => {
                        error!("FSReadDir error {}", e);
                        return Some(e);
                    }
                }
            }
            if let Some(error) = read_dir(path, &mut entries, recursive) {
                respond_status(StatusCode::BAD_REQUEST, CONTENT_TYPE_TEXT.to_string(), format!("FSReadDir error: {}", error).into_bytes(), responder);
                return;
            }
            match serde_json::to_string(&entries) {
                Ok(json) => {
                    respond_status(StatusCode::OK, CONTENT_TYPE_JSON.to_string(), json.into_bytes(), responder);
                },
                Err(e) => {
                    respond_status(StatusCode::INTERNAL_SERVER_ERROR, CONTENT_TYPE_TEXT.to_string(), format!("FSReadDir json serialization error: {}", e).into_bytes(), responder);
                }
            }
        },
        NodeCommand::FSReadFile { path, options } => {
            match fs::read(path.as_str()) {
                Ok (contents) => {
                    respond_status(StatusCode::OK, mime_guess::from_path(path).first_or_octet_stream().to_string(), contents, responder);
                },
                Err (e) => {
                    error!("FSReadFile error: {} - {}", path, e);
                    respond_status(StatusCode::BAD_REQUEST, CONTENT_TYPE_TEXT.to_string(), format!("FSReadFile error: {}", e).into_bytes(), responder);
                }
            }
        },
        NodeCommand::FSWriteFile { path, options } => {
            if let Some(data) = data_blob {
                if let Some(options) = options {
                    if let Some(_encoding) = options.encoding {
                        match fs::write(path.as_str(), data) {
                            Ok(_) => {
                                respond_ok(responder);
                            },
                            Err (e) => {
                                error!("FSWriteFile error: {}", e);
                                respond_status(StatusCode::BAD_REQUEST, CONTENT_TYPE_TEXT.to_string(), format!("FSWriteFile error: {}", e).into_bytes(), responder);
                            }
                        }
                        return;
                    }
                }
                match fs::write(path.as_str(), data) {
                    Ok(_) => {
                        respond_ok(responder);
                    },
                    Err (e) => {
                        error!("FSWriteFile error: {}", e);
                        respond_status(StatusCode::BAD_REQUEST, CONTENT_TYPE_TEXT.to_string(), format!("FSWriteFile error: {}", e).into_bytes(), responder);
                    }
                }
            } else {
                error!("FSWrite error, no data");
                respond_status(StatusCode::INTERNAL_SERVER_ERROR, CONTENT_TYPE_TEXT.to_string(), format!("FSWriteFile error, no data").into_bytes(), responder);
            }
        },
        NodeCommand::FSOpen {fd, path, flags, mode } => {
            let write = flags.contains("w") || flags.contains("a");
            match OpenOptions::new().read(true).write(write).create(write).truncate(flags.contains("w")).open(path) {
                Ok(mut file) => {
                    if flags.contains("a") {
                        let _ = file.seek(SeekFrom::End(0));
                    }
                    backend.fs_open(fd, file);
                    respond_status(StatusCode::OK, CONTENT_TYPE_TEXT.to_string(), fd.to_string().into_bytes(), responder); 
                },
                Err(e) => {
                    respond_status(StatusCode::BAD_REQUEST, CONTENT_TYPE_TEXT.to_string(), format!("FSOpen error: {}", e).into_bytes(), responder);
                }
            }
        },
        NodeCommand::FSClose { fd } => {
            backend.fs_close(fd);
            respond_ok(responder);
        },
        NodeCommand::FSRead { fd, offset, length, position } => {
            trace!("FSRead {}, {}, {}, {:?}", fd, offset, length, position);
            if let Some(mut file) = backend.fs_get(fd) {
                if let Some(position) = position {
                    let _ = file.seek(SeekFrom::Start(position));
                }
                let _ = file.seek(SeekFrom::Current(offset));
                let mut buf = vec![0; length];
                match file.read(&mut buf) {
                    Ok(read) => {
                        respond_status(StatusCode::OK, CONTENT_TYPE_BIN.to_string(), buf[0..read].to_vec(), responder);
                    },
                    Err(e) => {
                        error!("FSRead error: {}", e);
                        respond_status(StatusCode::INTERNAL_SERVER_ERROR, CONTENT_TYPE_TEXT.to_string(), format!("FSRead error: {}", e).into_bytes(), responder);
                    }
                }
            } else {
                respond_404(responder);
            }
        },
        NodeCommand::FSWrite { fd, offset, length, position } => {
            trace!("FSWrite {}, {}, {}, {:?}", fd, offset, length, position);
            if let Some(mut file) = backend.fs_get(fd) {
                if let Some(position) = position {
                    let _ = file.seek(SeekFrom::Start(position));
                }
                let _ = file.seek(SeekFrom::Current(offset));
                if let Some(mut data) = data_blob {
                    match file.write(&mut data) {
                        Ok(written) => {
                            respond_status(StatusCode::OK, CONTENT_TYPE_TEXT.to_string(), written.to_string().as_bytes().to_vec(), responder);
                        },
                        Err(e) => {
                            error!("FSWrite error: {}", e);
                            respond_status(StatusCode::INTERNAL_SERVER_ERROR, CONTENT_TYPE_TEXT.to_string(), format!("FSWrite error: {}", e).into_bytes(), responder);
                        }
                    }
                } else {
                    error!("FSWrite error, no data");
                    respond_status(StatusCode::INTERNAL_SERVER_ERROR, CONTENT_TYPE_TEXT.to_string(), format!("FSWrite error, no data").into_bytes(), responder);
                }
                
            } else {
                respond_404(responder);
            }
        },
        NodeCommand::FSRealPath { path } => {
            let rp = Path::new(path.as_str()).as_os_str().to_str().unwrap().to_string();
            respond_status(StatusCode::OK, CONTENT_TYPE_TEXT.to_string(), rp.as_bytes().to_vec(), responder);
        },
        NodeCommand::FSFdatasync { fd } => {
            if let Some(file) = backend.fs_get(fd) {
                let _ = file.sync_all();
                respond_ok(responder);
            } else {
                respond_404(responder);
            }
        },
        NodeCommand::HTTPRequest { options } => {
            tokio_runtime.spawn(
                async move {
                    let url = "https://".to_string()+options.hostname.as_str()+":"+options.port.to_string().as_str()+options.path.as_str();
                    
                    let mut headers = reqwest::header::HeaderMap::new();
                    headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36"));
                    headers.insert(reqwest::header::ACCEPT, reqwest::header::HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
                    if let Ok(client) = reqwest::Client::builder().timeout(std::time::Duration::from_secs(30)).default_headers(headers).build() {
                        let method:Method;
                        match Method::try_from(options.method.as_str()) {
                            Ok(m) => {
                                method=m;
                            },
                            Err(_e) => {
                                respond_client_error(format!("invalid method {}", options.method), responder);
                                return;
                            }
                        }
                        let rurl:Url;
                        match Url::parse(url.as_str()) {
                            Ok(r) => {
                                rurl=r;
                            },
                            Err(_e) => {
                                respond_client_error(format!("invalid url {}", url), responder);
                                return;
                            }
                        }

                        match client.execute(Request::new(method, rurl)).await {
                            Ok(response) => {
                                let headers = response.headers().clone();
                                match response.bytes().await {
                                    Ok(body) => {
                                        let mut rbuilder = Response::builder()
                                            .status(StatusCode::OK)
                                            .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*");
                                        for h in headers {
                                            if let Some(hname) = h.0 {
                                                rbuilder = rbuilder.header(hname, h.1);
                                            }
                                        }
                                        responder.respond(rbuilder.body(Vec::from(body)).unwrap());
                                    }, 
                                    Err(e) => {
                                        respond_client_error(format!("could not read response {}", e), responder);
                                    }
                                }
                            },
                            Err(e) => {
                                respond_client_error(format!("could not send request {}", e), responder);
                            }
                        }
                    }
                }
            );
        },
        NodeCommand::ChildProcessSpawn { cmd, args } => {
            child_process_spawn(cmd, args, backend, tokio_runtime, proxy, command_sender, responder);
        },
        NodeCommand::ChildProcessStdinWrite { pid } => {
            backend.child_process_callback(pid, "stdin".to_string(), data_blob);
            respond_ok(responder);
        },
        NodeCommand::ChildProcessDisconnect { pid } => {
            backend.child_process_disconnect(pid);
            respond_ok(responder);
        },
        NodeCommand::FSWatch { path, wid, options } => {
            let mut mode = RecursiveMode::NonRecursive;
            if let Some(options) = options {
                if let Some(rec) = options.recursive {
                    if rec {
                        mode=RecursiveMode::Recursive;
                    }
                }
            }
            let w_proxy = proxy.clone();
            let w_command_sender = command_sender.clone();
            let w_wid = wid.clone();
            match RecommendedWatcher::new(
                move |res| {
                    if let Ok(event) = res {
                        trace!("fswatch receive event {:?}", event);
                        let _ = send_command(&w_proxy, &w_command_sender, BackendCommand::FSWatchEvent { wid: w_wid.clone(), event: event });
                    }
                },
                Config::default()
            ) {
                Ok(mut watcher) => {
                    respond_ok(responder);
                    let _ = watcher.watch(path.as_ref(), mode);
                    backend.watch_start(wid, watcher);                  
                },
                Err(e) => {
                    respond_status(StatusCode::INTERNAL_SERVER_ERROR, CONTENT_TYPE_TEXT.to_string(), format!("Error: {}", e).to_string().into_bytes(), responder); 
                }
            }
        },
        NodeCommand::FSWatchClose { wid } => {
            backend.watch_stop(wid);
            respond_ok(responder);
        },
        NodeCommand::NETCreateServer {hook, options } => {
            trace!("NETCreateServer {}", hook);
            ipc_server(hook, tokio_runtime, proxy, command_sender, responder);
        },
        NodeCommand::NETCloseServer { id } => {
            backend.net_server_close(id);
            respond_ok(responder);
        },
        NodeCommand::NETCloseConnection { id } => {
            backend.net_connection_close(id);
            respond_ok(responder);
        },
        NodeCommand::NETCreateConnection { hook, id } => {
            trace!("NETCreateConnection {}, {}", hook, id);
            ipc_connection(hook, id, tokio_runtime, proxy, command_sender, responder);
        },
        NodeCommand::NETWriteConnection { id } => {
            trace!("NETWriteConnection {}", id);
            if let Some(data) = data_blob {
                backend.net_write_connection(id, data);
                respond_ok(responder);
            } else {
                error!("NETWriteConnection error, no data");
                respond_status(StatusCode::INTERNAL_SERVER_ERROR, CONTENT_TYPE_TEXT.to_string(), format!("NETWriteConnection error, no data").into_bytes(), responder);
            }
        },
        NodeCommand::GetDataBlob { id } => {
            if let Some(data) = backend.get_data_blob(id) {
                respond_status(StatusCode::OK, CONTENT_TYPE_BIN.to_string(), data, responder);
            } else {
                respond_404(responder);
            }
        }
    }
}
```

</pre></span></p>


-----------------------

/src/node/process.rs:
-----------------------

<p align="left"><span><pre>

```rust
use std::{process::{Command, Stdio}, sync::mpsc::{self, Sender, Receiver}, thread};

use log::{error, trace};
use reqwest::StatusCode;
use tao::event_loop::EventLoopProxy;
use tokio::runtime::Runtime;
use wry::RequestAsyncResponder;
use std::io::{Read, Write};

use crate::{backend::Backend, common::{respond_client_error, respond_status, CONTENT_TYPE_TEXT}, node::common::send_command, types::{BackendCommand, ChildProcess, ElectricoEvents}};

pub fn child_process_spawn(
        cmd:String, 
        args:Option<Vec<String>>,
        backend:&mut Backend,
        tokio_runtime:&Runtime,
        proxy: EventLoopProxy<ElectricoEvents>, 
        command_sender: Sender<BackendCommand>,
        responder:RequestAsyncResponder) {
    let mut pargs:Vec<String> = Vec::new();
    if let Some(args) = args {
        pargs = args;
    }
    match Command::new(cmd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .args(pargs).spawn() {
        Ok(mut child) => {
            let (sender, receiver): (Sender<ChildProcess>, Receiver<ChildProcess>) = mpsc::channel();
            backend.child_process_start(child.id().to_string(), sender);
            respond_status(StatusCode::OK, CONTENT_TYPE_TEXT.to_string(), child.id().to_string().into_bytes(), responder); 
            tokio_runtime.spawn(
                async move {
                    let mut stdout;
                    let mut stderr;
                    let mut stdin;
                    match child.stdout.take() {
                        Some(chstdout) => {
                            stdout=chstdout;
                        }, 
                        None => {
                            error!("ChildProcessSpawn stdout not available");
                            let _ = send_command(&proxy, &command_sender, BackendCommand::ChildProcessExit {pid:child.id().to_string(), exit_code:None});
                            return;
                        }
                    }
                    match child.stderr.take() {
                        Some(chstderr) => {
                            stderr=chstderr;
                        }, 
                        None => {
                            error!("ChildProcessSpawn stderr not available");
                            let _ = send_command(&proxy, &command_sender, BackendCommand::ChildProcessExit {pid:child.id().to_string(), exit_code:None});
                            return;
                        }
                    }
                    match child.stdin.take() {
                        Some(chstdin) => {
                            stdin=chstdin;
                        }, 
                        None => {
                            error!("ChildProcessSpawn stdid not available");
                            let _ = send_command(&proxy, &command_sender, BackendCommand::ChildProcessExit {pid:child.id().to_string(), exit_code:None});
                            return;
                        }
                    }
                    let mut exit_code:Option<i32> = None;
                    loop {
                        if let Ok(cp) = receiver.try_recv() {
                            match cp {
                                ChildProcess::StdinWrite { data } => {
                                    trace!("writing stdin {}", data.len());
                                    let _ = stdin.write(data.as_slice());
                                },
                                ChildProcess::Disconnect => {
                                    trace!("disconnect");
                                    break;
                                }
                            }
                        }
                        let mut stdinread:usize=0;
                        let mut stderrread:usize=0;
                        let stdout_buf:&mut [u8] = &mut [0; 1024];
                        if let Ok(read) = stdout.read(stdout_buf) {
                            trace!("stdout read {}", read);
                            stdinread = read;
                            if read>0 {
                                let data:Vec<u8> = stdout_buf[0..read].to_vec();
                                let _ = send_command(&proxy, &command_sender, BackendCommand::ChildProcessCallback { pid:child.id().to_string(), stream:"stdout".to_string(), data:Some(data) });
                            }
                        }
                        let stderr_buf:&mut [u8] = &mut [0; 1024];
                        if let Ok(read) = stderr.read(stderr_buf) {
                            trace!("stderr read {}", read);
                            stderrread = read;
                            if read>0 {
                                let data:Vec<u8> = stderr_buf[0..read].to_vec();
                                let _ = send_command(&proxy, &command_sender, BackendCommand::ChildProcessCallback { pid:child.id().to_string(), stream:"stderr".to_string(), data:Some(data) });
                            }
                        }
                        match child.try_wait() {
                            Ok(event) => {
                                if let Some(event) = event {
                                    exit_code = event.code();
                                }
                            }
                            Err(e) => {
                                error!("ChildProcessSpawn try_wait error: {}", e);
                                break;
                            }
                        }
                        if let Some(_exit_code) = exit_code {
                            if stdinread==0 && stderrread==0 {
                                break;
                            }
                        }
                        thread::yield_now();
                    }
                    let _ = send_command(&proxy, &command_sender, BackendCommand::ChildProcessExit {pid:child.id().to_string(), exit_code:exit_code});
                }
            );         
        },
        Err(e) => {
            respond_client_error(format!("Error: {}", e), responder);
        }
    }
}
```

</pre></span></p>

-----------------------

/src/node/types.rs:
-----------------------

<p align="left"><span><pre>

```rust
use std::time::SystemTime;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ConsoleLogLevel {
  Info,
  Debug,
  Warn,
  Error,
  Trace
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ConsoleLogParam {
  pub level: ConsoleLogLevel,
  pub logmsg: String,
  pub logdata: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FSOptions {
  pub encoding: Option<String>,
  pub recursive: Option<bool>,
  #[serde(rename = "withFileTypes")]
  pub with_file_types:  Option<bool>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FSDirent {
  pub name: String,
  #[serde(rename = "isDirectory")]
  pub is_directory: bool,
  #[serde(rename = "isFile")]
  pub is_file: bool,
}

impl FSDirent {
  pub fn new(name: String, is_directory: bool) -> FSDirent {
    FSDirent {name, is_directory, is_file:!is_directory}
  }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct HTTPOptions {
  pub hostname: String,
  pub port: i32,
  pub path: String,
  pub method: String
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FSStat {
  #[serde(rename = "isDirectory")]
  pub is_directory: bool,
  pub birthtime: Option<SystemTime>,
  pub mtime: Option<SystemTime>
}
impl FSStat {
  pub fn new(is_directory: bool, birthtime:Option<SystemTime>, mtime:Option<SystemTime>) -> FSStat {
    FSStat { is_directory, birthtime, mtime }
  }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NETOptions {
  
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "command")]
pub enum NodeCommand {
  ConsoleLog {params: ConsoleLogParam},
  GetProcessInfo,
  GetStartArgs,
  FSAccess {path:String, mode:i32},
  FSLstat {path:String},
  FSMkdir {path:String, options:Option<FSOptions>},
  FSReadFile {path:String, options:Option<FSOptions>},
  FSReadDir {path:String, options:Option<FSOptions>},
  FSWriteFile {path:String, options:Option<FSOptions>},
  FSWatch {path:String, wid:String, options:Option<FSOptions>},
  FSWatchClose {wid:String},
  FSOpen {fd:i64, path:String, flags:String, mode:String},
  FSClose {fd:i64},
  FSRead {fd:i64, offset:i64, length:usize, position:Option<u64>},
  FSWrite {fd:i64, offset:i64, length:usize, position:Option<u64>},
  FSRealPath {path:String},
  FSFdatasync {fd:i64},
  NETCreateServer {hook:String, options: Option<NETOptions>},
  NETCloseServer {id:String},
  NETCloseConnection {id:String},
  NETCreateConnection {hook:String, id:String},
  NETWriteConnection {id:String},
  HTTPRequest {options:HTTPOptions},
  ChildProcessSpawn {cmd: String, args:Option<Vec<String>>},
  ChildProcessStdinWrite {pid: String},
  ChildProcessDisconnect {pid: String},
  GetDataBlob {id: String}
}

#[derive(Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProcessVersions {
  pub node: String,
  pub chrome: String,
  pub electron: String,
}
impl ProcessVersions {
  pub fn new(node: String, chrome: String, electron: String) -> ProcessVersions {
    ProcessVersions {node, chrome, electron}
  }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProcessEnv {
  #[serde(rename = "NODE_ENV")]
  pub node_env: String,
  #[serde(rename = "ELECTRON_IS_DEV")]
  pub electron_is_dev: String,
  #[serde(rename = "HOME")]
  pub home: String,
}
impl ProcessEnv {
  pub fn new(node_env: String, electron_is_dev: String, home: String) -> ProcessEnv {
    ProcessEnv {node_env, electron_is_dev, home}
  }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Process {
  pub platform: String,
  pub versions: ProcessVersions,
  pub env: ProcessEnv,
  #[serde(rename = "resourcesPath")]
  pub resources_path: String
}
impl Process {
  pub fn new(platform:String, versions:ProcessVersions, env: ProcessEnv, resources_path: String) -> Process {
    Process {platform, versions, env, resources_path}
  }
}
```

</pre></span></p>


-----------------------

/src/types.rs:
-----------------------

<p align="left"><span><pre>

```rust
use std::sync::mpsc::Sender;
use notify::Event;
use wry::RequestAsyncResponder;
use crate::{electron::types::ElectronCommand, ipcchannel::IPCMsg, node::types::NodeCommand};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Resources {
    pub link: Option<String>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Package {
    pub main: String,
    pub version: String,
    pub name: String
}

pub enum ChildProcess {
  StdinWrite {data: Vec<u8>},
  Disconnect
}

pub enum NETConnection {
  Write {data: Vec<u8>},
  Disconnect,
  EndConnection
}

pub enum NETServer {
  Close
}

pub enum BackendCommand {
  ChildProcessCallback {pid:String, stream:String, data:Option<Vec<u8>>},
  ChildProcessExit {pid:String, exit_code:Option<i32>},
  FSWatchEvent {wid:String, event:Event},
  NETServerStart {id:String, sender:Sender<NETServer>},
  NETServerConnStart {hook:String, id:String, sender:Sender<NETConnection>},
  NETConnectionData {id:String, data:Option<Vec<u8>>},
  NETConnectionEnd {id:String},
  NETClientConnStart {id:String, sender:Sender<NETConnection>},
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "action")]
pub enum Command {
  PostIPC {browser_window_id: String, request_id:String, params: String},
  SetIPCResponse {request_id:String, params: String},
  DOMContentLoaded {browser_window_id: String, title:String},
  BrowserWindowReadFile {browser_window_id: String, file_path: String, module:bool},
  Node {invoke:NodeCommand},
  Electron {invoke:ElectronCommand},
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "action")]
pub enum FrontendCommand {
  PostIPC {request_id:String, nonce:String, params: String},
  GetProcessInfo {nonce:String},
  DOMContentLoaded {title: String},
  Alert {message: String}
}

pub enum ElectricoEvents {
  ExecuteCommand {command: Command, responder: RequestAsyncResponder, data_blob:Option<Vec<u8>>},
  FrontendNavigate {browser_window_id:String, page: String, preload: String},
  IPCCallRetry {browser_window_id:String, request_id:String, params:String, sender:Sender<IPCMsg>},
  SendChannelMessageRetry { browser_window_id:String, channel:String, args:String},
  Exit,
  Noop
}

```

</pre></span></p>


<p align="center">
	<br><span>[back to top](#---back)</span><br>
</p>

<br><br>
