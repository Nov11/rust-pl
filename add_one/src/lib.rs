#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(4, fn_add_one(3));
    }
}

pub fn fn_add_one(x: i32) -> i32 {
    x + 1
}
