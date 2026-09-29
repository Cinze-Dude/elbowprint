pub trait Luhn {
    fn luhn_check_sum(&self) -> u32;
    fn luhn_check_digit(&self) -> u8;
    fn luhn_pan(&self) -> String;
    fn luhn_check(&self) -> bool;
}

impl<T: AsRef<str>> Luhn for T {
    fn luhn_check_sum(&self) -> u32 {
        let pan = self.as_ref();
        let mut sum = 0;

        for (i, &b) in pan.as_bytes().iter().rev().enumerate() {
            let n = (b - b'0') as u32;

            sum += if i % 2 == 0 {
                let d = n * 2;
                if d > 9 { d - 9 } else { d }
            } else {
                n
            };
        }

        sum % 10
    }

    fn luhn_check_digit(&self) -> u8 {
        let sum = self.luhn_check_sum();
        b'0' + ((10 - sum) % 10) as u8
    }

    fn luhn_pan(&self) -> String {
        let check = self.luhn_check_digit();
        format!("{}{}", self.as_ref(), check as char)
    }

    fn luhn_check(&self) -> bool {
        let pan = self.as_ref();
        let mut sum = 0;

        for (i, &b) in pan.as_bytes().iter().rev().enumerate() {
            let mut n = (b - b'0') as u32;

            if i % 2 == 1 {
                n *= 2;
                if n > 9 {
                    n -= 9;
                }
            }

            sum += n;
        }

        sum % 10 == 0
    }
}
