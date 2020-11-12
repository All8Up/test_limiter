#[derive(Debug)]
pub struct ProcessTimes {
    pub user: std::time::Duration,
    pub kernel: std::time::Duration
}

impl std::ops::Sub<ProcessTimes> for ProcessTimes {
    type Output = ProcessTimes;

    fn sub(self, rhs: ProcessTimes) -> Self::Output {
        ProcessTimes {
            user: self.user - rhs.user,
            kernel: self.kernel - rhs.kernel
        }
    }
}

#[cfg(windows)]
pub fn get() -> Option<ProcessTimes> {
    use winapi::{
        um::processthreadsapi::{
            GetProcessTimes,
            GetCurrentProcess
        },
        shared::minwindef::FILETIME
    };

    unsafe {
        let process_handle = GetCurrentProcess();

        // Go get the win32 horribles.
        let mut win_creation_time = FILETIME { dwLowDateTime: 0, dwHighDateTime: 0};
        let mut win_exit_time = FILETIME { dwLowDateTime: 0, dwHighDateTime: 0};
        let mut win_kernel_time = FILETIME { dwLowDateTime: 0, dwHighDateTime: 0};
        let mut win_user_time = FILETIME { dwLowDateTime: 0, dwHighDateTime: 0};
        
        let result = GetProcessTimes(
            process_handle,
            &mut win_creation_time,
            &mut win_exit_time,
            &mut win_kernel_time,
            &mut win_user_time
        );
        if result == 0 {
            // Eh... ?
            return None;
        }

        // Turn the horribles to something useful.
        let kernel_time = std::time::Duration::from_nanos(
            (win_kernel_time.dwLowDateTime as u64 +
            (win_kernel_time.dwHighDateTime as u64) << 32) * 100
        );
        let user_time = std::time::Duration::from_nanos(
            (win_user_time.dwLowDateTime as u64 +
            (win_user_time.dwHighDateTime as u64) << 32) * 100
        );

        Some(ProcessTimes {
            user: user_time,
            kernel: kernel_time
        })
    }    
}

#[cfg(not(windows))]
pub fn get() -> Option<ProcessTimes> {
    Some(ProcessTimes {
        user: std::time::Duration::from_secs(0),
        kernel: std::time::Duration::from_secs(0)
    })
}
