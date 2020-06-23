extern crate gerasdb;

fn main() {
    static foo: i8 = gerasdb::math::add(2, 2);
    static boo: String = "hi".to_owned() + &(foo.to_string());
    println!("{}",boo);
}
