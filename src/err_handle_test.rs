
#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Read;
    use std::fs::File;
    use std::io::ErrorKind;

    #[test]
    fn result_test() {
        let f = File::open("hello.txt");
        let f = match f {
            Ok(file) => file,
            Err(error) => {
                panic!("Problem opening the file: {:?}", error);
            },
        };
    }

    #[test]
    fn multi_err_test() {
        let f = File::open("hello.txt");
        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => panic!("Problem opening the file: {:?}", other_error),
            },
        };
    }

    // unwrap是Result提供的方法。当Ok时返回存储的值，当Err时panic
    fn unwrap_test() {
        let f = File::open("hello.txt").unwrap();
    }

    fn expect_test() {
        let f = File::open("hello.txt").expect("Failed to open hello.txt");
    }

    fn return_result() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    // 当Ok时，返回值并继续执行。Err时，返回Err
    fn return_result_2() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    fn return_result_3() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }
}