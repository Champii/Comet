#[macro_export]
macro_rules! html {
    ({ $code:tt }) => {
        Box::new(Element::Text($code.to_string()))
    };

    ($ident:tt $($(@$ev:ident : $evcode:expr ),+, )? $({ $($e:tt)* })?) => {{
        use std::collections::BTreeMap;

        let elem_str = stringify!($ident);

        if elem_str.starts_with("\"") {
            Element::Text(elem_str.to_string())
        } else {
            #[allow(unused_mut, unused_assignments)]
            let mut children = vec![];

            $(
                children = vec![html!($($e)*)];
            )?

            #[allow(unused_mut, unused_assignments)]
            let mut evcode = BTreeMap::new();

            $(
                evcode = [$((stringify!($ev).into(), $evcode)),*].into();
            )?

            Element::Node {
                tag: elem_str.to_string(),
                events: evcode,
                children: children,
            }
        }
    }};
}
