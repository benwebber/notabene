macro_rules! invalid_span {
    ($check:ident) => {
        #[derive(Default)]
        pub struct $check;

        impl Check for $check {
            fn rule(&self) -> Rule {
                Rule::$check
            }

            fn visit_invalid_span(&mut self, context: &mut Context, span: &parsed::InvalidSpan) {
                if let parsed::InvalidSpan::$check(s) = span {
                    context.report(self.rule(), Some(*s));
                }
            }
        }
    };
}
