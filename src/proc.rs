use std::sync::LazyLock;
use sysinfo::{get_current_pid, Pid, Process, ProcessRefreshKind, RefreshKind, System};

/// Create a new System information
pub fn create_new_info() -> System {
    System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::new()))
}

/// The Interface to work with sysinfo
pub struct ProcessMan<'a> {
    sys: &'a System,
}

impl<'a> Default for ProcessMan<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ProcessMan<'a> {
    /// Create a new Instance of the `ProcessMan` &
    /// use the singleton object System
    pub fn new() -> Self {
        static _SYS_: LazyLock<System> = LazyLock::new(create_new_info);
        ProcessMan { sys: &_SYS_ }
    }

    /// Clone the process manager
    pub fn new_from(pm: &Self) -> Self {
        pm.clone()
    }

    /* STATIC METHODS */

    /// Getting the current pid of this process
    /// # Panics
    /// Panic on getting the current pid
    pub fn get_my_pid() -> Pid {
        get_current_pid().expect("unable to get PID of the current process")
    }

    /// Get the parent pid of the current process
    pub fn get_parent_pid() -> Pid {
        let proc = Self::new();
        proc.get_parent_pid_from(Self::get_my_pid())
    }

    /* METHODS */

    /// Get the process of the pid
    /// # Panics
    /// Panic on getting process
    pub fn get_process_from(&self, pid: Pid) -> &Process {
        self.sys
            .process(pid)
            .expect("no self process or invalid Pid")
    }

    /// Get the process of the current pid
    /// # Panics
    /// Panic on getting process
    pub fn get_my_process(&self) -> &Process {
        self.sys
            .process(Self::get_my_pid())
            .expect("no self process or invalid Pid")
    }

    /// Getting the pid of the parent process from a Pid
    /// # Panics
    /// Panic on getting pid
    pub fn get_parent_pid_from(&self, pid: Pid) -> Pid {
        self.get_process_from(pid)
            .parent()
            .expect("unable to get parent process")
    }

    /// Getting the parent process from his pid
    /// # Panics
    /// Panic on getting pid
    pub fn get_parent_process_from(&self, parent_pid: Pid) -> &Process {
        self.sys.process(parent_pid).expect("Error getting info")
    }

    /// Getting the parent process from the parent of this process
    pub fn get_parent_process(&self) -> &Process {
        self.get_parent_process_from(Self::get_parent_pid())
    }

    /// Get the name of the process from his pid
    /// # Panics
    /// Panic on getting process from the pid
    pub fn get_name_from(&self, pid: Pid) -> String {
        String::from(
            self.get_process_from(pid)
                .name()
                .to_str()
                .expect("Unable to get the name of this process"),
        )
    }

    /// Get the name of the current process
    pub fn get_my_name(&self) -> String {
        self.get_name_from(Self::get_my_pid())
    }

    /// Getting the name of the parent process
    pub fn get_parent_name(&self) -> String {
        self.get_name_from(self.get_parent_pid_from(Self::get_my_pid()))
    }
}

impl<'a> Clone for ProcessMan<'a> {
    fn clone(&self) -> Self {
        Self { sys: self.sys }
    }
}
