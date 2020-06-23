extern crate gerasdb;

fn main() {
    const FOO: i8 = gerasdb::math::add(2, 2);
    let boo: String = ["hi", &(FOO.to_string())].join(" ");
    println!("{}", boo.to_string());
}

#[test]
fn it_works() {
    assert_eq!(gerasdb::math::add(2, 2), 4);
}