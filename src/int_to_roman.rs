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