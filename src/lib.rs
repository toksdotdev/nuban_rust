//! # nuban_rust
//!
//! A rust crate for validating Nigerian Uniform Bank Account Number(NUBAN)
//!

//use std::ops::Index;

///
/// Contains details of account number, which includes **bank code**, and **serial number**
///
#[derive(Debug)]
pub struct Nuban {
    bank_code: String,
    serial_number: String,
}

impl Nuban {
    /// Create new instance of ```Struct Nuban```
    ///
    /// <br>
    /// # Examples
    ///
    /// ```
    /// use nuban_rust::*;
    /// 
    /// 
    /// let mut nuban_no = Nuban::new("011", "000001457");
    /// ```
    /// <br>
    /// <br>
    /// 
    pub fn new(bankcode: &str, serialnumber: &str) -> Nuban {
        Nuban {
            bank_code: String::from(bankcode),
            serial_number: String::from(serialnumber),
        }
    }

    /// Produces the fully valid **NUBAN (including check digit)** from the previously given struct value
    /// 
    /// <br>
    /// # Examples
    /// 
    /// ```
    /// use nuban_rust::*;
    /// 
    /// let mut nuban_no = Nuban::new("011", "000001457");
    /// 
    /// assert_eq!(nuban_no.full(), "0110000014579");
    /// ```
    /// <br>
    /// <br>
    /// 
    pub fn full(&mut self) -> String {
        let full_nuban =
            self.bank_code.clone() + &self.serial_number.clone() + &self.check_digit().to_string();

        full_nuban
    }

    /// Returns the correct check digit for the NUBAN details given
    /// 
    /// <br>
    /// # Examples
    /// 
    /// ```
    /// use nuban_rust::*;
    /// 
    /// let nuban_no = Nuban::new("011", "000001457");
    /// 
    /// assert_eq!(nuban_no.check_digit(), 9);
    /// ```
    /// <br>
    /// <br>
    /// 
    pub fn check_digit(&self) -> u32 {
        let mut sum: u32 = 3
            * (&self.index(0) + &self.index(2) + &self.index(3) + &self.index(5) + &self.index(6)
                + &self.index(8) + &self.index(9) + &self.index(11)) as u32;

        sum += 7 * (&self.index(1) + &self.index(4) + &self.index(7) + &self.index(10)) as u32;

        10 - (sum % 10)
    }

    /// Helps to properly index Nuban Struct just like a normal string. 
    /// This is achieved by concatenating the ```bank_code``` and ```serial_number``` together, before indexing
    /// 
    fn index(&self, index: usize) -> u8 {
        let value = self.bank_code.clone() + &self.serial_number.clone();

        let value = value.chars().nth(index);

        match value {
            Some(index_value) => {
                if !index_value.is_numeric() {
                    panic!("Invalid characters detected in NUBAN digits.");
                }

                // Return the given value at specified index
                index_value.to_digit(10).unwrap_or_default() as u8
            }

            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Nuban;

    #[test]
    fn generated_nuban_is_valid() {
        let mut nuban_no = Nuban::new("011", "000001457");

        assert_eq!(nuban_no.full(), "0110000014579");
    }

    #[test]
    fn nuban_check_digit_is_valid() {
        let nuban_no = Nuban::new("011", "000001457");

        assert_eq!(nuban_no.check_digit(), 9);
    }
}
