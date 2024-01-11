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
    let number_num: i32 = match number.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            return -1;
        }
    };
    return number_num;
}
