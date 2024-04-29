use std::fs;

pub fn create_dir() {
    let path = "./data";
    let my_path = std::path::Path::new(path);
    if my_path.exists() {
        println!("Directory already exists! Skipping creation...");
        return;
    }
    let create_dir_result = fs::create_dir(path);
    if create_dir_result.is_ok() {
        println!("Created new directory");
    } else {
        println!("Some problem occurred. {:?}", create_dir_result.err());
    }
}

pub fn remove_dir() {
    let path = "./data";
    _ = std::fs::remove_dir(path);
    //you are a terrible person
    //_ = std::fs::remove_dir_all(path);
}

pub fn create_files() {
    let path = "./data/file01.txt";
    let path2 = "./data/file02.txt";
    let path3 = "./data/file03.txt";
    let text = "Author: James Howard";
    let text2 = "Author: Nancy Drew";
    let text3 = "Author: Enid Blyton";

    _ = std::fs::write(path, text);
    _ = std::fs::write(path2, text2);
    _ = std::fs::write(path3, text3);

    _ = std::fs::remove_file(path2);
}

pub fn read_files() {
    let file_to_read = "./data/file03.txt";
    let read_data = std::fs::read(file_to_read);

    let convert_bytes_to_string = |mut a: String, v: &u8| {
        let new_char = char::from(*v);
        a.push(new_char);
        return a;
    };
    if read_data.is_ok() {
        println!(
            "Data found is {}",
            read_data
                .ok()
                .unwrap()
                .iter()
                .fold(String::from(""), convert_bytes_to_string)
        );
    }
}
