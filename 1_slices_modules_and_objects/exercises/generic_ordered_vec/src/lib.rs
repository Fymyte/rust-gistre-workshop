/// So, we haven't seen generics, but it's pretty similar to C++ or Java.
/// As we saw in class, inheritance is done via Trait bounds.
/// PartialOrd is a trait that allows you to compare two elements of the same type. For
/// example, all numbers implement `PartialOrd`, meaning that you can compare two numbers.
/// If you were to create a custom number (such as a BigNumber struct) and implement the
/// PartialOrd trait for them, you would then be able to compare two instances of that
/// type using the <= and >= operators. This is what we're doing here: We're making sure
/// that everything that can get inside this vector can be compared to another element
/// of the same type.
/// This exercise is EXTREMELY hard since it requires concepts we haven't seen yet. We
/// can help you tho! Feel free to ask
///
/// Considering the nature of this execise, rustfmt is having a bit of trouble with our
/// broken code. Remove the comments at line 16 and 45 to start the exercise

pub struct OrderedVec<T: PartialOrd> {
    vec: Vec<T>,
}

// FIXME: Add genericity to this implementation
impl<T: PartialOrd> OrderedVec <T> {
    /// Create a new, empty ordered vector
    pub fn new() -> OrderedVec<T> {
        Self {
            vec: Vec::new(),
        }
    }

    /// Add an element to the vector, in order
    pub fn push(&mut self, value: T) {
        self.vec.push(value);
        let mut i = self.vec.len() - 1;
        while i > 0 && self.vec[i] < self.vec[i - 1] {
            self.vec.swap(i, i - 1); 
            i -= 1;
        }
    }

    /// Remove the first element from the vector and return it
    pub fn pop(&mut self) -> T {
        self.vec.remove(0)
    }

    pub fn is_sorted(&self) -> bool {
        self.vec
            .as_slice()
            .windows(2)
            .all(|tuple| tuple[0] <= tuple[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _ = OrderedVec::<i32>::new();
    }

    #[test]
    fn one_elt() {
        let mut ov = OrderedVec::new();
        ov.push(1);

        assert!(ov.is_sorted())
    }

    #[test]
    fn ordered_push() {
        let mut ov = OrderedVec::new();
        ov.push(1);
        ov.push(10);
        ov.push(100);
        ov.push(1000);

        assert!(ov.is_sorted())
    }

    #[test]
    fn reverse_order_push() {
        let mut ov = OrderedVec::new();
        ov.push(1000);
        ov.push(100);
        ov.push(10);
        ov.push(1);

        assert!(ov.is_sorted())
    }

    #[test]
    fn mix_match_push() {
        let mut ov = OrderedVec::new();
        ov.push(1);
        ov.push(100);
        ov.push(1000);
        ov.push(10);

        assert!(ov.is_sorted())
    }

    #[test]
    fn ordered_pop() {
        let mut ov = OrderedVec::new();
        ov.push(1);
        ov.push(100);
        ov.push(1000);
        ov.push(10);

        assert_eq!(ov.pop(), 1);
        assert_eq!(ov.pop(), 10);

        assert!(ov.is_sorted())
    }

    #[test]
    fn test_genericity() {
        let mut ov = OrderedVec::new();
        ov.push(1.5);
        ov.push(0.19);
        ov.push(0.00000005);
        ov.push(67.15);

        assert!(ov.is_sorted());

        assert_eq!(ov.pop(), 0.00000005);
        assert_eq!(ov.pop(), 0.19);
        assert_eq!(ov.pop(), 1.5);
    }
}
