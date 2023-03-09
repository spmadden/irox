pub mod smarthttp;

use std::{
    io::Write,
    time::{Duration, Instant},
};

use git2::{Oid, PackBuilderStage, Progress, RemoteCallbacks};

use crate::units::{human_bytes, human_bytes_frac};

pub fn stdout_pack_progress(stage: PackBuilderStage, current: usize, total: usize) {
    let pct = current as f32 / total as f32;
    print!("{stage:?}: {current}/{total} : {pct:.2}%\r")
}

#[allow(unused_must_use)]
pub fn stdout_sideband_progress(msg: &[u8]) -> bool {
    let msg = String::from_utf8_lossy(msg);
    if (msg.starts_with("Counting") || msg.starts_with("Compressing")) && !msg.ends_with("done.") {
        print!("\r{msg}");
    } else {
        println!("{msg}");
    }
    std::io::stdout().flush();

    true
}

pub fn stdout_update_tips(msg: &str, old_oid: Oid, new_oid: Oid) -> bool {
    println!("Updated {msg}: {old_oid} -> {new_oid}");
    true
}

pub struct TransferProgress {
    pub last_call: Instant,
    pub last_bytes: usize,
    pub every_dur: Duration,
}

impl TransferProgress {
    pub fn new_update_every_duration(dur: Duration) -> TransferProgress {
        TransferProgress {
            last_call: Instant::now(),
            last_bytes: 0,
            every_dur: dur,
        }
    }

    #[allow(unused_must_use)]
    pub fn update(&mut self, progress: Progress) {
        let now = Instant::now();
        let since = now.duration_since(self.last_call);

        if since < self.every_dur {
            return;
        }
        self.last_call = now;

        let delta_bytes = progress.received_bytes() - self.last_bytes;
        let bytes_per_sec = delta_bytes as f64 / since.as_secs_f64();
        self.last_bytes = progress.received_bytes();

        print!(
            "recv: {} : {}/s  objects: idx'd {}/recv'd {}/lcl {}/tot {}  deltas: idx'd {}/tot {}\r",
            human_bytes(progress.received_bytes()),
            human_bytes_frac(bytes_per_sec),
            progress.indexed_objects(),
            progress.received_objects(),
            progress.local_objects(),
            progress.total_objects(),
            progress.indexed_deltas(),
            progress.total_deltas()
        );

        std::io::stdout().flush();
    }
}

impl Default for TransferProgress {
    fn default() -> Self {
        Self::new_update_every_duration(Duration::from_secs(1))
    }
}

pub fn stdout_callbacks<'a>() -> RemoteCallbacks<'a> {
    let mut cbs = RemoteCallbacks::new();

    let mut transfer_progress = TransferProgress::default();

    cbs.pack_progress(stdout_pack_progress);
    cbs.transfer_progress(move |p| {
        transfer_progress.update(p);
        true
    });
    cbs.sideband_progress(stdout_sideband_progress);
    cbs.update_tips(stdout_update_tips);

    cbs
}
