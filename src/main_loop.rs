use std::{iter, thread, time::Duration};

use crate::{
    system::SystemState,
    util::{format_duration, format_memory},
};

pub struct SystemDetails {
    pub poll_interval: u64,
    pub cpu_spike_threshold: f32,
    pub frame_update_interval: Option<u64>,
}

pub struct Extras {
    pub suppress_frame_update_message: bool,
    pub frame_threshold_type: FrameThresholdType,
}

pub enum FrameThresholdType {
    All,
    OverThreshold(f32),
    TopN(usize),
}

pub fn main_loop(details: SystemDetails, extras: Extras) {
    let SystemDetails {
        poll_interval,
        cpu_spike_threshold,
        frame_update_interval,
    } = details;

    let mut sys_state = SystemState::new();

    loop {
        sys_state.refresh();

        let sys_delta = sys_state.delta();

        if sys_delta.cpu_total_delta() > cpu_spike_threshold {
            println!(
                "[CPU usage spike detected! ΔCPU={:.2}%]",
                sys_delta.cpu_total_delta()
            );

            let sys_delta = sys_delta.frame_delta();
            let frame_delta = sys_delta.deltas(0.5);
            let snapshot_deltas: &mut dyn Iterator<Item = _> = match extras.frame_threshold_type {
                FrameThresholdType::All => &mut frame_delta.all(),
                FrameThresholdType::OverThreshold(threshold) => {
                    &mut frame_delta.over_threshold(threshold)
                }
                FrameThresholdType::TopN(n) => &mut frame_delta.top_n(n),
            };

            if let Some(first) = snapshot_deltas.next() {
                println!(
                    "Top CPU offenders (over last {}):",
                    format_duration(sys_delta.elapsed())
                );

                for (pid, delta) in iter::once(first).chain(snapshot_deltas) {
                    if let Some(proc) = sys_delta.process(pid) {
                        println!(
                            "  {} (PID {}): ΔCPU={:.2}%, MEM={}",
                            proc.name().to_str().unwrap(),
                            pid,
                            delta,
                            format_memory(proc.memory())
                        );
                    }
                }

                println!();
            }

            sys_delta.update();
        } else if let Some(frame_update_interval) = frame_update_interval
            && sys_delta.elapsed().as_millis() as u64 > frame_update_interval
        {
            if !extras.suppress_frame_update_message {
                println!("[Frame update]");
            }
            let sys_delta = sys_delta.frame_delta();
            sys_delta.update();
        } else {
            sys_delta.update();
        }

        thread::sleep(Duration::from_millis(poll_interval));
    }
}
