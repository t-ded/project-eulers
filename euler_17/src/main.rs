fn translate_below_twenty(num: &u32) -> String {
    if *num >= 20 || *num == 0 {
        String::new()
    } else {
        match num {
            1 => String::from("one"),
            2 => String::from("two"),
            3 => String::from("three"),
            4 => String::from("four"),
            5 => String::from("five"),
            6 => String::from("six"),
            7 => String::from("seven"),
            8 => String::from("eight"),
            9 => String::from("nine"),
            10 => String::from("ten"),
            11 => String::from("eleven"),
            12 => String::from("twelve"),
            13 => String::from("thirteen"),
            15 => String::from("fifteen"),
            18 => String::from("eighteen"),
            _ => translate_below_twenty(&(num % 10)) + &*String::from("teen"),
        }
    }
}

fn translate_multiples_of_ten(num: &u32) -> String {
    if *num < 20 || *num >= 100 {
        String::new()
    } else {
        match num {
            20 => String::from("twenty"),
            30 => String::from("thirty"),
            40 => String::from("forty"),
            50 => String::from("fifty"),
            80 => String::from("eighty"),
            _ => translate_below_twenty(&(num / 10)) + &*String::from("ty"),
        }
    }
}

fn translate_double_digits(num: &u32) -> String {
    if *num < 20 {
        translate_below_twenty(&num)
    } else if *num < 100 {
        if num % 10 == 0 {
            translate_multiples_of_ten(num)
        } else {
            let unit: u32 = num % 10;
            let truncated = num / 10 * 10;
            translate_multiples_of_ten(&truncated) + &*translate_below_twenty(&unit)
        }
    } else {
        String::new()
    }
}

fn translate_hundreds(num: &u32) -> String {
    if 100 <= *num && *num < 1_000 {
        translate_below_twenty(&(num / 100)) + &*String::from("hundred")
    } else {
        String::new()
    }
}

fn translate_number(num: &u32) -> String {

    if *num < 100 {
        translate_double_digits(&num)
    } else if *num < 1_000 {
        let hundred_part = translate_hundreds(&num);
        let tens_part = translate_double_digits(&(num % 100));
        if tens_part.len() > 0 {
            hundred_part + &*String::from("and") + &*tens_part
        } else {
            hundred_part
        }

    } else if *num == 1_000 {
        String::from("onethousand")
    } else {
        String::new()
    }
}


fn main() {
    let mut num_letters = 0;
    for num in 1..=1_000 {
        let num_translated = translate_number(&num);
        println!("Number {num} written in words is {num_translated}");
        num_letters += num_translated.len();
    }
    println!("Total number of letters is {num_letters}.");
}
