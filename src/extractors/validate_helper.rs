use validator::ValidationError;

pub fn validate_registration_numbers(
    registration_numbers: &[String]
) -> Result<(), ValidationError> {
    for registration_number in registration_numbers {
        if !(4..=14).contains(&registration_number.len()) {
            return Err(ValidationError::new(
                "registration_number_length"
            ));
        }
    }

    Ok(())
}