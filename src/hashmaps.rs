use std::collections::HashMap;

pub fn test_hashmaps () {
    let mut stock_list: HashMap<String, f32> = HashMap::new();
    // let stock_list = HashMap::<String, f32>::new();

    stock_list.insert("NVDA".to_string(), 475.78);
    stock_list.insert("AAPL".to_string(), 232.08); 
    stock_list.insert("TSLA".to_string(), 190.78);
    stock_list.insert("AMSC".to_string(), 50.78);

    println!("{:#?}", stock_list);

    stock_list.remove(&("AAPL".to_string()));

    println!("{:#?}", stock_list);
    println!("{}", stock_list.len());
    println!("{}", stock_list.is_empty());
}