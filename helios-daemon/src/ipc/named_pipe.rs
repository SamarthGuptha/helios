use windows::core::PCSTR;
use windows::Win32::Foundation::{GetLastError, ERROR_PIPE_CONNECTED, INVALID_HANDLE_VALUE};
use windows::Win32::Storage::FileSystem::{ReadFile, WriteFile};
use windows::Win32::System::Pipes::{
    ConnectNamedPipe, CreateNamedPipeA, DisconnectNamedPipe, PIPE_ACCESS_DUPLEX,
    PIPE_READMODE_MESSAGE, PIPE_TYPE_MESSAGE, PIPE_WAIT,
};

const PIPE_NAME: &str = "\\\\.\\pipe\\helios_ipc\0";

pub fn start_ipc_server() {
    println!("Daemon IPC: Listening on {}", PIPE_NAME);

    unsafe {
        let pipe_handle = CreateNamedPipeA(
            PCSTR(PIPE_NAME.as_ptr()),
            PIPE_ACCESS_DUPLEX,
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
            let connected = ConnectNamedPipe(pipe_handle, None).as_bool()
                    || GetLastError() == ERROR_PIPE_CONNECTED;
            if connected {
                let mut buffer = [0u8; 4096];
                let mut bytes_read = 0;

                if ReadFile(pipe_handle, Some(buffer.as_mut()), Some(&mut bytes_read), None).is_ok() {
                    let cmd = std::str::from_utf8(&buffer[..bytes_read as usize]).unwrap_or("");
                    println!("Daemon IPC received: {}", cmd);
                    let response = "ACK: Task queued";
                    let mut bytes_written = 0;
                    let _ = WriteFile(pipe_handle, Some(response.as_bytes()), Some(&mut bytes_written), None);
                }
            }

            let _ = DisconnectNamedPipe(pipe_handle);
        }
    }
}