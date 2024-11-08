use std::{io::{Read, Write}, sync::mpsc::{self, Receiver, Sender}, time::Duration};

use log::trace;
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use tao::event_loop::EventLoopProxy;
use tokio::runtime::Runtime;
use wry::RequestAsyncResponder;

use crate::{backend::Backend, common::{respond_404, respond_ok}, node::{common::send_command, node::AppEnv}, types::{BackendCommand, ChildProcess, ElectricoEvents}};

use super::types::PTYCommand;

pub fn process_pty_command(tokio_runtime:&Runtime, _app_env:&AppEnv,
    proxy:EventLoopProxy<ElectricoEvents>,
    backend:&mut Backend,
    command:PTYCommand,
    responder:RequestAsyncResponder,
    _data_blob:Option<Vec<u8>>)  {
    
    let command_sender = backend.command_sender();
    match command {
        PTYCommand::Spawn {id, shell, args, opt } => {
            match native_pty_system().openpty(PtySize {
                rows: opt.rows,
                cols: opt.cols,
                pixel_width: 0,
                pixel_height: 0,
            }) {
                Ok(pair) => {
                    let mut cmd = CommandBuilder::new(shell);
                    cmd.args(args);
                    cmd.cwd(opt.cwd);
                    match pair.slave.spawn_command(cmd) {
                        Ok(_child) => {
                            let (sender, receiver): (Sender<ChildProcess>, Receiver<ChildProcess>) = mpsc::channel();
                            backend.child_process_start(&id, sender.clone());

                            tokio_runtime.spawn(
                                async move {
                                    let mut reader: Box<dyn Read + Send>;
                                    match pair.master.try_clone_reader() {
                                        Ok(r) => {
                                            reader = r;
                                        },
                                        Err(e) => {
                                            log::error!("PTYCommand try_clone_reader Error: {}", e);
                                            return;
                                        }
                                    }
                                    let mut writer: Box<dyn Write + Send>;
                                    match pair.master.take_writer() {
                                        Ok(w) => {
                                            writer = w;
                                        },
                                        Err(e) => {
                                            log::error!("PTYCommand take_writer Error: {}", e);
                                            return;
                                        }
                                    }
                                    let (read_end_sender, read_end_receiver): (Sender<ChildProcess>, Receiver<ChildProcess>) = mpsc::channel();
                                    let write_proxy = proxy.clone();
                                    let write_command_sender = command_sender.clone();
                                    let write_id = id.clone();
                                    tokio::spawn(async move {
                                        trace!("PTYCommand read");
                                        let stdout_buf:&mut [u8] = &mut [0; 65536];
                                        loop {
                                            if let Ok(read) = reader.read(stdout_buf) {
                                                trace!("PTYCommand bytes read {}", read);
                                                if let Ok(d) = read_end_receiver.try_recv() {
                                                    match d {
                                                        ChildProcess::Disconnect => {
                                                            break;
                                                        },
                                                        _ => ()
                                                    }
                                                }
                                                if read>0 {
                                                    let data:Vec<u8> = stdout_buf[0..read].to_vec();
                                                    let _ = send_command(&proxy, &command_sender, BackendCommand::ChildProcessCallback { pid:id.clone(), stream:"stdout".to_string(), data:Some(data) });
                                                }
                                            }
                                        }
                                    });
                                    tokio::spawn(async move {
                                        loop {
                                            if let Ok(cp) = receiver.recv_timeout(Duration::from_millis(100)) {
                                                match cp {
                                                    ChildProcess::StdinWrite { data } => {
                                                        trace!("PTYCommand bytes write {}", data.len());
                                                        let _ = writer.write(data.as_slice());
                                                    },
                                                    ChildProcess::Disconnect => {
                                                        let _ = read_end_sender.send(ChildProcess::Disconnect);
                                                        break;
                                                    },
                                                    _ =>()
                                                }
                                            }
                                        }
                                        trace!("PTYCommand spawn exit");
                                        let _ = send_command(&write_proxy, &write_command_sender, BackendCommand::ChildProcessExit {pid:write_id, exit_code:None});
                                    });
                                }
                            );
                            respond_ok(responder);
                        },
                        Err(e) => {
                            log::error!("PTYCommand spawn_command Error: {}", e);
                            respond_404(responder);
                        }
                    }
                },
                Err(e) => {
                    log::error!("PTYCommand Spawn Error: {}", e);
                    respond_404(responder);
                }
            }
        }
    }
}