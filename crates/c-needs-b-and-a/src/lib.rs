#![allow(dead_code)]
#[cfg(any(feature = "tests", test))]
use arbtest::arbitrary;

#[cfg_attr(any(feature = "tests", test), derive(arbitrary::Arbitrary))]
pub struct C {
    a: a::A,
    b: b::B
}

#[cfg(test)]
mod prop_test {
    use super::*;

    #[test]
    fn does_not_crash() {
        arbtest::builder()
            .budget_ms(1_000)
            .run(|u| {
                let _a: a::A = u.arbitrary()?;
                let _b: b::B = u.arbitrary()?;
                let _c: C = u.arbitrary()?;
                Ok(())
            })
    }
}