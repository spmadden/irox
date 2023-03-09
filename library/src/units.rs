pub fn human_bytes(bytes: usize) -> String {
    if bytes < KB {
        format!("{bytes} bytes")
    } else if bytes < MB {
        let val = bytes as f64 / KB as f64;
        return format!("{val:.3} KB");
    } else if bytes < GB {
        let val = bytes as f64 / MB as f64;
        return format!("{val:.3} MB");
    } else if bytes < TB {
        let val = bytes as f64 / GB as f64;
        return format!("{val:.3} GB");
    } else if bytes < PB {
        let val = bytes as f64 / TB as f64;
        return format!("{val:.3} TB");
    } else {
        let val = bytes as f64 / PB as f64;
        return format!("{val:.3} PB");
    }
}

pub fn human_bytes_frac(bytes: f64) -> String {
    if bytes < KB as f64 {
        format!("{} bytes", bytes as u64)
    } else if bytes < MB as f64 {
        let val = bytes / KB as f64;
        return format!("{val:.3} KB");
    } else if bytes < GB as f64 {
        let val = bytes / MB as f64;
        return format!("{val:.3} MB");
    } else if bytes < TB as f64 {
        let val = bytes / GB as f64;
        return format!("{val:.3} GB");
    } else if bytes < PB as f64 {
        let val = bytes / TB as f64;
        return format!("{val:.3} TB");
    } else {
        let val = bytes / PB as f64;
        return format!("{val:.3} PB");
    }
}

pub const KB: usize = 1024;
pub const MB: usize = KB * 1024;
pub const GB: usize = MB * 1024;
pub const TB: usize = GB * 1024;
pub const PB: usize = TB * 1024;
