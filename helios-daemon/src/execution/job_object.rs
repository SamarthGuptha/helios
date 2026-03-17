use std::os::windows::io::AsRawHandle;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::JobObjects::{
    AssignProcessToJobObject, CreateJobObjectA, SetInformationJobObject,
    JobObjectExtendedLimitInformation, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
    JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
};

pub struct SandboxJob {
    handle: HANDLE,
}

impl SandboxJob {
    pub fn new() -> Result<Self, String> {
        unsafe {
            let handle = CreateJobObject(None, None)
                .map_err(|e| format!("Failed to create Windows Job Object: {}", e))?;

             let mut limit_info = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
             limit_info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;

             SetInformationJobObject(
                handle,
                JobObjectExtendedLimitInformation,
                &limit_info as *const _ as *const core::ffi::c_void,
                std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,

             ).map_err(|e| {
                let _ = CloseHandle(handle);
                format!("Failed to enforce Job Object limits: {}", e)
             })?;

             Ok(Self { handle })
        }
    }
    pub fn assign_process(&self, process: &std::process::Child) -> Result<(), String> {
        let process_handle = HANDLE(process.as_raw_handle() as _);
        unsafe {
            AssignProcessToJobObject(self.handle, process_handle)
                .map_err(|e| format!("Failed to bind process to Sandbox: {}", e))
        }
    }

}

impl Drop for SandboxJob {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.handle);
        }
    }
}