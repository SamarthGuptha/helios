use crate::orchestrator::scheduler::PeerRegistry;
use helios_proto::CompileTask;
use std::sync::Arc;
use windows::core::PCSTR;
use windows::Win32::Foundation::{ERROR_PIPE_CONNECTED, INVALID_HANDLE_VALUE};
use windows::Win32::Storage::FileSystem::{ReadFile, WriteFile, FILE_FLAGS_AND_ATTRIBUTES};
use windows::Win32::System::Pipes::{
    ConnectNamedPipe, CreateNamedPipeA, DisconnectNamedPipe,
    PIPE_READMODE_MESSAGE, PIPE_TYPE_MESSAGE, PIPE_WAIT,
};

const PIPE_NAME: &str = "\\\\.\\pipe\\helios_ipc\0";

pub fn start_ipc_server(registry: Arc<PeerRegistry>) {
    println!("Daemon IPC: Listening on {}", PIPE_NAME);

    unsafe {
        let pipe_handle = CreateNamedPipeA(
            PCSTR(PIPE_NAME.as_ptr()),
            FILE_FLAGS_AND_ATTRIBUTES(3),
            PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
            1,
            65536,
            65536,
            0,
            None,
        ).expect("Failed to create named pipe");

        if pipe_handle == INVALID_HANDLE_VALUE {
            panic!("Invalid pipe handle");
        }

        loop {
            let connected = match ConnectNamedPipe(pipe_handle, None) {
                Ok(_) => true,
                Err(e) => e.code() == ERROR_PIPE_CONNECTED.into(),
            };

            if connected {
                let mut buffer = [0u8; 4096];
                let mut bytes_read = 0;

                if ReadFile(pipe_handle, Some(buffer.as_mut()), Some(&mut bytes_read), None).is_ok() {
                    let cmd = std::str::from_utf8(&buffer[..bytes_read as usize]).unwrap_or("").trim();
                    println!("Daemon IPC received: {}", cmd);
                    let parts: Vec<&str> = cmd.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let compiler = parts[0].to_string();
                        let source_file = parts[1].to_string();
                        if let Ok(content) = std::fs::read(&source_file) {
                            println!("Read {} bytes from {}. Dispatching to network...", content.len(), source_file);

                            let task = CompileTask {
                                task_id: uuid::Uuid::new_v4().to_string(),
                                source_filename: source_file.clone(),
                                source_content: content,
                                compiler_flags: vec![],
                                compiler_version: compiler,
                                target_triple: "".to_string(),
                                env_vars: std::collections::HashMap::new(),
                            };
                            let registry_clone = registry.clone();
                            std::thread::spawn(move || {
                                let rt = tokio::runtime::Runtime::new().unwrap();
                                rt.block_on(async {
                                    match registry_clone.dispatch(task).await {
                                        Ok(response) => {
                                            println!("Success! Received compiled object ({} bytes) in {} ms",
                                                response.object_file.len(), response.duration_ms);
                                        },
                                        Err(e) => eprintln!("Network Dispatch Failed: {}", e),
                                    }
                                });
                            });
                        } else {
                            eprintln!("Failed to read source file: {}", source_file);
                        }
                    }

                    let response = "ACK: Task queued!";
                    let mut bytes_written = 0;
                    let _ = WriteFile(pipe_handle, Some(response.as_bytes()), Some(&mut bytes_written), None);
                }
            }
            let _ = DisconnectNamedPipe(pipe_handle);
        }
    }
}