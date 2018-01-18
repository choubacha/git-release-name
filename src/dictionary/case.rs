use std::str::FromStr;

/// The various cases that can be supported. This is a type used to coerce
/// from a string to the enum.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Case {
    Snake,
    Kebab,
    Pascal,
    Camel,
    Title,
    Sentence,
    Lower,
    Upper,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParseCaseError {
    InvalidFormat,
}

impl FromStr for Case {
    type Err = ParseCaseError;

    fn from_str(format: &str) -> Result<Case, ParseCaseError> {
        let case = match format {
            "snake" => Case::Snake,
            "kebab" => Case::Kebab,
            "pascal" => Case::Pascal,
            "camel" => Case::Camel,
            "title" => Case::Title,
            "sentence" => Case::Sentence,
            "lower" => Case::Lower,
            "upper" => Case::Upper,
            _ => return Err(ParseCaseError::InvalidFormat),
        };

        Ok(case)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_can_be_parsed_to_a_format() {
        assert_eq!(Case::Snake, "snake".parse::<Case>().unwrap());
        assert_eq!(Case::Kebab, "kebab".parse::<Case>().unwrap());
        assert_eq!(Case::Camel, "camel".parse::<Case>().unwrap());
        assert_eq!(Case::Pascal, "pascal".parse::<Case>().unwrap());
        assert_eq!(Case::Title, "title".parse::<Case>().unwrap());
        assert_eq!(Case::Sentence, "sentence".parse::<Case>().unwrap());
        assert_eq!(Case::Lower, "lower".parse::<Case>().unwrap());
        assert_eq!(Case::Upper, "upper".parse::<Case>().unwrap());
        assert!("alsdkfj".parse::<Case>().is_err());
    }
}
