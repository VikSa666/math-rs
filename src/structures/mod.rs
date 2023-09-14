pub mod complex;
pub mod errors;
pub mod integers;
pub mod rationals;
pub mod reals;

use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
    str::FromStr,
};

use crate::{
    equality::Equals,
    identities::{One, Zero},
    num_types::{AsF32, FromF32},
};

/// Defines the necessary behavior of an element of a group.
///
/// # Definition
/// A **group** is a nonempty set _G_ together with a binary operation _·_ on _G_ such that the following axioms hold:
/// 1. **Closure**: For all _a_, _b_ in _G_, the result of the operation _a_ · _b_ is also in _G_.
/// 2. **Associativity**: For all _a_, _b_, _c_ in _G_, the equation (_a_ · _b_) · _c_ = _a_ · (_b_ · _c_) holds.
/// 3. **Identity element**: There exists an element _e_ in _G_ such that for every element _a_ in _G_, the equation
/// _e_ · _a_ = _a_ · _e_ = _a_ holds. Such an element is unique and thus one speaks of **the** identity element.
/// 4. **Inverse element**: For each _a_ in _G_, there exists an element _b_ in _G_ such that _a_ · _b_ = _b_ · _a_ = _e_, where _e_ is the identity element.
///
/// # Implementation
/// ## Trait bounds
/// 1. The trait [`Add`] is used to define the binary operation _·_. It is usually associated with a sum.
/// 2. The trait [`Neg`] is used to define the **inverse element**. It is usually associated with the additive inverse.
/// 3. The trait [`Sub`] is used for simplicity, as it is the same of [`Add`] and [`Neg`] combined.
/// 4. The trait [`Zero`] is used to define the **identity element**. It is usually associated with the additive identity.
/// 5. All other traits are needed for the implementation of a generic numeric type.
///
/// ## Methods
/// 1. The method [`Group::identity`] will return the identity element. It is unnecessary as it will be the same as the defined
/// [`Zero`] element. But for the sake of maintaining the mathematical notation of the definition, it is written.
/// 2. The method [`Group::inverse`] will return the inverse element of the current element. Also might seem unneded because the
/// trait [`Neg`] is already implemented, but it is necessary to maintain the mathematical notation of the definition.
/// 3. The method [`Group::op`] will return the result of the operation _·_ between the current element and the element passed as
/// argument.
///
/// # Example
/// Let's write the implementation of a group for the [`isize`] type.
/// ```ignore
/// use crate::math_rs::structures::Group;
/// impl Group for isize {
///     fn identity() -> Self {
///        0 as isize
///     }
///     fn inverse(&self) -> Self {
///         -*self
///     }
///     fn op(&self, rhs: &Self) -> Self {
///        *self + *rhs
///     }
/// }
/// ```
/// # References
/// 1. [Wikipedia](https://en.wikipedia.org/wiki/Group_(mathematics))
/// 2. [MathWorld](https://mathworld.wolfram.com/Group.html)
/// 3. [ProofWiki](https://proofwiki.org/wiki/Definition:Group)
/// 4. [PlanetMath](https://planetmath.org/definitionofagroup)
pub trait Group:
    Add<Output = Self>
    + Sub<Output = Self>
    + Neg<Output = Self>
    + Zero
    + Equals
    + Sized
    + Copy
    + Display
    + FromStr
    + FromF32
    + AsF32
{
    /// Will return the identity element. It is unnecessary as it will be the same as the defined
    /// [`Zero`] element. But for the sake of maintaining the mathematical notation of the definition, it is written.
    fn identity() -> Self;

    /// Will return the inverse element of the current element. Also might seem unneded because the
    /// trait [`Neg`] is already implemented, but it is necessary to maintain the mathematical notation of the definition.
    fn inverse(&self) -> Self;
    /// will return the result of the operation _·_ between the current element and the element passed as
    /// argument.
    fn op(&self, rhs: &Self) -> Self;
}

macro_rules! impl_group_for_primitives {
    ($($t:ty),*) => {
        $(impl Group for $t {
            fn identity() -> Self {
                0 as $t
            }

            fn inverse(&self) -> Self {
                -*self
            }

            fn op(&self, rhs: &Self) -> Self {
                *self + *rhs
            }
        })*
    };
}

impl_group_for_primitives!(isize, i8, i16, i32, i64, i128);

