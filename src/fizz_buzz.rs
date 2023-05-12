pub fn fizz_buzz_func(n: i32) -> Vec<String> {
    let mut array: Vec<String> = Vec::new();
    let mut count = 1;
    while count <= n {
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
        count += 1;
    }
    return array;
}
