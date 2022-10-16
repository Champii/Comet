#[macro_export]
macro_rules! html_arr {
    ({ $code:tt }) => {
        vec![Element::Text($code.to_string())]
    };

    ($( $ident:tt $([$($attr_name:ident : $attr_value:expr),*])? $($(@$ev:ident : $evcode:expr ),+, )? $({ $($e:tt)+ })? ),*) => {{
        use std::collections::BTreeMap;
        vec![$(
            {
            let elem_str = stringify!($ident);

            if elem_str.starts_with("\"") {
                Element::Text(elem_str.to_string())
            } else {
                #[allow(unused_mut, unused_assignments)]
                let mut children = vec![];

                $(
                    children = html_arr!($($e)+);
                )?

                #[allow(unused_mut, unused_assignments)]
                let mut attrs = BTreeMap::new();

                $(
                    attrs = [$((stringify!($attr_name).to_string(), $attr_value.to_string())),*].into();
                )?

                #[allow(unused_mut, unused_assignments)]
                let mut evcode = BTreeMap::new();

                $(
                    evcode = [$((stringify!($ev).into(), $evcode)),*].into();
                )?

                Element::Node {
                    tag: elem_str.to_string(),
                    attrs,
                    events: evcode,
                    children: children,
                }

            }
            }

        ),*]
    }};
}

#[macro_export]
macro_rules! html {
    ($ident:tt $([$($attr_name:ident : $attr_value:expr),*])? $($(@$ev:ident : $evcode:expr ),+, )? $({ $($e:tt)+ })? ) => {{
        html_arr!($ident $([$($attr_name : $attr_value),*])? $($(@$ev : $evcode),+,)? $({ $($e)+ })?).get(0).unwrap().clone()
    }};
}

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod test {
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Debug, Clone)]
    enum Msg {
        Increment,
    }

    #[wasm_bindgen_test]
    fn test_html() {
        use crate::element::Element;
        use crate::renderable::Renderable;

        let elem = html!(div [height: 100] {
            button
                [style: "background-color: red;"]
                @click: Msg::Increment, {
                {{ 2 }}
            }
        });

        assert_eq!(
            elem.render().outer_html(),
            r#"<div height="100"><button style="background-color: red;"><span>2</span></button></div>"# // r#"<div height="100"><button style="background-color: red;" onclick="Increment">2</button></div>"#
        );
    }
}
