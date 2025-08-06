// The `From` trait is used for value-to-value conversions. If `From` is
// implemented, an implementation of `Into` is automatically provided.
// You can read more about it in the documentation:
// https://doc.rust-lang.org/std/convert/trait.From.html

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// We implement the Default trait to use it as a fallback when the provided
// string is not convertible into a `Person` object.
impl Default for Person {
    fn default() -> Self {
        Self {
            name: String::from("John"),
            age: 30,
        }
    }
}

// TODO: Complete this `From` implementation to be able to parse a `Person`
// out of a string in the form of "Mark,20".
// Note that you'll need to parse the age component into a `u8` with something
// like `"4".parse::<u8>()`.
//
// Steps:
// 1. Split the given string on the commas present in it.
// 2. If the split operation returns less or more than 2 elements, return the
//    default of `Person`.
// 3. Use the first element from the split operation as the name.
// 4. If the name is empty, return the default of `Person`.
// 5. Parse the second element from the split operation into a `u8` as the age.
// 6. If parsing the age fails, return the default of `Person`.
impl From<&str> for Person {
    fn from(s: &str) -> Self {
        // 1. 按逗号分割
        let parts: Vec<&str> = s.split(',').collect();
        // 2. 如果分割结果不等于2，返回默认
        if parts.len() != 2 {
            return Person::default();
        }

        let name = parts[0].trim();
        let age_str = parts[1].trim();

        // 3. 如果名字为空，返回默认
        if name.is_empty() {
            return Person::default();
        }

        // 4. 尝试解析年龄
        let age = match age_str.parse::<u8>() {
            Ok(a) => a,
            Err(_) => return Person::default(),
        };

        // 5. 构造 Person
        Person {
            name: name.to_string(),
            age,
        }
    }
}

fn main() {
    // Use the `from` function.
    let p1 = Person::from("Mark,20");
    println!("{p1:?}");

    // Since `From` is implemented for Person, we are able to use `Into`.
    let p2: Person = "Gerald,70".into();
    println!("{p2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }

    #[test]
    fn test_bad_convert() {
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_good_convert() {
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }

    #[test]
    fn test_bad_age() {
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,dog");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
