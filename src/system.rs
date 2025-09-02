use std::time::{Duration, Instant};

use sysinfo::{Pid, System};

use crate::frame::{Frame, FrameDelta};

pub struct SystemState {
    sys: System,
    prev_cpu_total: f32,
    prev_frame: Frame,
    last_frame_update: Instant,
}

impl SystemState {
    pub fn new() -> Self {
        let mut sys = System::new();
        sys.refresh_all();

        SystemState {
            prev_cpu_total: sys.global_cpu_usage(),
            prev_frame: Frame::from_sys(&sys),
            sys,
            last_frame_update: Instant::now(),
        }
    }

    pub fn delta(&'_ mut self) -> SystemDelta<'_> {
        SystemDelta {
            curr_cpu_total: self.sys.global_cpu_usage(),
            sys_state: self,
        }
    }

    pub fn process(&self, pid: Pid) -> Option<&sysinfo::Process> {
        self.sys.process(pid)
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_all();
    }
}

pub struct SystemDelta<'a> {
    sys_state: &'a mut SystemState,
    curr_cpu_total: f32,
}

impl<'a> SystemDelta<'a> {
    pub fn cpu_total_delta(&self) -> f32 {
        self.curr_cpu_total - self.sys_state.prev_cpu_total
    }

    pub fn elapsed(&self) -> Duration {
        self.sys_state.last_frame_update.elapsed()
    }

    pub fn update(self) {
        self.sys_state.prev_cpu_total = self.curr_cpu_total;
    }

    pub fn frame_delta(self) -> SystemFrameDelta<'a> {
        SystemFrameDelta {
            curr_frame: Frame::from_sys(&self.sys_state.sys),
            sys_delta: self,
        }
    }
}

pub struct SystemFrameDelta<'a> {
    sys_delta: SystemDelta<'a>,
    curr_frame: Frame,
}

impl SystemFrameDelta<'_> {
    pub fn deltas(&self, threshold: f32) -> FrameDelta {
        self.curr_frame
            .delta(&self.sys_delta.sys_state.prev_frame, threshold)
    }

    pub fn elapsed(&self) -> Duration {
        self.sys_delta.sys_state.last_frame_update.elapsed()
    }

    pub fn update(self) {
        self.sys_delta.sys_state.prev_frame = self.curr_frame;
        self.sys_delta.sys_state.last_frame_update = Instant::now();
        self.sys_delta.update();
    }

    pub fn process(&self, pid: Pid) -> Option<&sysinfo::Process> {
        self.sys_delta.sys_state.process(pid)
    }
}
