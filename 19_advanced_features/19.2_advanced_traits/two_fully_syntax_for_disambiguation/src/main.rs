trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot say")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard say")
    }
}

impl Human {
    fn fly(&self) {
        println!("Human say")
    }
}

fn main() {
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);

    Human::fly(&person);
}
