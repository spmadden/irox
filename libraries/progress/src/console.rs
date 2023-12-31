// SPDX-License-Identifier: MIT
// Copyright ${YEAR} IROX Contributors
//

use std::io::{stdout, Error, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Sender, TryRecvError};
use std::sync::Arc;

use log::error;

use irox_time::epoch::UnixTimestamp;
use irox_time::format::iso8601::ISO8601Duration;
use irox_time::format::Format;
use irox_time::Duration;

use crate::{get_human, ProgressPrinter, Task};

pub struct ConsoleProgressBar {
    width: usize,
}

impl ConsoleProgressBar {
    pub fn new(width: usize) -> Self {
        ConsoleProgressBar { width }
    }

    pub fn print_infinite_progress(&self, task: &Task) -> Result<(), Error> {
        let current = task.current_progress_count();
        if let Some(started) = task.get_started() {
            let current = current as f64;
            let elapsed = started.elapsed().as_seconds_f64();
            let avg_per_sec = current / elapsed;
            let (avg_per_sec, avg_unit) = get_human!(avg_per_sec);
            let (current, unit) = get_human!(current);

            let time = (elapsed * 2.) as u64 & 0x7;
            let state = match time {
                0 => "\u{25b6}\u{25b9}\u{25b9}\u{25b9}\u{25b9}",
                1 => "\u{25b9}\u{25b6}\u{25b9}\u{25b9}\u{25b9}",
                2 => "\u{25b9}\u{25b9}\u{25b6}\u{25b9}\u{25b9}",
                3 => "\u{25b9}\u{25b9}\u{25b9}\u{25b6}\u{25b9}",
                4 => "\u{25b9}\u{25b9}\u{25b9}\u{25b9}\u{25b6}",
                _ => "\u{25b9}\u{25b9}\u{25b9}\u{25b9}\u{25b9}",
            };
            let state2 = match time & 0x3 {
                0 => "\u{25dc}",
                1 => "\u{25dd}",
                2 => "\u{25de}",
                _ => "\u{25df}",
            };

            let out = format!("| ({current:.02}{unit})  {avg_per_sec:.02}{avg_unit}/s\r");
            let spaces =
                " ".repeat(((self.width as i32 - out.len() as i32 - 9).max(1) / 4) as usize);
            let status = task.current_status().unwrap_or_default();
            let out = format!("| ({current:.02}{unit}) {spaces}{state2}{spaces} {state} {spaces}{state2}{spaces}{avg_per_sec:.02}{avg_unit}/s {status}\r");

            let mut stdio = stdout();
            stdio.write_all(out.as_bytes())?;
            return stdio.flush();
        }
        Ok(())
    }

    pub fn print_progress(&self, task: &Task) -> Result<(), Error> {
        let pct = task.current_progress_frac();
        let current = task.current_progress_count();
        let max = task.max_elements();
        if max == u64::MAX {
            return self.print_infinite_progress(task);
        }

        let rem_str = ISO8601Duration.format(&task.get_remaining_time());

        let w_pct = self.width as f64 * pct;
        let whole = w_pct.floor() as usize;
        let part = (8.0 * w_pct.fract()).round() as u8;
        let mut char = match part {
            0 => "\u{00A0}",
            1 => "\u{258F}",
            2 => "\u{258E}",
            3 => "\u{258D}",
            4 => "\u{258C}",
            5 => "\u{258B}",
            6 => "\u{258A}",
            7 => "\u{2589}",
            _ => "\u{2588}",
        };

        let mut rem = self.width - whole;
        // println!("{current} {max} {pct} {whole} {part} {rem}");
        if rem == 1 {
            rem = 0;
        } else if rem == 0 {
            char = "";
        } else {
            rem -= 1;
        }
        let whole = "\u{2588}".repeat(whole);
        let rem = " ".repeat(rem);
        let status = task.current_status().unwrap_or_default();
        let out = format!(
            "{:>3.0}%|{whole}{char}{rem}| ({current}/{max}) {rem_str} {status}\r",
            pct * 100.
        );
        let mut stdio = stdout();
        stdio.write_all(out.as_bytes())?;
        stdio.flush()
    }
}

pub struct ConsoleProgressPrinter {
    thread_handle: Option<std::thread::JoinHandle<()>>,
    running_flag: Arc<AtomicBool>,
    sender: Sender<Task>,
}

impl ConsoleProgressPrinter {
    pub fn new_update_rate(update_rate: Duration) -> ConsoleProgressPrinter {
        let running_flag = Arc::new(AtomicBool::new(true));
        let running = running_flag.clone();

        let (sender, receiver) = std::sync::mpsc::channel();

        #[allow(unused_assignments)]
        let thread_handle = std::thread::spawn(move || {
            let mut tasks: Vec<Task> = Vec::new();
            let mut last_run = UnixTimestamp::now();
            while running.load(Ordering::Relaxed) {
                last_run = UnixTimestamp::now();
                let next_run = last_run + update_rate;
                let _r = stdout().write_all(&[0x1B, b'[', b'2', b'K']);
                for task in &tasks {
                    let _res = ConsoleProgressBar::new(60).print_progress(task);
                }

                match receiver.try_recv() {
                    Ok(task) => tasks.push(task),
                    Err(e) => {
                        if e == TryRecvError::Disconnected {
                            return;
                        }
                    }
                };

                let delay = next_run - UnixTimestamp::now();
                std::thread::sleep(delay.into());
            }
            let _r = stdout().write_all(&[0x1B, b'[', b'2', b'K']);
            for task in &tasks {
                let _res = ConsoleProgressBar::new(40).print_progress(task);
            }
        });

        ConsoleProgressPrinter {
            thread_handle: Some(thread_handle),
            running_flag,
            sender,
        }
    }
}

impl Drop for ConsoleProgressPrinter {
    fn drop(&mut self) {
        self.running_flag.store(false, Ordering::Relaxed);
        if let Some(handle) = self.thread_handle.take() {
            let _res = handle.join();
        }
    }
}

impl ProgressPrinter for ConsoleProgressPrinter {
    fn track_task_progress(&self, task: &Task) {
        if let Err(e) = self.sender.send(task.clone()) {
            error!("Error sending task to printer: {e:?}");
        }
    }
}
