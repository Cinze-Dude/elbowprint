// func luhnCheckDigit(num string) byte {
// 	sum := 0
// 	double := true
// 	for i := len(num) - 1; i >= 0; i-- {
// 		d := int(num[i] - '0')
// 		if double {
// 			d *= 2
// 			if d > 9 {
// 				d -= 9
// 			}
// 		}
// 		sum += d
// 		double = !double
// 	}
// 	return byte((10-(sum%10))%10 + '0')
// }

pub trait Luhn {
    fn luhn_check_sum(pan: &str) -> u32 {
        let mut s = 0;
        pan.as_bytes()[..pan.len() - 1]
            .iter()
            .enumerate()
            .for_each(|(i, &b)| {
                let n = (b - b'0') as u32;
                s += if i % 2 == 1 { n * 2 % 9 } else { n };
            });
        s % 10
    }

    fn luhn_check_digit(pan: &str) -> u8 {
        let mut sum = 0;
        let mut double = true;

        pan.as_bytes().iter().for_each(|b| {
            let mut d = b - b'0';
            if double {
                d = d * 2 % 9;
            }
            sum += d;
            double = !double;
        });

        b'0' + (10 - sum % 10) % 10
    }

    fn luhn_pan(pan: &mut String) {
        let check = Self::luhn_check_digit(pan);
        pan.push(char::from(check));
    }
}
