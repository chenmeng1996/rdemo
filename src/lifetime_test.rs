// 泛型生命周期 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个。
// longest 函数并不需要知道 x 和 y 具体会存在多久，而只需要知道有某个可以被 'a 替代的作用域将会满足这个签名
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 结构体存放引用，所以其定义需要生命周期注解
// 这个注解意味着 ImportantExcerpt 的实例不能比其 part 字段中的引用存在的更久
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // 
    fn level(&self) -> i32 {
        3
    }

    // 适用于第三条生命周期省略规则
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}


#[cfg(test)]
mod tests {
    use crate::lifetime_test::*;

    #[test]
    fn lifetime_test() {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        // result是string1或string2的引用
        // 函数不知道外面变量的生命周期，为了保证函数传递出去的引用是一定正常的，在函数中加入生命周期引用
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    #[test]
    fn lifetime_test1() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.')
            .next()
            .expect("Could not find a '.'");
        let i = ImportantExcerpt { part: first_sentence };
    }
}