pub fn process_string_asterisks(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_string = String::new();
    let mut inside_asterisks = false;

    for c in input.chars() {
        match c {
            '*' => {
                if !current_string.is_empty() {
                    result.push(current_string.clone());
                    current_string.clear();
                }
                inside_asterisks = !inside_asterisks;
            }
            _ => {
                current_string.push(c);
            }
        }
    }

    if !current_string.is_empty() {
        result.push(current_string);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_string_asterisks_works() {
        let string_without = "Hello, how're you?";
        assert_eq!(
            process_string_asterisks(&string_without),
            vec![string_without]
        );

        let string_with = "This *text* has some *important* bits";
        assert_eq!(
            process_string_asterisks(&string_with),
            vec![
                "This ".to_string(),
                "text".to_string(),
                " has some ".to_string(),
                "important".to_string(),
                " bits".to_string()
            ],
        );
    }
}
