#[derive(Debug)]
enum AnimalType{
    Dog,
    Cat,
}

#[derive(Debug)]
struct Animal {
    animal_name: String,
    age: u32,
    territory: String,
}

impl Animal {
    fn create_animal(
        animal_name: &str,
        age: u32,
        territory: &str,
    ) -> Self {
        Self {
            animal_name: String::from(animal_name),
            age: u32,
            territory: String::from(territory),
        }
    }

    fn print_animal(&self) {
        let Animal {
            animal_name,
            age,
            territory,
        } = self;
        println!("I created a new animal called {animal_name}, he is {age} years old, and he live in a {territory}");
    }
}