use windows::core::PCSTR;
use windows::Win32::Foundation::{CloseHandle, GENERIC_READ, GENERIC_WRITE, INVALID_HANDLE_VALUE};
use windows::Win32::Storage::FileSystem::{CreateFileA, ReadFile, WriteFile, OPEN_EXISTING};
use windows::Win32::System::Pipes::{WaitNamedPipeA, NMPWAIT_USE_DEFAULT_WAIT};

const PIPE_NAME: &str = "\\\\.\\pipe\\helios_ipc\0";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let payload = if args.is_empty() { "ping".to_string() } else { args.join(" ") };

    unsafe {
        if WaitNamedPipeA(PCSTR(PIPE_NAME.as_ptr()), NMPWAIT_USE_DEFAULT_WAIT).is_err() {
            eprintln!("Daemon not running");
            std::process::exit(1);
        }
        let pipe_handle = CreateFileA(
            PCSTR(PIPE_NAME.as_ptr()),
            GENERIC_READ.0 | GENERIC_WRITE.0,
            0,
            None,
            OPEN_EXISTING,
            0,
            None,
        ).expect("Failed to connect to pipe");

        if pipe_handle == INVALID_HANDLE_VALUE {
            eprintln!("Invalid handle");
            std::process::exit(1);
        }
        let mut bytes_written = 0;
        WriteFile(pipe_handle, Some(payload.as_bytes()), Some(&mut bytes_written), None).unwrap();

        let mut buffer = [0u8; 1024];
        let mut bytes_read = 0;
        if ReadFile(pipe_handle, Some(buffer.as_mut()), Some(&mut bytes_read), None).is_ok() {
            let response = std::str::from_utf8(&buffer[..bytes_read as usize]).unwrap_or("");
            println!("Daemon: {}", response);
        }

        let _ = CloseHandle(pipe_handle);
    }
}