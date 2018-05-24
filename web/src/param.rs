use rn_dictionary::Case;
use serde::{de, Deserialize, Deserializer};
use std::ops::Deref;

pub struct CSV(Vec<String>);

impl Deref for CSV {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for CSV {
    fn deserialize<D>(d: D) -> Result<CSV, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(d)?;
        Ok(CSV(s.split(',').map(|s| s.to_string()).collect()))
    }
}

#[cfg(test)]
mod csv_test {
    use super::*;
    use serde_json;

    #[test]
    fn it_can_parse_to_a_vec() {
        let data = "\"abc,123\"";
        let csv: CSV = serde_json::from_str(&data).unwrap();
        assert_eq!(*csv, vec!["abc", "123"]);
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Format(Case);

impl Deref for Format {
    type Target = Case;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Format {
    fn deserialize<D>(d: D) -> Result<Format, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(d)?;
        let case = s.parse::<Case>()
            .map_err(|_| de::Error::custom("Invalid case format"))?;
        Ok(Format(case))
    }
}

impl From<Case> for Format {
    fn from(case: Case) -> Format {
        Format(case)
    }
}

#[cfg(test)]
mod format_test {
    use super::*;
    use serde_json;

    #[test]
    fn it_can_parse_to_a_vec() {
        let data = "\"snake\"";
        let fmt: Format = serde_json::from_str(&data).unwrap();
        assert_eq!(*fmt, Case::Snake);
    }
}
