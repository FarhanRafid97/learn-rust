pub fn fizz_buzz_func(arg: i32) -> Vec<String> {
    let mut array: Vec<String> = Vec::new();
    let mut count = 0;
    loop {
        count += 1;
        let mut string = String::from("");
        if count % 3 == 0 {
            string += "Fizz";
        }
        if count % 5 == 0 {
            string += "Buzz";
        }

        if string.len() == 0 {
            string += &count.to_string();
        }
        array.push(string);
        if count >= arg {
            break;
        }
    }
    return array;
}
