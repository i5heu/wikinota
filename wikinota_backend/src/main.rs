use std::process;

extern crate gerasdb;

fn main() {
    let result = match gerasdb::init() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    println!("hello world {}", result.db_name);
}

#[test]
fn main_test() {
    main();
}

#[test]
fn it_works() -> Result<(), gerasdb::dbError> {
    let result = gerasdb::init()?;
    assert_eq!(result.db_name, "HelloDBName");

    Ok(())
}
