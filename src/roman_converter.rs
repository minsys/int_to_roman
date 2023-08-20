pub fn convert(i: i64) -> Result<String, &'static str> {
    if i < 0 {
        return Err("Number must not be negative.");
    }

    let thousands : i64 = i / 1000;
    let fivehundreds : i64 = (i - thousands * 1000) / 500;
    let hundreds : i64 = (i - thousands * 1000 - fivehundreds * 500) / 100;
    let fiftees : i64 = (i - thousands * 1000 - fivehundreds * 500 - hundreds * 100) / 50;
    let tens : i64 = (i - thousands * 1000 - fivehundreds * 500 - hundreds * 100 - fiftees * 50) / 10;
    let fives : i64 = (i - thousands * 1000 - fivehundreds * 500 - hundreds * 100 - fiftees * 50 - tens * 10) / 5;
    let ones : i64 = i - thousands * 1000 - fivehundreds * 500 - hundreds * 100 - fiftees * 50 - tens * 10 - fives * 5;

    let mut roman_number = String::new();

    for _ in 0..thousands {
        roman_number.push_str("M");
    }
    append_order(fivehundreds, "D", hundreds, "C", "M", &mut roman_number);
    append_order(fiftees, "L", tens, "X", "C", &mut roman_number);
    append_order(fives, "V", ones, "I", "X", &mut roman_number);

    Ok(roman_number)
}

fn append_order( fives : i64, five_symbol : &str, ones : i64, one_symbol : &str, ten_symbol : &str, roman_number : &mut String) {
    for _ in 0..fives {
        roman_number.push_str(five_symbol);
    }

    if ones == 4 && fives == 1 {
        roman_number.pop();
        roman_number.push_str(one_symbol);
        roman_number.push_str(ten_symbol);
    }
    else if ones == 4 {
        roman_number.push_str(one_symbol);
        roman_number.push_str(five_symbol);
    } else {
        for _ in 0..ones {
            roman_number.push_str(one_symbol);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_1_return_i() {
        assert_eq!(convert(1), Ok(String::from("I")));
    }

    #[test]
    fn given_5_return_v() {
        assert_eq!(convert(5), Ok(String::from("V")));
    }

    #[test]
    fn given_10_return_x() {
        assert_eq!(convert(10), Ok(String::from("X")));
    }

    #[test]
    fn given_50_return_l() {
        assert_eq!(convert(50), Ok(String::from("L")));
    }

    #[test]
    fn given_100_return_c() {
        assert_eq!(convert(100), Ok(String::from("C")));
    }

    #[test]
    fn given_500_return_d() {
        assert_eq!(convert(500), Ok(String::from("D")));
    }

    #[test]
    fn given_1000_return_m() {
        assert_eq!(convert(1000), Ok(String::from("M")));
    }

    #[test]
    fn given_2_return_ii() {
        assert_eq!(convert(2), Ok(String::from("II")));
    }

    #[test]
    fn given_4_return_iv() {
        assert_eq!(convert(4), Ok(String::from("IV")));
    }

    #[test]
    fn given_9_return_ix() {
        assert_eq!(convert(9), Ok(String::from("IX")));
    }

    #[test]
    fn given_40_return_xl() {
        assert_eq!(convert(40), Ok(String::from("XL")));
    }

    #[test]
    fn given_94_return_xciv() {
        assert_eq!(convert(94), Ok(String::from("XCIV")));
    }

    #[test]
    fn given_9494_return_mmmmmmmmmcdxciv() {
        assert_eq!(convert(9494), Ok(String::from("MMMMMMMMMCDXCIV")));
    }

    #[test]
    fn given_1_to_5000000() {
        for i in 0..5000000 {
            let _ = convert(i);
        }
    }

    #[test]
    #[should_panic]
    fn given_negative_return_error() {
        assert_eq!(convert(5), Err(""));
    }
}