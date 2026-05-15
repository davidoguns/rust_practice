pub fn do_foo_sep() {
    println!("Do foo sep...");
    super::do_bar();
}

pub(crate) fn do_foo() {
    println!("Doing foo..");
    super::do_bar();
}
