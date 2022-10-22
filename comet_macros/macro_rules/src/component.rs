#[macro_export]
macro_rules! component {
    ($type:ty, $($e:tt)+) => {
        paste! {
            mod [<__component_ $type:lower>] {
                use super::*;

                extract_msg!{$($e)+}

                impl Component<Msg> for $type {
                    fn update(&mut self, msg: Msg) {
                        extract_update!{self, msg, $type, $($e)+}
                    }

                    fn view<F>(&self, f: F) -> web_sys::Element
                    where
                        F: Fn(Msg) + Clone + 'static
                    {
                        html! {self, f, $($e)+ }.into_element()
                    }
                }
            }
        }
    };
}
