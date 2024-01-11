use std::io;

/// Get a number from the user
///
/// This modules get a number from the user. If number is
/// not a number, it returns -1.
///
/// # Returns
///
/// - `i32`- The number from the user
///
/// # Examples
///
/// ```rust
/// let number = get_number_from_user();
/// ```
pub fn get_number_from_user() -> i32 {
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    // let number_num: i32 = number.trim().parse::<i32>().expect("Please type a number");

    return sanitize_number(number);
}

/// Sanitize the number from the user
/// If the number is not a number, it returns -1
///
/// # Arguments
///
/// - `number` - The number from the user
///
/// # Returns
///
/// - `i32` - The number from the user
///
/// # Examples
///
/// ```rust
/// let number = sanitize_number("1".to_string());
/// ```
fn sanitize_number(number: String) -> i32 {
    return match number.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            return -1;
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_number_from_user() {
        assert_eq!(sanitize_number("1".to_string()), 1)
    }

    #[test]
    fn test_get_number_from_user_not_number() {
        assert_eq!(sanitize_number("a".to_string()), -1)
    }

    #[test]
    fn test_get_number_from_user_no_value() {
        assert_eq!(sanitize_number("".to_string()), -1)
    }
}
