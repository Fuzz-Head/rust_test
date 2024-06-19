use regex::Regex;

pub fn test(){
    test_regex();
}

fn test_regex(){
    let my_pattern = "[A-Z]{1}[a-zA-Z]{2,8}";
    let input_text = "George";
    let name_regex = Regex::new(&my_pattern);

    if name_regex.is_err(){
        //panic!("Error in regex pattern!");
        println!("Error in regex pattern");
    }

    //let match_result = name_regex.unwrap().is_match(&input_text);
    let match_result = name_regex.unwrap().find(&input_text);//.unwrap(); // gives extra details 

    if let Some(my_match) = match_result {
        println!("Who went to the store? {}", my_match.as_str());
    }


}