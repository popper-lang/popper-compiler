

macro_rules! object {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut object = Object::new();
            $(
                object.insert($key, $value);
            )*
            object
        }
    };
}