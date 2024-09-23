use super::custom_ring;
use super::FieldElement;
use super::RingElement;

custom_ring!(FoiFieldElement, 18446744069414584321, "oxfoi");

impl FieldElement for FoiFieldElement {}

impl std::ops::Div for FoiFieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self * other.inv().unwrap()
    }
}
