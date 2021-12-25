#[cfg(test)]
mod tests {

    fn create_string() {
        let mut s = String::new();
        let mut s = "hello world".to_string();
        let mut s = String::from("hello world");
    }
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

    #[test]
    fn for4() {
        let s = "hello陈";
        for item in s.as_bytes().iter() {
            println!("{}", item);
        }
    }

    #[test]
    fn join_string1() {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        // 注意 s1 被移动了，不能继续使用
        // 相当于调用方法
        // fn add(self, s: &str) -> String
        // &String被强制转换为&str，即&s2转换为&s2[..]
        // 获得s1的所有权，并push_str(&s2)，然后返回所有权给s3。比拷贝高效。
        let s3 = s1 + &s2;
    }

    #[test]
    fn join_string2() {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        // 不获得任何字符串的所有权
        let s = format!("{} {}", s1, s2);
        println!("{}", s)
    }
}