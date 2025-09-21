macro_rules! invalid_span {
    ($check:ident) => {
        #[derive(Default)]
        pub struct $check {
            spans: Vec<Span>,
        }

        impl Check for $check {
            fn rule(&self) -> Rule {
                Rule::$check
            }

            fn spans(&self) -> &[Span] {
                self.spans.as_slice()
            }

            fn visit_invalid_span(&mut self, span: &parsed::InvalidSpan) {
                if let parsed::InvalidSpan::$check(s) = span {
                    self.spans.push(*s);
                }
            }
        }
    };
}
