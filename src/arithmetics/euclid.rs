use crate::{
    identities::Zero,
    structures::{integers::Integer, Ring}, equality::Equals,
};

pub fn gcd<R>(a: Integer<R>, b: Integer<R>) -> Integer<R>
where
    R: Ring,
{
    if b.equals(&Integer::zero(), 0.) {
        return a;
    }
    gcd(b, a % b)
}

pub fn euclidean_division<R>(a: Integer<R>, b: Integer<R>) -> (Integer<R>, Integer<R>)
where
    R: Ring,
{
    let q = a / b;
    let r = a - q.clone() * b;
    (q, r)
}

pub fn quotient<R>(a: Integer<R>, b: Integer<R>) -> Integer<R>
where
    R: Ring,
{
    euclidean_division(a, b).0
}

#[cfg(test)]
mod test {
    use crate::structures::integers::Integer;

    #[test]
    fn test_euclid() {
        let a = Integer::<isize>::new(1);
        let b = Integer::<isize>::new(1);
        assert_eq!(super::gcd(a, b), Integer::<isize>::new(1));
    }

    #[test]
    fn test_gcd() {
        let a = Integer::<isize>::new(252);
        let b = Integer::<isize>::new(105);
        assert_eq!(super::gcd(a, b), Integer::<isize>::new(21));
    }
}