/// Defines the necessary behavior of an element of a ring.
///
/// # Definition
///
/// A **ring** is a set _R_ equipped with two binary operations _+_ and _·_ satisfying the following axioms:
/// 1. _R_ is an abelian group under addition, meaning that:
///    1. It is closed under addition.
///    2. It is associative under addition.
///    3. There is an element _0_ in _R_, called the **additive identity** or **zero element**, such that _a_ + _0_ = _a_ for all _a_ in _R_.
///    4. For each _a_ in _R_, there exists an element _−a_ in _R_, called the **additive inverse** of _a_, such that _a_ + (−_a_) = 0.
/// 2. _R_ is a monoid under multiplication, meaning that:
///    1. It is closed under multiplication.
///    2. It is associative under multiplication.
///    3. There is an element _1_ in _R_, called the **multiplicative identity** or **unity**, such that _a_ · 1 = _a_ and 1 · _a_ = _a_ for all _a_ in _R_.
/// 3. Multiplication is distributive with respect to addition:
///    1. _a_ · (_b_ + _c_) = (_a_ · _b_) + (_a_ · _c_) for all _a_, _b_, _c_ in _R_ (left distributivity).
///    2. (_a_ + _b_) · _c_ = (_a_ · _c_) + (_b_ · _c_) for all _a_, _b_, _c_ in _R_ (right distributivity).
///
/// # Implementation
///
/// ## Trait bounds
///
/// 1. The trait [`Mul`] is used to define the binary operation _·_. It is usually associated with a product. The binary operation _+_ is
/// implicitly defined by the trait [`Group`].
/// 3. The trait [`Rem`] is used to define the behaviour of the division in a ring.
/// 4. The trait [`Div`] is used to define the division in a ring.
/// 5. The trait [`One`] is used to define the **multiplicative identity**. It is usually associated with the multiplicative identity.
/// 6. All other traits are needed for the implementation of a generic numeric type.
///
/// ## Methods
///
/// 1. The method [`Ring::sum`] will return the result of the operation _+_ between the current element and the element passed as
/// argument.
/// 2. The method [`Ring::mul`] will return the result of the operation _·_ between the current element and the element passed as
/// argument.
/// 3. The method [`Ring::inverse_addition`] will return the **additive inverse** of the current element.
///
/// # Example
///
/// Let's write the implementation of a ring for the [`isize`] type.
/// ```ignore
/// use crate::math_rs::structures::Ring;
/// impl Ring for isize {
///     fn sum(&self, rhs: &Self) -> Self {
///        *self + *rhs
///     }
///     fn mul(&self, rhs: &Self) -> Self {
///        *self * *rhs
///     }
///     fn inverse_addition(&self) -> Self {
///       -*self
///     }
/// }
/// ```
///
/// # References
///
/// 1. [Wikipedia](https://en.wikipedia.org/wiki/Ring_(mathematics))
/// 2. [MathWorld](https://mathworld.wolfram.com/Ring.html)
/// 3. [ProofWiki](https://proofwiki.org/wiki/Definition:Ring_(Abstract_Algebra))
/// 4. [PlanetMath](https://planetmath.org/definitionofaring)
pub trait Ring: Group + Mul<Output = Self> + Rem<Output = Self> + One + Div<Output = Self> {
    /// Will return the result of the operation _+_ between the current element and the element passed as
    /// argument.
    fn sum(&self, rhs: &Self) -> Self;

    /// Will return the result of the operation _·_ between the current element and the element passed as
    /// argument.
    fn mul(&self, rhs: &Self) -> Self;

    /// Will return the **additive inverse** of the current element.
    fn inverse_addition(&self) -> Self {
        Self::inverse(&self)
    }
}

macro_rules! impl_ring_for_primitives {
    ($($t:ty),*) => {
        $(impl Ring for $t {

            fn sum(&self, rhs: &Self) -> Self {
                *self + *rhs
            }

            fn mul(&self, rhs: &Self) -> Self {
                *self * *rhs
            }

            fn inverse_addition(&self) -> Self {
                -*self
            }
        })*
    };
}

impl_ring_for_primitives!(isize, i8, i16, i32, i64, i128);

/// Defines the necessary behavior of an element of a field.
///
/// # Definition
///
/// A **field** is a set _F_ equipped with two binary operations _+_ and _·_ satisfying the following axioms:
/// 1. _F_ is an abelian group under addition, meaning that:
///   1. It is closed under addition.
///   2. It is associative under addition.
///   3. There is an element _0_ in _F_, called the **additive identity** or **zero element**, such that _a_ + _0_ = _a_ for all _a_ in _F_.
///   4. For each _a_ in _F_, there exists an element _−a_ in _F_, called the **additive inverse** of _a_, such that _a_ + (−_a_) = 0.
/// 2. _F_ is an abelian group under multiplication, meaning that:
///   1. It is closed under multiplication.
///   2. It is associative under multiplication.
///   3. There is an element _1_ in _F_, called the **multiplicative identity** or **unity**, such that _a_ · 1 = _a_ and 1 · _a_ = _a_ for all _a_ in _F_.
///   4. For each _a_ ≠ 0 in _F_, there exists an element _a_<sup>−1</sup> in _F_, called the **multiplicative inverse** of _a_, such that _a_ · _a_<sup>−1</sup> = 1.
/// 3. Multiplication is distributive with respect to addition:
///   1. _a_ · (_b_ + _c_) = (_a_ · _b_) + (_a_ · _c_) for all _a_, _b_, _c_ in _F_ (left distributivity).
///   2. (_a_ + _b_) · _c_ = (_a_ · _c_) + (_b_ · _c_) for all _a_, _b_, _c_ in _F_ (right distributivity).
///
/// # Implementation
///
/// ## Trait bounds
///
/// 1. The trait [`Ring`] is used to englobe the necessary traits for the implementation of a field.
/// 2. The trait [`Div`] is used to define the division in a field.
///
/// ## Methods
///
/// 1. The method [`Field::inverse_multiplication`] will return the **multiplicative inverse** of the current element.
///
/// # Example
///
/// To be done...
pub trait Field: Ring + Div {
    fn inverse_multiplication(&self) -> Self;
}
