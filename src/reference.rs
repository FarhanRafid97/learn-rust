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
