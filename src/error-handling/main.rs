use std::io::{Error, ErrorKind};

fn return_some() -> Option<i32> {
    Some(5)
}

fn return_none() -> Option<i32> {
    None
}

fn return_ok() -> Result<i32, Error> {
    Ok(42)
}

fn return_err() -> Result<i32, Error> {
    let custom_error = Error::new(ErrorKind::Other, "oh no!");
    Err(custom_error)
}

fn main() -> Result<(), Error> {
    // Option
    let _some = return_some();
    let _none = return_none();

    // Result
    let _ok = return_ok();
    let _err = return_err();

    // unwrap() vs expect()
    let _will_panic = return_err().unwrap();
    let _will_panic_with_custom_message = return_err().expect("Everything is not OK!");

    // ? operator propagates errors by returning early from a function if an Err or None is encountered
    let _will_not_early_return = return_ok()?;
    let _will_early_return = return_err()?;

    println!("Hello, world!");
    Ok(())
}
