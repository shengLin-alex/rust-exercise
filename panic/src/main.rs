use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind;

fn main() -> Result<(), std::io::Error> {
    let f = File::open("input.txt");

    let mut f = match f {
        Ok(f) => f,
        Err(err) => {
            panic!("{:?}", err)
        }
    };
    let mut buffer = String::new();

    // use question mark to propagating error
    f.read_to_string(&mut buffer)?;

    // must flush
    f.flush().unwrap();

    println!("{}", buffer);

    let content_one: String = more_match_for_err().unwrap();
    println!("{}", content_one);

    let content_two: String = use_unwrap().unwrap();
    println!("{}", content_two);

    let content_three: String = use_expect().unwrap();
    println!("{}", content_three);

    Ok(())
}

fn more_match_for_err() -> Result<String, std::io::Error> {
    let f = File::open("input.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("input.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };

    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;
    f.flush().unwrap();

    Ok(buffer)
}

fn use_unwrap() -> Result<String, std::io::Error> {
    let mut f = File::open("input.txt").unwrap();

    // also can do: let mut f = File::open("input.txt").expect("some msg");
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;
    f.flush().unwrap();

    Ok(buffer)
}

fn use_expect() -> Result<String, std::io::Error> {
    let mut f = File::open("input.txt").expect("failure");
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;
    f.flush().unwrap();

    Ok(buffer)
}