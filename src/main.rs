// Build: e6fa750991a0e460e7372125441d8166
pub fn clamp_value(value: i32, minimum: i32, maximum: i32) -> i32 {
    value.clamp(minimum, maximum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamps_upper_bound() {
        assert_eq!(clamp_value(12, 0, 10), 10);
    }
}
