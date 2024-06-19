use csv::{Reader, ReaderBuilder};

#[derive(serde::Deserialize, Debug)]
struct Vehicle {
    #[serde(rename(deserialize = "Manufacturer"))]
    manufacturer: String,
    #[serde(rename(deserialize = "Model"))]
    model: String,
    #[serde(rename(deserialize = "VIN"))]
    vin: String,
}

pub fn testCSV() {
    // CSVSReader();
    //CSVSReaderBuilder();
    CSVSerde();
}

fn CSVSerde() {
    let file_name = "data.csv";
    let mut builder = ReaderBuilder::new();//seperate like so if borrowed value dropped 
    builder.double_quote(false).comment(Some(b'-')).has_headers(true);//.delimiter(b'|'); // for different delimiters

    let result = builder.from_path(file_name);//can unwrap and have no error handling

    if result.is_err(){
        println!("Filed to read CSV. File path does not exists");
        std::process::exit(9);
    }

    let mut my_reader = result.unwrap();

    for record in my_reader.deserialize() {
        let car: Vehicle = record.unwrap();
        //reader will exclude the header row 
        println!("Your car manufacturer is {}", car.manufacturer);
        println!("Your car model is {}", car.model);
        println!("Your car VIN is {}", car.vin);
    }
}

fn CSVSReader() {
    let file_name = "data.csv";
    let result = Reader::from_path(file_name);

    if result.is_err(){
        println!("Filed to read CSV. File path does not exists");
        std::process::exit(9);
    }

    let mut my_reader = result.unwrap();

    for record in my_reader.records() {
        let car = record.unwrap();
        //reader will exclude the header row 
        println!("Your car manufacturer is {}", car.get(0).unwrap());
        println!("Your car model is {}", car.get(1).unwrap());
        println!("Your car VIN is {}", car.get(2).unwrap());
    }
}

fn CSVSReaderBuilder() {
    let file_name = "data.csv";
    let mut builder = ReaderBuilder::new();//seperate like so if borrowed value dropped 
    builder.double_quote(false).comment(Some(b'-')).has_headers(true);

    let result = builder.from_path(file_name);//can unwrap and have no error handling

    if result.is_err(){
        println!("Filed to read CSV. File path does not exists");
        std::process::exit(9);
    }

    let mut my_reader = result.unwrap();

    for record in my_reader.records() {
        let car = record.unwrap();
        //reader will exclude the header row 
        println!("Your car manufacturer is {}", car.get(0).unwrap());
        println!("Your car model is {}", car.get(1).unwrap());
        println!("Your car VIN is {}", car.get(2).unwrap());
    }
}