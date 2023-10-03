use crate::{equality::Equals, structures::Ring};

use super::SquareMatrix;

impl<R: Ring> Equals for SquareMatrix<R> {
    fn equals(&self, rhs: &Self, tolerance: f32) -> bool {
        if self.dimension() != rhs.dimension() {
            return false;
        }
        self.data
            .iter()
            .zip(rhs.data.iter())
            .all(|(row, other_row)| {
                row.iter()
                    .zip(other_row.iter())
                    .all(|(element, other_element)| element.equals(other_element, tolerance))
            })
    }
}
