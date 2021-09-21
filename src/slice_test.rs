

/*
字符串字面值的类型是&str，就是切片。因为&str是不可变引用，所以字符串字面值不可变。
 */

#[cfg(test)]
mod tests {

    #[test]
    fn string_slice() {
        let s = String::from("hello world");

        let hello = &s[0..5];
        let world = &s[6..11];

        println!("{} {}", hello, world)
    }

    #[test]
    fn first_word_test() {
        let s = String::from("hello world");
        // let mut s = String::from("hello world");
        let s1 = first_word(&s);
        // 编译错误，因为同时有s的不可变引用和可变引用
        // s.clear();
        println!("{}", s1);
    }

    #[test]
    fn first_word_test_v2() {
        let s = String::from("hello world");
        let s1 = first_word_v2(&s);
        println!("{}", s1);

        let s1 = first_word_v2("hello world");
        println!("{}", s1);

        let s1 = first_word_v2(&s[..8]);
        println!("{}", s1);
    }

    // 用切片返回字符串第一个词组
    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i]
            }
        }
        &s[..]
    }

    // 改进版。可以同时接收&str和String作为参数
    fn first_word_v2(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i]
            }
        }
        &s[..]
    }

    #[test]
    fn array_and_slice() {
        let array = [1, 2, 3];
        let slice = &array[1..3];
        // 打印数组和切片的方法
        print!("{:?} {:?}", array, slice);
    }

    #[test]
    fn traverse_slice() {
        let array = [1, 2, 3];
        let slice = &array[1..3];

        let mut max = 0;
        // slice里的值都是原数组值的引用
        for v in slice {
            if *v > max {
                max = *v;
            }
        }

        let mut max = 0;
        for &v in slice {
            if v > max {
                max = v;
            }
        }
    }
}