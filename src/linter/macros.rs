macro_rules! invalid_span {
    ($check:ident,$rule:path,$span:path) => {
        #[derive(Default)]
        pub struct $check {
            spans: Vec<Span>,
        }

        impl Check for $check {
            fn rule(&self) -> Rule {
                $rule
            }

            fn spans(&self) -> &[Span] {
                self.spans.as_slice()
            }

            fn visit_invalid_span(&mut self, span: &parsed::InvalidSpan) {
                if let $span(s) = span {
                    self.spans.push(*s);
                }
            }
        }
    };
}
