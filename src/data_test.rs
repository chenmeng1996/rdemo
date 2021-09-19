#[cfg(test)]
mod tests {
    #[test]
    fn mut_var() {
        let mut x = 1;
        println!("{}", x);
        x = 2;
        println!("{}", x);
    }

    #[test]
    fn shadow_var() {
        let mut x = 1;
        println!("{}", x);
        let x = x.to_string() + "a";
        println!("{}", x);
    }

    #[test]
    fn transform_var() {
        let a: u32 = "42".parse().expect("not a number");
        println!("{}", a);
    }

    #[test]
    fn tuple() {
        let x = (1, "a", 1.1);
        println!("{},{},{}", x.0, x.1, x.2);
        let (a,b,c) = x;
        println!("{},{},{}", a, b, c);
    }

    #[test]
    fn array() {
        let x = [7, 8];
        println!("{},{},{}", x.len(), x[0], x[1]);
    }

    #[test]
    fn loop_test() {
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        println!("{}", result)
    }

    #[test]
    fn while_test() {
        let mut counter = 0;
        while counter < 10 {
            counter += 1;
        }
        println!("{}", counter)
    }

    #[test]
    fn for_test() {
        let a = [1, 2];
        for e in a.iter() {
            println!("{}", e)
        }

        for e in (1..4).rev() {
            println!("{}", e)
        }
    }
}