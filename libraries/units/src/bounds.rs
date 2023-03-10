pub struct Bounds<T> {
    upper_left: T,
    upper_right: T,
    lower_left: T,
    lower_right: T,
}

impl<T> Bounds<T> {
    pub const fn new(upper_left: T, upper_right: T, lower_left: T, lower_right: T) -> Bounds<T> {
        Bounds {
            upper_left,
            upper_right,
            lower_left,
            lower_right,
        }
    }

    pub fn upper_left_corner(&self) -> &T {
        &self.upper_left
    }

    pub fn lower_left_corner(&self) -> &T {
        &self.lower_left
    }

    pub fn upper_right_corner(&self) -> &T {
        &self.upper_right
    }

    pub fn lower_right_corner(&self) -> &T {
        &self.lower_right
    }
}
