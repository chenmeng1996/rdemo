#[cfg(test)]
mod tests {

    #[test]
    fn copy_test() {
        let a = "a";
        let b = a;
        println!("{}", a);
        println!("{}", b);
    }

    #[test]
    fn move_test() {
        let a = String::from("a");
        let b = a;
        // println!("{}", a); // 错误
        println!("{}", b);
    }

    #[test]
    fn clone_test() {
        let a = String::from("a");
        let b = a.clone();
        println!("{}", a);
        println!("{}", b);
    }

    #[test]
    fn take_owner_ship_test() {
        let mut a = String::from("a");
        take_owner_ship(a);
        // println!("{}", a); // 错误
    }

    #[test]
    fn borrow_and_return_test() {
        let mut a = String::from("a");
        a = borrow_and_return(a);
        println!("{}", a);
    }

    #[test]
    fn unmut_reference_test() {
        let a = String::from("a");
        unmut_reference(&a);
        println!("{}", a);
    }

    #[test]
    fn mut_reference_test() {
        let mut a = String::from("a");
        mut_reference(&mut a);
        println!("{}", a);
    }

    fn take_owner_ship(a: String) {
        println!("{}", a);
    }

    fn borrow_and_return(a: String) -> String {
        println!("{}", a);
        return a;
    }

    fn unmut_reference(a: &String) {
        println!("{}", a);
    }

    fn mut_reference(a: &mut String) {
        a.push_str("b");
        println!("{}", a);
    }

    #[test]
    fn string_slice() {
        let s = String::from("hello world");
        // 切片是对字符串的某部分的引用
        let slice1 = &s[0..5];
        let slice2 = &s[5..];
        println!("{},{}", slice1, slice2);
        let total_slice = &s[..];
        let refer = &s;
        println!("{},{}", total_slice, refer);
    }

    #[test]
    fn first_world_test() {
        let s = String::from("hello world");
        let hello = first_world(&s);
        println!("{}", hello);
    }

    #[test]
    fn first_world_better_test() {
        let s = String::from("hello world");
        let hello = first_world_better(&s[..]);
        println!("{}", hello);

        let hello = first_world_better("hello world");
        println!("{}", hello);
    }

    fn first_world(s: &String) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    fn first_world_better(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
}