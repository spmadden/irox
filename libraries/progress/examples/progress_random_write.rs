// SPDX-License-Identifier: MIT
// Copyright ${YEAR} IROX Contributors
//

use std::fs::OpenOptions;
use std::io::{BufWriter, Error};

use irox_bits::MutBits;
use irox_progress::console::ConsoleProgressPrinter;
use irox_progress::write::WriterTask;
use irox_progress::{ProgressPrinter, Task};
use irox_time::Duration;
use irox_tools::random::{Random, PRNG};

pub fn main() -> Result<(), Error> {
    let mut rand = Random::default();

    let cons = ConsoleProgressPrinter::new_update_rate(Duration::from_millis(100));
    let task = Task::new_infinite_named("Writer".to_string());
    cons.track_task_progress(&task);
    task.mark_started();

    let out = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("test")?;
    let out = BufWriter::new(out);
    let mut out = WriterTask::new(out, task);

    let gb = 1_000_000_000 / 8;
    for _i in 0..gb {
        out.write_be_u64(rand.next_u64())?;
    }

    Ok(())
}
