use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, to_string_pretty};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Dog {
    name: String,
    year_born: i32,
    owner: DogOwner,
    #[serde(rename = "dog_breed")]
    breed: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
struct DogOwner {
    first_name: String,
    last_name: String,
}

pub fn test_serde() {
    let owner01 = DogOwner {
        first_name: "SomeBody".to_string(),
        last_name: "fromSomeWhere".to_string(),
    };
    let dog01 = Dog {
        name: "Bingo".to_string(),
        year_born: 2020,
        owner: owner01,
        breed: "Husky".to_string(),
    };
    let dog_ser = to_string_pretty(&dog01);
    if dog_ser.is_ok() {
        println!("{}", dog_ser.ok().unwrap());
    } else {
        println!("{:#?}", dog_ser.err());
    }
}

pub fn test_deser() {
    let json_string = r#"
    {
        "name": "Bingo",
        "yearBorn": 2020,
        "owner": {
          "FIRST-NAME": "SomeBody",
          "LAST-NAME": "fromSomeWhere"
        },
        "dog_breed": "Pug"
      }
      "#;

    let dog_deser = from_str::<Dog>(json_string);

    if dog_deser.is_ok() {
        println!("{:#?}", dog_deser.ok().unwrap());
    } else {
        println!("{:#?}", dog_deser.err());
    }
}
