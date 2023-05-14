pub fn reference() {
    let mut s1 = String::from("value");
    do_stuff(&mut s1);
    println!("{}", s1);
    prt(&mut s1);
    println!("{}", s1);
}

fn do_stuff(s: &mut String) {
    s.insert_str(0, "Saya, ");
    *s = String::from("Saya adalah");
}
fn prt(s: &mut String) {
    *s = String::from("Saya adalah suda berubah");
}

pub fn inspect(arg: &String) {
    if arg.ends_with("s") {
        println!("{} is plural", arg)
    } else {
        println!("{} is singular", arg)
    }
}
pub fn change(arg: &mut String) {
    arg.push_str("s")
}
pub fn eat(arg: &String) -> bool {
    if arg.starts_with("b") && arg.contains("a") {
        return true;
    } else {
        return false;
    }
}
pub fn bedazzle(arg: &mut String) {
    *arg = String::from("sparkly");
}
