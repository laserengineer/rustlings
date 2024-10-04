fn bigger(a: i32, b: i32) -> i32 {
    // TODO: Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    // You can optionally experiment here.
    println!("Bigger number: {}", bigger(10, 8));
    println!("Bigger number: {}", bigger(32, 42));
    println!("Bigger number: {}", bigger(42, 42));
    println!("Bigger number: {}", bigger(10, 10)); // should return 10 as both numbers are equal.
    println!("Bigger number: {}", bigger(8, 10)); // should return 10 as 8 is less than 10.
    println!("Bigger number: {}", bigger(42, 32)); // should return 42 as 42 is greater than 32.
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
