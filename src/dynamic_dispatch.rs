struct Dog {}
struct Antelope {}
struct Bear {}

trait AnimalEating {
    fn eat_food(&self);
}

trait AnimalSound {
    fn make_sound(&self);
}

impl AnimalEating for Dog {
    fn eat_food(&self) {
        println!("Dog is eating dog food");
    }
}

impl AnimalSound for Dog {
    fn make_sound(&self) {
        println!("the dog barks");
    }
}

impl Animal for Dog {}

impl AnimalEating for Antelope {
    fn eat_food(&self) {
        println!("Antelope is eating natural desert plants");
    }
}

impl AnimalSound for Antelope {
    fn make_sound(&self) {
        println!("the antelope bleats");
    }
}

impl Animal for Antelope {}

impl AnimalEating for Bear {
    fn eat_food(&self) {
        println!("Bad boy bear is eating other animals");
    }
}

impl AnimalSound for Bear {
    fn make_sound(&self) {
        println!("the bear roars");
    }
}

impl Animal for Bear {}

trait Animal: AnimalEating + AnimalSound {}

pub fn dynamic_dispatch_traits() {
    let dog01 = Dog {};
    let antelope01: &dyn Animal = &Antelope {};
    let antelope02: &dyn Animal = &Antelope {};

    make_some_noise(dog01);
    make_some_noise_dyn(antelope01);
    eat_some_food(antelope02);

    let animal01 = get_animal();
    animal01.eat_food();
    //animal01.make_sound();  not possible but can be done with super-traits
    animal01.make_sound();
}

//making it generic might look complicated but is simple
//complier will know its always getting animal and will call make sound

// fn make_some_noise<Animal: AnimalSound>(a: Animal)
//fn eat_some_food(a: &dyn AnimalEating)
//all these could now be the Animal Trait rather than individual specific traits
fn make_some_noise<Animal: AnimalSound>(a: Animal) {
    a.make_sound();
}
//now this was simple generics

fn make_some_noise_dyn(a: &dyn Animal) {
    a.make_sound();
}
//dynamic dispatch

fn eat_some_food(a: &dyn Animal) {
    a.eat_food();
}

fn get_animal() -> Box<dyn Animal> {
    let animal = Bear {};
    return Box::from(animal);
}
