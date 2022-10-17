mod app;
mod log_macro;
pub mod prelude;
mod renderable;

#[cfg(test)]
mod test {

    use crate::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Clone)]
    enum Msg {
        Increment,
    }

    #[derive(Clone)]
    struct TestComponent;

    impl Component<Msg> for TestComponent {
        fn update(&mut self, msg: Msg) {
            match msg {
                Msg::Increment => {}
            }
        }

        fn view(&self) -> Element<Msg> {
            html! {
                div {
                    button
                        @click: Msg::Increment, {
                        "Increment"
                    }
                }
            }
        }
    }

    #[wasm_bindgen_test]
    fn test_html() {
        use crate::renderable::Renderable;

        let component = TestComponent;
        let view = component.view();

        assert_eq!(
            view.render().outer_html(),
            r#"<div height="100"><button style="background-color: red;"><span>2</span></button></div>"#
        ); // r#"<div height="100"><button style="background-color: red;" onclick="Increment">2</button></div>"#
           /* let elem = html!(div [height: 100] {
               button
                   [style: "background-color: red;"]
                   @click: Msg::Increment, {
                   {{ 2 }}
               }
           }); */

        /* assert_eq!(
            elem.render().outer_html(),
            r#"<div height="100"><button style="background-color: red;"><span>2</span></button></div>"# // r#"<div height="100"><button style="background-color: red;" onclick="Increment">2</button></div>"#
        ); */
    }
}
