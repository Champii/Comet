use std::fmt::Debug;

mod app;
mod component;
mod element;

#[macro_use]
pub mod html_macro;

pub mod prelude;
mod renderable;

#[cfg(target_arch = "wasm32")]
#[macro_use]
pub mod wasm;

/* #[cfg(not(target_arch = "wasm32"))]
mod server; */

use prelude::*;

pub fn run<Comp, Msg>(root: Comp)
where
    Comp: Component<Msg>,
    Msg: Debug,
{
    App::new(root).run();
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod test {
    #[derive(Debug, Clone)]
    enum Msg {
        Increment,
    }

    #[test]
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

        assert_eq!(elem.render(), (),);
    }
}
