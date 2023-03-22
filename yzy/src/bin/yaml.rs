use serde::{Serialize, Serializer};
use serde_yaml::{to_string};

#[derive(Serialize)]
struct Person {
    age: u8,
    #[serde(serialize_with = "serialize_name")]
    name: Vec<String>,
}

fn serialize_name<S>(names: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let name_str = names.join(",");
    serializer.serialize_str(&format!("[{}]", name_str))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let person1 = Person { age: 17, name: vec!["小陈".to_owned()] };
    let person2 = Person { age: 18, name: vec!["你好".to_owned()] };
    let people = vec![person1, person2];

    let yaml_string = to_string(&people)?;
    println!("{}", yaml_string);

    std::fs::write("people.yaml", yaml_string)?;

    Ok(())
}