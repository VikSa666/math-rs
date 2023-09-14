use crate::{
    equality::Equals,
    identities::Zero,
    structures::{integers::Integer, Ring},
};

pub fn gcd<R>(a: &Integer<R>, b: &Integer<R>) -> Integer<R>
where
    R: Ring,
{
    if b.equals(&Integer::zero(), 0.) {
        return a.clone();
    }
    gcd(b, &(a.clone() % b.clone()))
}

pub fn euclidean_division<R>(a: &Integer<R>, b: &Integer<R>) -> (Integer<R>, Integer<R>)
where
    R: Ring,
{
    let q = a.to_owned() / b.to_owned();
    let r = a.to_owned() - q.clone() * b.to_owned();
    (q, r)
}

pub fn quotient<R>(a: &Integer<R>, b: &Integer<R>) -> Integer<R>
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
        assert_eq!(super::gcd(&a, &b), Integer::<isize>::new(1));
    }

    #[test]
    fn test_gcd() {
        let a = Integer::<isize>::new(252);
        let b = Integer::<isize>::new(105);
        assert_eq!(super::gcd(&a, &b), Integer::<isize>::new(21));
    }
}
