use regex::{bytes::Captures, Regex};

pub fn test(){
    test_regex();
}

fn test_regex(){
    //let my_pattern = "[A-Z]{1}[a-zA-Z]{2,8}";
    let my_pattern = r"(?<FirstName>[A-Z]{1}[a-z]{2,8})\|(?<LastName>[A-Z]{1}[a-z]{2,8})\|(?<Year>[0-9]{4})";
    let input_text = "George|Russlle|2024";
    let name_regex = Regex::new(&my_pattern);

    if name_regex.is_err(){
        //panic!("Error in regex pattern!");
        println!("Error in regex pattern");
    }

    //let match_result = name_regex.unwrap().is_match(&input_text);
    //let match_result = name_regex.unwrap().find(&input_text);//.unwrap(); // gives extra details 
    //for capturing groups with indexes 
    let match_result = name_regex.unwrap().captures(&input_text);//.unwrap(); 


    //if let Some(my_match) = match_result 
    if let Some(captures) = match_result{
        // println!("{}", my_match.as_str());
        println!("{}, {}, {}", /* Via index 
        captures.get(1).unwrap().as_str(),
        captures.get(2).unwrap().as_str(),
        captures.get(3).unwrap().as_str(),
        */
        captures.name("FirstName").unwrap().as_str(),
        captures.name("LastName").unwrap().as_str(),
        captures.name("Year").unwrap().as_str(),
        )
    }

}