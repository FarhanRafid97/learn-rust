pub fn area_of(x: i32, y: i32) -> i32 {
    return x * y;
}
pub fn volume(x: i32, y: i32, z: i32) -> i32 {
    return x * y * z;
}

pub fn main_function() {
    let width = 4;
    let height = 7;
    let depth = 10;

    {
        let area = area_of(width, height);
        println!("Area is {}", area);
    }
    println!("Valume is {}", volume(width, height, depth));
}
