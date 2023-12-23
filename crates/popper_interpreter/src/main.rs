use popper_interpreter::object;
use popper_interpreter::object::{Buildable, PopObject};

fn main() {
    // Utilisez la macro pour créer un objet Int avec des propriétés
    let int_object = object::Int::new().build();

    int_object.display();
}
