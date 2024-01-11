use rand::Rng;

/// Get a random number from 1 to the upper_range
///
/// This module is used to get a random number from 1 to the
/// upper_range
///
/// # Arguments
///
/// * `upper_range` - The upper range of the random number
///
/// # Returns
///
/// * `i32` - The random number
///
/// # Examples
///
/// * This will return number between 1 and 10
/// ```rust
/// let random_number = get_random_number(10);
/// ```
/// * This will return number between 1 and 100
/// ```rust
/// let random_number = get_random_number(-1);
/// ```
pub fn get_random_number(upper_range: i32) -> i32 {
    let range = if upper_range > 0 { upper_range } else { 100 };

    let rng = rand::thread_rng().gen_range(1..=range);
    println!("The secret number is: {rng}");

    return rng;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_number_provided() {
        let random_number = get_random_number(10);
        assert!(random_number >= 1 && random_number <= 10);
    }

    #[test]
    fn test_get_random_number_not_provided() {
        let random_number = get_random_number(-1);
        assert!(random_number >= 1 && random_number <= 100);
    }
}
