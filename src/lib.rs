use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn get_config() {
    let mut contents = String::new();
    let home: String;
    match env::var("HOME") {
        Ok(v) => home = v,
        Err(_) => {
            println!("Environment var $HOME is not set.");
            return;
        }
    };

    match File::open(format!("{}/.config/sxhkd/sxhkdrc", home)) {
        Err(_) => println!("could not find sxhkdrc at file path"),
        Ok(mut f) => match f.read_to_string(&mut contents) {
            Ok(_) => println!("read to string worked"),
            Err(_) => println!("something went wrong reading the file"),
        },
    }
    println!("{}", contents);
    return;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(get_config(), ());
    }
}
