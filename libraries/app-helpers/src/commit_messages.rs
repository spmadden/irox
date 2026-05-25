// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

// Inspired by https://whatthecommit.com/
// https://github.com/ngerakines/commitment

use irox_tools::random::{system_random, PRNG};
use irox_tools::static_init;

static_init!(get_messages, Vec<&'static str>, {
    let data = include_str!("../data/commit_messages.txt");
    let lines = data.lines();
    lines.collect::<Vec<_>>()
});
static_init!(get_names, Vec<&'static str>, {
    let data = include_str!("../data/commit_names.txt");
    let lines = data.lines();
    lines.collect::<Vec<_>>()
});

///
/// Returns a random commit-ish message - good for testing alerts, notifications, and logs
pub fn get_message() -> String {
    let random = system_random();
    let base_msg = random.prng(|r| r.choice(get_messages()));
    let name = random.prng(|r| r.choice(get_names()));
    let base_msg = base_msg.replace("XNAMEX", name);
    let base_msg = base_msg.replace("XUPPERNAMEX", &name.to_uppercase());
    base_msg.replace("XLOWERNAMEX", &name.to_lowercase())
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test1() {
        for _ in 0..100 {
            let msg = super::get_message();

            // println!("{msg}");
            assert!(!msg.is_empty());
            assert!(!msg.contains("XNAMEX"));
            assert!(!msg.contains("XUPPERNAMEX"));
            assert!(!msg.contains("XLOWERNAMEX"));
        }
    }
}
