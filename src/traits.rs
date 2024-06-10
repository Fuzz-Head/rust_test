struct Person<PetType, PetType2: Animal + Dangerous>
where
    PetType: Animal + NotDangerous,
{
    //alternative syntax Person <PetType> where PetType: Animal + NotDangerous
    first_name: String,
    pet: PetType,
    pet2: PetType2,
}

trait NotDangerous {}
trait Dangerous {}

trait Animal {
    fn make_sound(&self) -> ();
}
struct Dog {}
impl NotDangerous for Dog {}
impl Animal for Dog {
    fn make_sound(&self) -> () {
        println!("Dog barked");
    }
}
struct Cat {}
impl NotDangerous for Cat {}
impl Animal for Cat {
    fn make_sound(&self) -> () {
        println!("Cat meowed");
    }
}
struct Bear {}
impl Dangerous for Bear {}
impl Animal for Bear {
    fn make_sound(&self) -> () {
        println!("Bear roared");
    }
}
struct Tiger {}
impl Dangerous for Tiger {}
impl Animal for Tiger {
    fn make_sound(&self) -> () {
        println!("Bear roared");
    }
}

pub fn create_person() {
    let pet1 = Dog {};
    let pet2 = Tiger {};
    let p1 = Person {
        first_name: "Tony".to_string(),
        pet: pet1,
        pet2: pet2,
    };
    p1.pet.make_sound();
}
