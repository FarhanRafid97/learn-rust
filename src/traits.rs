struct RedFox {
    enemy: bool,
    life: u32,
}
pub trait Noisy {
    fn get_noise(&self) -> &str;
}

impl Noisy for RedFox {
    fn get_noise(&self) -> &str {
        "STR MEOWS"
    }
}

pub fn print_noise<T: Noisy>(item: T) {
    println!("{}", item.get_noise())
}

impl Noisy for &str {
    fn get_noise(&self) -> &str {
        "TEST"
    }
}
