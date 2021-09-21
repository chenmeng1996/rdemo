#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn create_hashmap() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("red"), 20);

        let teams  = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];
        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
        println!("{:?}", scores);
    }

    fn insert_hashmap() {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut scores = HashMap::new();
        // copy或move到hashmap。原变量move后失去所有权
        scores.insert(field_name, field_value);
    }

    #[test]
    fn read_hashmap() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("red"), 20);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name);
        if let Some(v) = score {
            println!("{}", v);
        } else {
            println!("not found");
        }
    }

    #[test]
    fn traverse_hashmap() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("red"), 20);

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
    }

    #[test]
    fn insert_when_absent() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("red"), 20);

        scores.entry(String::from("Blue")).or_insert(50);
        scores.entry(String::from("Yellow")).or_insert(50);

        println!("{:?}", scores);
    }

    #[test]
    fn update_when_exist() {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
    
        for word in text.split_whitespace() {
            // or_insert返回这个键对应的值的可变引用
            let count = map.entry(word).or_insert(0);
            // 解引用，并更新值
            *count += 1;
        }
    
        println!("{:?}", map);
    }
}