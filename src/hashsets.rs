use std::collections::HashSet;

pub fn test_hashsets() {
    let mut planets = HashSet::from(["Mercury", "venus", "Mars"]);
    let planet_list = HashSet::from(["Earth", "Mars", "Saturn"]);
    //println!("{:?}", planets);

    // for planet in planets {
    //     println!("Thanks for adding {}", planet);
    // }

    let planet_difference = planets.difference(&planet_list);
    let planet_symdiff = planets.symmetric_difference(&planet_list);

    // for planet in planet_difference {
    //     println!("Thanks for adding {}", planet);
    // }

    planets.insert("Pluto");
    planets.insert("Mars");
    planets.insert("Jupiter");
    //this will cause error uncomment lines below only
    // for planet in planet_symdiff {
    //     println!("Thanks for adding {}", planet);
    // }

    for planet in planets {
        println!("Thanks for adding {}", planet);
    }
}
