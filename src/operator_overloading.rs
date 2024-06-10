use std::ops::Add;

struct Person {
    first_name: String,
    last_name: String,
}

struct Marriage {
    husband: Person,
    wife: Person,
    location: String,
    date: chrono::NaiveDate,
}

impl Add for Person {
    type Output = Marriage;
    fn add(self, rhs: Self) -> Self::Output {
        let new_marriage = Marriage {
            husband: self,
            wife: rhs,
            location: "SanFransisco".to_string(),
            date: chrono::offset::Local::now().date_naive(),
        };
        return new_marriage;
    }
}

struct GroceryItem {
    name: String,
    price: f32,
}

struct GroceryBill {
    items: Vec<GroceryItem>,
    tax_rate: f32,
}

impl Add<GroceryItem> for GroceryBill {
    type Output = GroceryBill;
    fn add(self, rhs: GroceryItem) -> Self::Output {
        let mut bill = self;
        bill.items.push(rhs);
        return bill;
    }
}

impl GroceryBill {
    fn calculate_total(&self) -> f32 {
        let items_total = self.items.iter().fold(0f32, |a, i| return a + i.price);
        let tax_value = items_total * (self.tax_rate / 100.00);
        return items_total + tax_value;
    }
}

pub fn overloading_test() {
    let person1 = Person {
        first_name: "SomeGuy".to_string(),
        last_name: "his".to_string(),
    };
    let person2 = Person {
        first_name: "SomeGal".to_string(),
        last_name: "her".to_string(),
    };

    let marriage = person1 + person2;

    println!(
        "{} got married to {} on the day {}",
        marriage.husband.first_name, marriage.wife.first_name, marriage.date
    );
}

pub fn grocery_bill_generation() {
    let mut new_bill = GroceryBill {
        items: Vec::new(),
        tax_rate: 18.00,
    };

    let carrots = GroceryItem {
        name: "Bag of Carrots".to_string(),
        price: 65.00,
    };

    let cheese = GroceryItem {
        name: "Cottage cheese".to_string(),
        price: 350.00,
    };

    new_bill = new_bill + carrots + cheese;
    let total = new_bill.calculate_total();

    println!("Your total is {total}");
}
