use std::collections::{HashMap, hash_map};

use sysinfo::{Pid, System};

use crate::snapshot::Snapshot;

#[derive(Debug, Clone)]
pub struct Frame {
    snapshots: HashMap<Pid, Snapshot>,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            snapshots: HashMap::new(),
        }
    }

    pub fn from_sys(sys: &System) -> Self {
        let mut frame = Frame::new();

        for (pid, process) in sys.processes() {
            frame.add(Snapshot::new(*pid, process));
        }

        frame
    }

    pub fn add(&mut self, snapshot: Snapshot) {
        self.snapshots.insert(snapshot.pid(), snapshot);
    }

    pub fn get(&self, pid: &Pid) -> Option<&Snapshot> {
        self.snapshots.get(pid)
    }

    pub fn delta(&self, prev: &Frame, threshold: f32) -> FrameDelta {
        let mut deltas: Vec<(Pid, f32)> = Vec::new();
        for (pid, snapshot) in self {
            if let Some(prev) = prev.get(pid) {
                let delta = snapshot.cpu() - prev.cpu();
                if delta > threshold {
                    // small filter
                    deltas.push((*pid, delta));
                }
            } else {
                deltas.push((*pid, snapshot.cpu()));
            }
        }

        FrameDelta { deltas }
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator for &'a Frame {
    type Item = (&'a Pid, &'a Snapshot);
    type IntoIter = hash_map::Iter<'a, Pid, Snapshot>;

    fn into_iter(self) -> Self::IntoIter {
        self.snapshots.iter()
    }
}

pub struct FrameDelta {
    deltas: Vec<(Pid, f32)>,
}

impl FrameDelta {
    pub fn all(mut self) -> impl Iterator<Item = (Pid, f32)> {
        self.deltas.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        self.deltas.into_iter().filter(|(_, delta)| *delta > 0.5)
    }

    pub fn top_n(self, n: usize) -> impl Iterator<Item = (Pid, f32)> {
        self.all().take(n)
    }

    pub fn over_threshold(mut self, threshold: f32) -> impl Iterator<Item = (Pid, f32)> {
        self.deltas.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let threshold = self.deltas.iter().fold(0f32, |a, &(_, b)| a.max(b)) * threshold;
        self.deltas
            .into_iter()
            .filter(move |(_, delta)| *delta > threshold)
    }
}
