#[cfg(test)]
mod tests {
    fn create_vector() {
        let v2 = vec![1, 2, 3];

        let mut v1: Vec<i32> = Vec::new();
        v1.push(1);
        v1.push(2);
    }

    fn read_vector() {
        let v = vec![1, 2, 3];
        let a = &v[2];
        println!("{}", a);
        match v.get(2) {
            Some(a) => println!("{}", a),
            None => println!("no"),
        }
    }

    #[test]
    fn traverse_vector() {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }

        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
        for i in &v {
            println!("{}", i);
        }
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // vector存储枚举，实现vector存储不同类型
    #[test]
    fn store_different_type_in_vector() {
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
        for i in &row {
            match i {
                SpreadsheetCell::Int(v) => println!("Int:{}", v),
                SpreadsheetCell::Float(v) => println!("Float:{}", v),
                SpreadsheetCell::Text(v) => println!("String:{}", v),
                _ => {}
            }
        }
    }
}