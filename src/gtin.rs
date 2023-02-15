use crate::error::GTINError;

use core::fmt;
use std::{num::NonZeroU8, ops::Deref, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub struct GTIN(pub [u8; 14]);

impl GTIN {
    // TODO: Is it posible to warn the user if they are using a GTIN that is not valid? especially for hardcoded GTINs that call this at compile time. Just an idea.
    #[must_use]
    pub const fn new(gtin: [u8; 14]) -> GTIN {
        GTIN(gtin)
    }

    pub const fn as_array(&self) -> [u8; 14] {
        self.0
    }

    pub const fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub const fn get_type(&self) -> Result<GTINType, GTINError> {
        if self.0[0] != 0 {
            Ok(GTINType::GTIN14)
        } else if self.0[1] != 0 {
            Ok(GTINType::GTIN13)
        } else if self.0[2] != 0 {
            Ok(GTINType::GTIN12)
        } else if self.0[2] == 0
            && self.0[3] == 0
            && self.0[4] == 0
            && self.0[5] == 0
            && self.0[6] != 0
        {
            Ok(GTINType::GTIN8)
        } else {
            Err(GTINError("invalid format"))
        }
    }

    #[must_use]
    pub fn leading_zeros(&self) -> u8 {
        let mut zeros = 0;
        for &digit in &self.0 {
            if digit == 0 {
                zeros += 1;
            } else {
                break;
            }
        }
        zeros
    }

    pub const fn indicator_digit(&self) -> Result<NonZeroU8, GTINError> {
        // SAFE: The indicator digit is always non-zero, otherwise get_type returns an error, so it is safe to use NonZeroU8::new_unchecked(...),
        unsafe {
            match self.get_type() {
                Ok(GTINType::GTIN8) => Ok(NonZeroU8::new_unchecked(self.0[6])),
                Ok(GTINType::GTIN12) => Ok(NonZeroU8::new_unchecked(self.0[3])),
                Ok(GTINType::GTIN13) => Ok(NonZeroU8::new_unchecked(self.0[1])),
                Ok(GTINType::GTIN14) => Ok(NonZeroU8::new_unchecked(self.0[0])),
                Err(e) => Err(e),
            }
        }
    }

    #[must_use]
    pub const fn check_digit(&self) -> u8 {
        self.0[13]
    }

    pub const fn calculate_check_digit(&self) -> u8 {
        macro_rules! each_digit {
            ($($i:literal),*) => {
                $(
                    self.0[$i] * if $i % 2 == 0 { 3 } else { 1 }+
                )*0
            };
        }
        let sum = each_digit!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
        (10 - (sum % 10)) % 10
    }

    #[must_use]
    pub const fn is_check_digit_valid(&self) -> bool {
        self.check_digit() == self.calculate_check_digit()
    }

    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.is_check_digit_valid() && self.get_type().is_ok()
    }
}

impl<'de> Deserialize<'de> for GTIN {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let gtin = String::deserialize(deserializer)?;
        match gtin.parse() {
            Ok(gtin) => Ok(gtin),
            Err(e) => Err(serde::de::Error::custom(e.to_string())),
        }
    }
}

impl Serialize for GTIN {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl TryFrom<String> for GTIN {
    type Error = GTINError;

    fn try_from(gtin: String) -> Result<Self, GTINError> {
        gtin.parse()
    }
}

impl TryFrom<&str> for GTIN {
    type Error = GTINError;

    fn try_from(gtin: &str) -> Result<Self, GTINError> {
        gtin.parse()
    }
}

impl FromStr for GTIN {
    type Err = GTINError;

    fn from_str(gtin: &str) -> Result<Self, Self::Err> {
        println!("gtin: {gtin} {}", gtin.len());
        if gtin.len() != 14 {
            Err(GTINError("string length is not 14"))?
        }
        let mut gtin_array = [0; 14];
        for (i, digit) in gtin.chars().enumerate() {
            gtin_array[i] = digit
                .to_digit(10)
                .ok_or(GTINError("string contains non-digit char"))?
                .try_into()
                .unwrap();
        }
        Ok(GTIN(gtin_array))
    }
}

impl fmt::Display for GTIN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|&digit| (digit + 48) as char)
                .collect::<String>()
        )
    }
}

impl Deref for GTIN {
    type Target = [u8; 14];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(PartialEq, Debug)]
pub enum GTINType {
    GTIN8,
    GTIN12,
    GTIN13,
    GTIN14,
}

// GTIN numbers are inacurate to prove the code works as expected
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_type() {
        let gtin = GTIN::new([9, 9, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 7, 7]);
        assert_eq!(gtin.get_type().unwrap(), GTINType::GTIN14);

        let gtin = GTIN::new([0, 9, 9, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 7]);
        assert_eq!(gtin.get_type().unwrap(), GTINType::GTIN13);

        let gtin = GTIN::new([0, 0, 9, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 7]);
        assert_eq!(gtin.get_type().unwrap(), GTINType::GTIN12);

        let gtin = GTIN::new([0, 0, 0, 0, 0, 0, 2, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(gtin.get_type().unwrap(), GTINType::GTIN8);
    }

    #[test]
    fn test_get_check_digit() {
        let gtin = GTIN::new([0, 9, 9, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 2]);
        assert_eq!(gtin.check_digit(), 2);
    }

    #[test]
    fn test_calculate_check_digit() {
        let gtin = GTIN::new([0, 9, 9, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
        assert_eq!(gtin.calculate_check_digit(), 7);
    }

    #[test]
    fn test_to_from_string() {
        let gtin = "09921234567897".parse::<GTIN>().unwrap();
        let gtin_string = gtin.to_string();
        assert_eq!(gtin_string, "09921234567897");

        let gtin = GTIN::try_from("09921234567897").unwrap();
        let gtin_string = gtin.to_string();
        assert_eq!(gtin_string, "09921234567897");
    }
}
