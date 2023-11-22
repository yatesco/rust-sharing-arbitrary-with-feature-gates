#![allow(dead_code)]
#[cfg(any(feature = "tests", test))]
use arbtest::arbitrary;

#[cfg_attr(any(feature = "tests", test), derive(arbitrary::Arbitrary))]
pub struct B {
    a: a::A
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
                let _b: B = u.arbitrary()?;
                Ok(())
            })
    }
}