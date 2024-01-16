/// This function validate user choise and compare it to secrete number
///
/// # Arguments
///
/// * `guess` - user choise
/// * `secret_number` - secrete number
/// * `tries` - number of tries
///
/// # Returns
///
/// * `bool` - true if guess is correct, false otherwise
///
/// # Examples
/// 1. Correct guess
/// ```rust
/// let guess = 10;
/// let secret_number = 10;
/// let tries = 10;
///
/// validate_guess(guess, secret_number, tries);
///
/// // Output:
/// // Correct guess: 10
/// ```
///
/// 2. Incorrect close but lower guess
/// ```rust
/// let guess = 16;
/// let secret_number = 20;
/// let tries = 10;
///
/// validate_guess(guess, secret_number, tries);
///
/// // Output:
/// // Close guess, but go higher
/// ```
/// 3. Incorrect higher guess
/// ```rust
/// let guess = 20;
/// let secret_number = 10;
/// let tries = 10;
///
/// validate_guess(guess, secret_number, tries);
///
/// // Output:
/// // Go much lower
/// ```
///
pub fn validate_guess(guess: i32, secret_number: i32, tries: i32) -> bool {
    if guess == secret_number {
        println!("Correct guess: {guess}");
        return true;
    }

    if tries == 1 {
        return false;
    }

    if guess < secret_number {
        if guess + 5 > secret_number {
            println!("Close guess, but go higher")
        } else {
            println!("Go much higher");
        }
    } else if guess > secret_number {
        if guess - 5 < secret_number {
            println!("Close guess, but go lower")
        } else {
            println!("Go much lower");
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_guess() {
        assert!(validate_guess(10, 10, 1));
        assert!(!validate_guess(6, 10, 1));
        assert!(!validate_guess(12, 10, 1));
    }
}
