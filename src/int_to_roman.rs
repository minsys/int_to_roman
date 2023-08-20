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
    let ones : i64 = (i - thousands * 1000 - fivehundreds * 500 - hundreds * 100 - fiftees * 50 - tens * 10 - fives * 5);

    let mut roman_number = String::new();

    for _ in 1..thousands {
        roman_number.push_str("M");
    }
    append_order(fivehundreds, 'D', hundreds, 'C', 'M', roman_number);
    append_order(fiftees, 'L', tens, 'X', 'C', roman_number);
    append_order(fives, 'V', ones, 'I', 'X', roman_number);

    Ok(roman_number)
}

fn append_order( fives : i64, five_symbol : str&, ones : i64, oneSymbol : str&, tenSymbol : str&, roman_number : mut String&) {

}