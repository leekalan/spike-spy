use sysinfo::Pid;

#[derive(Debug, Clone, Copy)]
pub struct Snapshot {
    pid: Pid,
    cpu: f32,
    mem: u64,
}

impl Snapshot {
    /// temporary
    pub fn new(pid: Pid, process: &sysinfo::Process) -> Self {
        Snapshot {
            pid,
            cpu: process.cpu_usage(),
            mem: process.memory(),
        }
    }

    pub fn pid(&self) -> Pid {
        self.pid
    }

    pub fn cpu(&self) -> f32 {
        self.cpu
    }

    pub fn mem(&self) -> u64 {
        self.mem
    }
}
