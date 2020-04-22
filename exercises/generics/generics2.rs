// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

struct Wrapper<T> {
    value: T
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

// If I do this way I also must specify e.g. Wrapper::<u32>::new(2); Wrapper::<&[char]>::new(t);
// impl Wrapper<u32> {
//     pub fn new(value: u32) -> Self {
//         Wrapper { value }
//     }
// }

// impl<'a> Wrapper<&'a [char]> {
//     pub fn new(value: &'a [char]) -> Self {
//         Wrapper { value }
//     }
// }

// More complex example by nappa showing difference with traits
// struct Wrapper<T> {
//     value: T
// }

// trait Constructor<T> {
//     fn new(value: T) -> Self;
// }

// impl Constructor<u32> for Wrapper<u32> {
//     fn new(value: u32) -> Self {
//         Wrapper { value }
//     }
// }

// impl<'a> Constructor<&'a [char]> for Wrapper<&'a [char]> {
//     fn new(value: &'a [char]) -> Self {
//         Wrapper { value }
//     }
// }

// fn main() {
//     let _ = Wrapper::new(2);
//     let t: &[char] = &['a', 'b'];
//     let _ = Wrapper::new(t);
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value,  42);
    }

    #[test]
    fn store_str_in_wrapper() {
        // TODO: Delete this assert and uncomment the one  below once you have  finished the exercise.
        // assert!(false);
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
