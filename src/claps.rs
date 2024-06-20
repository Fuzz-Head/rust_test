use  clap::{command, Arg, ArgGroup, Command};

pub fn testclaps() {
    // testclaps1();
    testclaps2();
}
pub fn testclaps1() {
    let match_result = command!()
    .about("This application serves various purposes")
    .group(
        ArgGroup::new("person register")
            .arg("firstname")
            .arg("lastname")
    )
    .group(
        ArgGroup::new("dog-register")
            .arg("pet-name")
    )
    .arg(
        Arg::new("firstname")
        .short('f')
        .long("firstname")
        .aliases(["names", "fname", "first-name"])
    )
    .arg(
        Arg::new("lastname")
        .short('l')
        .required(true)
        //.conflicts_with("firstname")
    )
    .arg(Arg::new("pet_name")
        .long("pet-name")
        .short('p')
        .required(true)
    )
    .arg(
        Arg::new("lost")
        .short('L')
        .help("whether loser or not")
    )
    .get_matches();
}

fn testclaps2() {
    let match_result = command!()
    .subcommand(Command::new("register-person")
        .arg(
            Arg::new("firstname")
            .short('f')
            .long("firstname")
            .aliases(["names", "fname", "first-name"])
        )
        .arg(
            Arg::new("lastname")
            .short('l')
            .required(true)
            //.conflicts_with("firstname")
        )
    )
    .subcommand(Command::new("register-pet")
        .arg(
            Arg::new("pet-name")
            .long("pet-name")
            .short('p')
            .required(true)
        )
        .arg(
            Arg::new("lost")
            .short('L')
            .help("whether loser or not")
        )
    )
    .about("This application serves various purposes")
    .arg(
        Arg::new("fluffy")
        .short('p')
        .long("fluffy")
        .help("whether fluffy or not")
    )
    //remaining common args come here 

    .get_matches();

    //println!("Fluffy: {}", match_result.get_one::<String>("fluffy").unwrap());

    //println!("PetName: {}", match_result.get_one::<String>("pet_name").unwrap_or(&"No Pet".to_string()));

    // let pet_args= match_result.subcommand_matches("register-pet");
    // println!("Does pet name exists? {}", pet_args.unwrap().get_one::<String>("pet-name").unwrap());

    let person_args= match_result.subcommand_matches("register-person");
    println!("First name: {} last name: {}", 
        person_args.unwrap().get_one::<String>("firstname").unwrap(),
        person_args.unwrap().get_one::<String>("lastname").unwrap()
    );
}

