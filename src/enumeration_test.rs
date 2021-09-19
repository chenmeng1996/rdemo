enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    // 枚举可以关联数据，并且数据的类型和个数可以不同
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    // 关联匿名结构体
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
    }
}



#[cfg(test)]
mod tests {
    use crate::enumeration_test::{IpAddrKind, IpAddr, Message};

    fn create_enum() {
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        let home = IpAddr::V4(127,0,0,0);
        let home = IpAddr::V6(String::from("::1"));
    }

    fn enum_method() {
        let m = Message::Write(String::from("hello"));
        m.call();
    }

    fn option_test() {
        let a = Some(5);
        let b = Some("he");
        let c: Option<i32> = None;
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    #[derive(Debug)] // 这样可以立刻看到州的名称
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            },
        }
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    #[test]
    fn plus_one_test() {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
    }

    #[test]
    fn default_match() {
        let some_u8_value = 0u8;
        match some_u8_value {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            7 => println!("seven"),
            _ => (),
        }
    }

    fn one_match() {
        let some_u8_value = Some(0u8);
        match some_u8_value {
            Some(3) => println!("three"),
            _ => (),
        }
    }

    fn if_let() {
        let some_u8_value = Some(0u8);
        if let Some(3) = some_u8_value {
            println!("three");
        }
    }
}