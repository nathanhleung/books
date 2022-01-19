use num::Integer;
use num::FromPrimitive;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

/// Function that adds two to any integer type
pub fn add_two<T: Integer + FromPrimitive>(x: T) -> T {
    x + T::from_i32(2).unwrap()
}