mod calculator;
mod fizz_buzz;
fn main() {
    let sum_add = calculator::add(32, 44);
    let sum_multiply: i32 = calculator::multiply(10, 2);

    println!("Sum Add = {sum_add}");
    println!("Sum Multiply = {sum_multiply}");
    calculator::string_select("select string");
    let fizz_buzz_result = fizz_buzz::fizz_buzz_func(20);
    println!("result fizz buzz {:?}", fizz_buzz_result);
}
