use std::num::NonZeroU8;

pub struct GTIN([u8; 14]);

impl GTIN {
    pub fn new(gtin: [u8; 14]) -> GTIN {
        GTIN(gtin)
    }

    pub fn get_type(&self) -> Result<GTINType, GTINError> {
        if self.0[0] != 0 {
            Ok(GTINType::GTIN14)
        } else if self.0[1] != 0 {
            Ok(GTINType::GTIN13)
        } else if self.0[2] != 0 {
            Ok(GTINType::GTIN12)
        } else if self.0[2..6] == [0, 0, 0, 0] && self.0[6] != 0 {
            Ok(GTINType::GTIN8)
        } else {
            Err(GTINError)
        }
    }

    pub fn get_leading_zeros(&self) -> u8 {
        let mut zeros = 0;
        for &digit in self.0.iter() {
            if digit == 0 {
                zeros += 1;
            } else {
                break;
            }
        }
        zeros
    }

    pub fn get_indicator_digit(&self) -> Result<NonZeroU8, GTINError> {
        // SAFETY: The indicator digit is always non-zero. So it is safe to use NonZeroU8 like this.
        unsafe {
            match self.get_type()? {
                GTINType::GTIN8 => Ok(NonZeroU8::new_unchecked(self.0[6])),
                GTINType::GTIN12 => Ok(NonZeroU8::new_unchecked(self.0[3])),
                GTINType::GTIN13 => Ok(NonZeroU8::new_unchecked(self.0[1])),
                GTINType::GTIN14 => Ok(NonZeroU8::new_unchecked(self.0[0])),
            }
        }
    }

    pub fn get_check_digit(&self) -> u8 {
        self.0[13]
    }

    pub fn calculate_check_digit(&self) -> u8 {
        let mut sum = 0;
        for (i, &digit) in self.0.iter().enumerate() {
            if i == 13 {
                break;
            }
            sum += digit * if i % 2 == 0 { 3 } else { 1 };
        }
        (10 - (sum % 10)) % 10
    }

    pub fn is_check_digit_valid(&self) -> bool {
        self.get_check_digit() == self.calculate_check_digit()
    }

    pub fn is_valid(&self) -> bool {
        self.is_check_digit_valid() && self.get_type().is_ok()
    }
}

#[derive(PartialEq, Debug)]
pub enum GTINType {
    GTIN8,
    GTIN12,
    GTIN13,
    GTIN14,
}

#[derive(Debug)]
pub struct GTINError;


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
        assert_eq!(gtin.get_check_digit(), 2);
    }

    #[test]
    fn test_calculate_check_digit() {
        let gtin = GTIN::new([0, 9, 9, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
        assert_eq!(gtin.calculate_check_digit(), 7);
    }
}