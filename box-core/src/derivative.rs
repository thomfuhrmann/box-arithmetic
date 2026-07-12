use malachite::{Natural, base::num::arithmetic::traits::SaturatingSub};

use crate::{BoxKind, BoxValue, MultinumBox, PolynumBox};

impl BoxValue<PolynumBox> {
    /// Derivative of a polynumber
    pub fn derivative(self) -> Self {
        let mut result = BoxValue::<PolynumBox>::new();
        result.kinds.push(self.get_kind(0));
        result.colors.push(self.get_color(0));
        result.multiplicities.push(self.get_multiplicity(0));
        result.lengths.push(1);

        let mut max_depth = 0;
        for mut child in self {
            let kind = child.get_kind(0);
            if kind == BoxKind::Num {
                let coeff = child.get_multiplicity(0);
                let exp = child.get_multiplicity(1);
                child.set_multiplicity(0, coeff * exp.clone());

                let new_exp = exp.saturating_sub(Natural::from(1_u32));

                if new_exp == 0 {
                    child.remove(1);
                    child.set_kind(0, BoxKind::Empty);
                    child.set_length(0, 1);
                } else {
                    child.set_multiplicity(1, new_exp);
                }

                let child_len = child.get_length(0);
                max_depth = max_depth.max(child_len);
                result.extend(child);
            }
        }

        if max_depth == 1 {
            result.set_kind(0, BoxKind::Num);
        }
        result
    }
}

impl BoxValue<MultinumBox> {
    /// Derivative of a multinumber
    pub fn derivative(self, index: impl Into<Natural>) -> Self {
        let index = index.into();

        let mut result = BoxValue::<MultinumBox>::new();
        result.kinds.push(self.get_kind(0));
        result.colors.push(self.get_color(0));
        result.multiplicities.push(self.get_multiplicity(0));
        result.lengths.push(1);

        let mut max_depth = 0;
        for mut child in self {
            let kind = child.get_kind(0);
            if kind == BoxKind::Polynum {
                let var_index = child.get_multiplicity(2);

                if var_index == index {
                    let coeff = child.get_multiplicity(0);
                    let exp = child.get_multiplicity(1);
                    child.set_multiplicity(0, coeff * exp.clone());
                    let new_exp = exp.saturating_sub(Natural::from(1_u32));
                    if new_exp == 0 {
                        child.remove(2);
                        child.remove(1);
                        child.set_kind(0, BoxKind::Empty);
                        child.set_length(0, 1);
                    } else {
                        child.set_multiplicity(1, new_exp);
                    }

                    let child_len = child.get_length(0);
                    max_depth = max_depth.max(child_len);
                    result.extend(child);
                }
            }
        }

        if max_depth == 1 {
            result.set_kind(0, BoxKind::Num);
        } else if max_depth == 0 {
            result.set_kind(0, BoxKind::Empty);
        }
        result
    }
}

#[cfg(test)]
mod tests {

    use crate::{BoxValue, PolynumBox};

    #[test]
    fn test_der_uni() {
        let poly = BoxValue::alpha() + 3_u32 * BoxValue::alpha() * BoxValue::alpha();
        let der = poly.derivative();
        let exp = BoxValue::from(1) + 6_u32 * BoxValue::alpha();
        assert_eq!(der, exp);

        let poly = 3_u32 * BoxValue::alpha();
        let der = poly.derivative();
        let exp = BoxValue::from(3).cast::<PolynumBox>();
        assert_eq!(der, exp);
    }

    #[test]
    fn test_der_multi() {
        let multi = 3_u32 * BoxValue::beta(2_u32) * BoxValue::beta(2_u32);
        let der = multi.clone().derivative(2_u32);
        let exp = 6 * BoxValue::beta(2_u32);
        assert_eq!(der, exp);

        let der = multi.clone().derivative(1_u32);
        let exp = BoxValue::zero();
        assert_eq!(der, exp.cast());

        let der = multi.derivative(2_u32);
        let der = der.derivative(2_u32);
        let exp = BoxValue::from(6);
        assert_eq!(der, exp.cast());
    }
}
