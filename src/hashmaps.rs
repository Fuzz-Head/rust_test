use std::collections::HashMap;

pub fn test_hashmaps() {
    let mut stock_list: HashMap<String, f32> = HashMap::new();
    // let stock_list = HashMap::<String, f32>::new();

    stock_list.insert("NVDA".to_string(), 475.28);
    stock_list.insert("AAPL".to_string(), 232.08);
    stock_list.insert("TSLA".to_string(), 170.18);
    stock_list.insert("AMZN".to_string(), 50.78);

    stock_list.insert("AAPL".to_string(), 242.78);

    //to prevent overwriting
    stock_list.entry("META".to_string()).or_insert(312.22);
    stock_list.entry("META".to_string()).or_insert(314.02);

    println!("{:#?}", stock_list);

    stock_list.remove(&("TSLA".to_string()));

    println!("{:#?}", stock_list);
    println!("{}", stock_list.len());
    println!("{}", stock_list.is_empty());

    for (ticker, value) in stock_list {
        println!("{} is trading at {}", ticker, value);
    }
}
