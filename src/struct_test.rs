#[cfg(test)]
mod tests {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    #[test]
    fn create_struct() {
        let mut user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        user1.email = String::from("anotheremail@example.com");

        let user2 = build_user(String::from("a"), String::from("b"));
    }

    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }

    #[test]
    fn create_struct_from_other_struct() {
        let mut user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };

        let user2 = User {
            email: String::from("another@example.com"),
            username: String::from("anotherusername567"),
            ..user1
        };
    }

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    #[test]
    fn create_tuple_struct() {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }




    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // 带self的是结构体的方法
        // &self是仅读，mut &self是读写，self是获得所有权
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        // 不带self的是结构体的关联函数
        fn square(size: u32) -> Rectangle {
            Rectangle {width: size, height: size}
        }
    }

    #[test]
    fn trait_test() {
        let r = Rectangle {width: 1, height: 2};
        println!("{:?}", r);
    }

    #[test]
    fn trait_test2() {
        let r = Rectangle {width: 1, height: 2};
        println!("{:#?}", r);
    }

    #[test]
    fn trait_test3() {
        let r = Rectangle {width: 1, height: 2};
        println!("{}", r.area());
    }

    #[test]
    fn train_test4() {
        let r1 = Rectangle {width: 1, height: 2};
        let r2 = Rectangle {width: 3, height: 4};
        println!("{}", r1.can_hold(&r2));
    }

    #[test]
    fn associated_function_test() {
        let r = Rectangle::square(1);
        println!("{:#?}", r);
    }
}