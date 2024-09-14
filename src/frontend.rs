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
    frontendalljs:String,
    rsrc_dir:PathBuf
}
impl Frontend {
    pub fn new(rsrc_dir:PathBuf) -> Frontend {
        let mut frontendalljs:String = String::new();
        const JS_DIR_SHARED: Dir = include_dir!("src/js/shared");
        frontendalljs = append_js_scripts(frontendalljs, JS_DIR_SHARED);
        frontendalljs = append_js_scripts(frontendalljs, JS_DIR_FRONTEND);
        Frontend {
            window_ids:HashMap::new(),
            windows:HashMap::new(),
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
                            let _ = ipc_proxy.send_event(ElectricoEvents::ExecuteCommand{command:Command::PostIPC {browser_window_id:ipc_handler_id.clone(), request_id, params}, responder});
                        },
                        FrontendCommand::GetProcessInfo {nonce} => {
                            if nonce!=ipc_handler_id {
                                error!("frontend GetProcessInfo call nonce does not match - forbidden client-nonce:{} backend-nonce:{}", nonce, ipc_handler_id.clone());
                                respond_status(StatusCode::FORBIDDEN, CONTENT_TYPE_TEXT.to_string(), "forbidden".to_string().into_bytes(), responder);
                                return;
                            }
                            let _ = ipc_proxy.send_event(ElectricoEvents::ExecuteCommand{command:Command::Node { invoke: crate::node::types::NodeCommand::GetProcessInfo }, responder});
                        },
                        FrontendCommand::DOMContentLoaded {title } => {
                            let _ = ipc_proxy.send_event(ElectricoEvents::ExecuteCommand{command:Command::DOMContentLoaded {browser_window_id:ipc_handler_id.clone(), title}, responder});
                        },
                        FrontendCommand::Alert {message } => {
                            let _ = ipc_proxy.send_event(ElectricoEvents::ExecuteCommand{command:Command::Electron { invoke: crate::electron::types::ElectronCommand::ShowMessageBoxSync { options: ShowMessageBoxOptions::new(message) } }, responder});
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
            }, responder});
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
                if r.len()>0 {
                    trace!("send_channel_message OK");
                } else {
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
    pub fn close(&mut self, id:&String) {
        if let Some(win) = self.windows.get(id) {
            self.window_ids.remove(&win.id);
            self.windows.remove(id);
        }
    }
    pub fn count(&mut self) -> usize {
        self.window_ids.len()
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