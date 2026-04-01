// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//! BCR-2020-012 Bytewords

use crate::cfg_feature_alloc;
use core::fmt::Write;

pub static WORDLIST: [&str; 256] = [
    "able", "acid", "also", "apex", "aqua", "arch", "atom", "aunt", "away", "axis", "back", "bald",
    "barn", "belt", "beta", "bias", "blue", "body", "brag", "brew", "bulb", "buzz", "calm", "cash",
    "cats", "chef", "city", "claw", "code", "cola", "cook", "cost", "crux", "curl", "cusp", "cyan",
    "dark", "data", "days", "deli", "dice", "diet", "door", "down", "draw", "drop", "drum", "dull",
    "duty", "each", "easy", "echo", "edge", "epic", "even", "exam", "exit", "eyes", "fact", "fair",
    "fern", "figs", "film", "fish", "fizz", "flap", "flew", "flux", "foxy", "free", "frog", "fuel",
    "fund", "gala", "game", "gear", "gems", "gift", "girl", "glow", "good", "gray", "grim", "guru",
    "gush", "gyro", "half", "hang", "hard", "hawk", "heat", "help", "high", "hill", "holy", "hope",
    "horn", "huts", "iced", "idea", "idle", "inch", "inky", "into", "iris", "iron", "item", "jade",
    "jazz", "join", "jolt", "jowl", "judo", "jugs", "jump", "junk", "jury", "keep", "keno", "kept",
    "keys", "kick", "kiln", "king", "kite", "kiwi", "knob", "lamb", "lava", "lazy", "leaf", "legs",
    "liar", "limp", "lion", "list", "logo", "loud", "love", "luau", "luck", "lung", "main", "many",
    "math", "maze", "memo", "menu", "meow", "mild", "mint", "miss", "monk", "nail", "navy", "need",
    "news", "next", "noon", "note", "numb", "obey", "oboe", "omit", "onyx", "open", "oval", "owls",
    "paid", "part", "peck", "play", "plus", "poem", "pool", "pose", "puff", "puma", "purr", "quad",
    "quiz", "race", "ramp", "real", "redo", "rich", "road", "rock", "roof", "ruby", "ruin", "runs",
    "rust", "safe", "saga", "scar", "sets", "silk", "skew", "slot", "soap", "solo", "song", "stub",
    "surf", "swan", "taco", "task", "taxi", "tent", "tied", "time", "tiny", "toil", "tomb", "toys",
    "trip", "tuna", "twin", "ugly", "undo", "unit", "urge", "user", "vast", "very", "veto", "vial",
    "vibe", "view", "visa", "void", "vows", "wall", "wand", "warm", "wasp", "wave", "waxy", "webs",
    "what", "when", "whiz", "wolf", "work", "yank", "yawn", "yell", "yoga", "yurt", "zaps", "zero",
    "zest", "zinc", "zone", "zoom",
];

///
/// Gets a single unique word from the wordlist
pub fn get_word(data: u8) -> &'static str {
    if let Some(s) = WORDLIST.get(data as usize) {
        s
    } else {
        ""
    }
}
///
/// Iterate over the matching words, calling the function for each word
pub fn iter_words<F: FnMut(&str)>(data: &[u8], mut f: F) {
    for d in data {
        f(get_word(*d));
    }
}

/// Get the set of matching words for the provided bytes.
pub fn get_words<const N: usize>(data: [u8; N]) -> [&'static str; N] {
    let mut out = [""; N];
    for (d, out) in data.iter().zip(&mut out) {
        *out = get_word(*d);
    }
    out
}

/// Writes the set of associated words to the output writer
pub fn write_words<T: Write>(data: &[u8], sep: &str, out: &mut T) -> core::fmt::Result {
    let mut first = true;
    for d in data {
        let word = get_word(*d);
        if !first {
            write!(out, "{sep}")?;
        }
        first = false;
        write!(out, "{word}")?;
    }
    Ok(())
}
cfg_feature_alloc! {
    /// Creates a string based on the words for the provided data, separated with the separator
    ///
    /// ```
    /// let result = words_to_string(&[0x70, 0x01, 0x02, 0xFF], "-");
    /// assert_eq!(result, "judo-acid-also-zoom");
    /// ```
    pub fn words_to_string(data: &[u8], separator: &str) -> String {
        let mut out = String::new();
        let mut first = true;
        for d in data {
            let word = get_word(*d);
            if !first {
                out += separator;
            }
            first = false;
            out += word;
        }
        out
    }
}

#[cfg(test)]
mod test {
    use crate::hash::bytewords::{get_word, get_words, WORDLIST};

    #[test]
    pub fn test1() {
        for d in 0u8..=255 {
            let word = get_word(d);
            let exp = WORDLIST.get(d as usize).copied();
            assert_eq!(exp, Some(word));
        }
    }
    #[test]
    pub fn test2() {
        assert_eq!(["able"], get_words([0x0]));
        assert_eq!(["judo", "chef"], get_words([0x70, 0x19]));
    }

    #[test]
    #[cfg(feature = "alloc")]
    pub fn test3() {
        let data = [0x70, 0x01, 0x02, 0xFF];
        let separator = "-";
        let expected = "judo-acid-also-zoom";
        let result = crate::hash::bytewords::words_to_string(&data, separator);
        assert_eq!(result, expected);
    }
}
