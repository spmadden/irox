// SPDX-License-Identifier: MIT
// Copyright ${YEAR} IROX Contributors
//

use irox_progress::{console::ConsoleProgressPrinter, ProgressPrinter, Task};
use irox_time::Duration;

pub fn main() -> Result<(), std::io::Error> {
    let elements = 1000;

    let prog = ConsoleProgressPrinter::new_update_rate(Duration::from_millis(100));
    let task = Task::new(0, "Test Task".to_string(), elements);
    prog.track_task_progress(&task);
    task.mark_started();
    for _i in 0..elements {
        task.mark_one_completed();

        std::thread::sleep(std::time::Duration::from_millis(5));
    }

    Ok(())
}
