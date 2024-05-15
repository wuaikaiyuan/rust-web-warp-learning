use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Deserializer, Serialize,
};

#[derive(Debug, Serialize, PartialEq)]
struct MyStruct {
    message: String,
}

async fn to_and_from_json() {
    // let json = json!({
    //     "message": "Hello, World!"
    // });

    // let my_struct: MyStruct =
    //     serde_json::from_str(&serde_json::to_string(&json).unwrap())
    //         .unwrap();
    // assert_eq!(
    //     my_struct,
    //     MyStruct {
    //         message: "Hello, World!".to_string()
    //     }
    // );
    // assert!(serde_json::to_string(&my_struct).is_ok());

    let j = "[1,2,3]";
    let u: Vec<i32> = match serde_json::from_str(j) {
        Ok(u) => u,
        Err(_) => Vec::new(),
    };
    assert_eq!(u, vec![1, 2, 3]);
}

// only traits defined in the current crate can be implemented for primitive
// types define and implement a trait or new type instead
// impl Serialize for i32 {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer {
//         serializer.serialize_i32(*self)
//     }
// }

mod color {
    use super::*;

    struct Color {
        r: u8,
        g: u8,
        b: u8,
    }

    impl Serialize for Color {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            // 3 is the number of fields in the struct.
            let mut s = serializer.serialize_struct("Color", 3)?;
            s.serialize_field("r", &self.r)?;
            s.serialize_field("g", &self.g)?;
            s.serialize_field("b", &self.b)?;
            s.end()
        }
    }
}

mod visitor {
    use super::*;
    use std::fmt;

    struct MyStructVisitor;

    impl<'de> serde::de::Visitor<'de> for MyStructVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "A message that can either be deserialized from an i32 or String")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(s.to_owned())
        }

        fn visit_string<E>(self, s: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(s)
        }

        fn visit_i32<E>(self, i: i32) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(i.to_string())
        }
    }

    impl<'de> Deserialize<'de> for MyStruct {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let message = deserializer
                .deserialize_string(MyStructVisitor)
                .map(|message| MyStruct { message })?;
            Ok(message)
        }
    }
}

mod serde_derived {
    use super::*;

    // 使用 `rename_all` 属性宏，将字段名转换为驼峰命名法
    #[derive(Debug, Deserialize, Default)]
    #[serde(rename_all = "camelCase", deny_unknown_fields, tag = "gg")]
    #[allow(dead_code)]
    struct MyStruct {
        #[serde(rename = "message")]
        my_message: String,
        #[serde(default)]
        age: Option<u16>,
        integral: u32, // 更改为u32，因为level_default函数接受u32
        // #[serde(default = "level_default")]
        #[serde(rename = "level")]
        level: Option<u8>,
        gg: String,
    }

    impl MyStruct {
        #[allow(dead_code)]
        fn level_default(&self) -> Option<u8> {
            match self.integral {
                x if (80..=100).contains(&x) => Some(Level::A as u8),
                x if (60..=79).contains(&x) => Some(Level::B as u8),
                _ => Some(Level::C as u8),
            }
        }
    }

    enum Level {
        A = 1,
        B = 2,
        C = 0,
    }

    // impl Default for MyStruct {
    //     fn default() -> Self {
    //         Self {
    //             my_message: String::new(),
    //             age: None,
    //             integral: 0,
    //             level: None,
    //             gg: String::new(),
    //         }
    //     }
    // }

    // 由于使用了枚举，这里需要实现从i32到Level的序列化
    impl<'de> Deserialize<'de> for Level {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let v = u8::deserialize(deserializer)?;
            match v {
                1 => Ok(Level::A),
                2 => Ok(Level::B),
                0 => Ok(Level::C),
                _ => Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Unsigned(v as u64),
                    &"Level must be A, B, or C",
                )),
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    to_and_from_json().await;
    Ok(())
}

#[cfg(test)]
mod test {

    fn cache(a: &i32, b: &mut i32) {
        *b = *a + *a;
        assert_eq!(*b, *a * 2);
    }

    #[test]
    fn test_cache() {
        // cache(&10, &mut 1);

        // let q = 5;
        // let mut a = Box::new(10);
        // let b = &a;
        // if q > 9 {
        //     *a = 1;
        // } else {
        //     println!("{}", b);
        // }
        // println!("{}", b);

        let mut a = Box::new(10);
        let mut b = &a;
        for i in 1..10 {
            a = Box::new(i);
            b = &a;
        }
        println!("{}", b);
        println!("{}", a);
    }
}
