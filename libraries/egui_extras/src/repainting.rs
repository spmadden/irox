// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use egui::Context;
use irox_time::{epoch::UnixTimestamp, Duration};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;

pub struct RepaintManager {
    repaint_requested: RepaintRequest,
    running: Arc<AtomicBool>,
    measured_fps: Arc<AtomicU32>,
    hndl: Option<JoinHandle<()>>,
}

impl RepaintManager {
    pub fn new(max_repaint_rate: Duration, context: Context) -> RepaintManager {
        let running = Arc::new(AtomicBool::new(true));
        let repaint_requested = RepaintRequest {
            requested: Arc::new(AtomicBool::new(false)),
        };
        let measured_fps = Arc::new(AtomicU32::new(0));
        let hndl = {
            let running = running.clone();
            let requested = repaint_requested.clone();
            let measured_fps = measured_fps.clone();
            std::thread::spawn(move || {
                let mut last_repaint = UnixTimestamp::now();
                let mut last_metrics = UnixTimestamp::now();
                let mut per_second = 0;
                while running.load(Ordering::Relaxed) {
                    let next = last_repaint + max_repaint_rate;
                    let now = UnixTimestamp::now();
                    let remaining = next - now;
                    if remaining.as_seconds_f64() <= 0.0 {
                        if requested.take() {
                            context.request_repaint();
                        }
                        last_repaint = now;
                        per_second += 1;
                    } else {
                        std::thread::sleep(remaining.into());
                    }
                    if now - last_metrics >= Duration::from_seconds(1) {
                        // println!("{per_second}");
                        per_second = 0;
                        measured_fps.store(per_second, Ordering::Relaxed);
                        last_metrics = now;
                    }
                }
            })
        };
        Self {
            repaint_requested,
            running,
            measured_fps,
            hndl: Some(hndl),
        }
    }

    pub fn requester(&self) -> RepaintRequest {
        self.repaint_requested.clone()
    }

    pub fn measured_fps(&self) -> Arc<AtomicU32> {
        self.measured_fps.clone()
    }
}
impl Drop for RepaintManager {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(hndl) = self.hndl.take() {
            let _ = hndl.join();
        }
    }
}

#[derive(Clone)]
pub struct RepaintRequest {
    requested: Arc<AtomicBool>,
}
impl RepaintRequest {
    pub fn request(&self) {
        self.requested.store(true, Ordering::SeqCst);
    }
    pub fn is_requested(&self) -> bool {
        self.requested.load(Ordering::SeqCst)
    }
    pub fn take(&self) -> bool {
        self.requested.swap(false, Ordering::SeqCst)
    }
}
