pub fn add(first_arg: i32, second_arg: i32) -> i32 {
    let sum: i32 = first_arg + second_arg;
    return sum;
}
pub fn multiply(first_arg: i32, second_arg: i32) -> i32 {
    let mut my_mutable_variable = 4;
    let sum: i32 = first_arg * second_arg;
    my_mutable_variable = sum * my_mutable_variable;
    return my_mutable_variable;
}
pub fn string_select(arg_str: &str) {
    let my_string = String::from(arg_str);
    let my_str: &str = &my_string[4..9]; // "quick"
    println!("My string {my_string} {my_str}")
}
