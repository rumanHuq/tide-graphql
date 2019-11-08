use std::ops::Add;

struct Point<T, U> {
    x: T, // T is unknown type, we dont know if it can do mathematical ops
    y: U,
}

// We are making assumption T must implement Add trait, in order to ensure it can do mathematics
// We are also saying Add returns OUTPUT of type T ensuring mathematical capabilities
impl<T: Add<Output = T>, U: Add<Output = U>> Add for Point<T, U> {
    type Output = Point<T, U>;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn assoc_types_test() {
        let a = Point { x: 1, y: 2 };
        let b = Point { x: 1, y: 2 };

        let c = a + b;

        assert_eq!(2, c.x);
        assert_eq!(4, c.y);
    }
}
