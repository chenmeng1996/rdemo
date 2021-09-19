#[cfg(test)]
mod tests {

    // 遍历字符串的utf-8字符
    #[test]
    fn for1() {
        let s = "hello陈";
        for item in s.as_bytes().iter() {
            println!("{}", item);
        }
    }

    #[test]
    fn for2() {
        let s = String::from("hello陈");
        for (i, &item) in s.as_bytes().iter().enumerate() {
            println!("{} {}", i, item);
        }
    }

    // 遍历字符串的字符
    #[test]
    fn for3() {
        let s = "hello陈";
        for item in s.chars() {
            println!("{}", item)
        }
    }
}