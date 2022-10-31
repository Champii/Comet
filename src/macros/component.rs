#[macro_export]
macro_rules! component {
    ($type:ty { $($e:tt)+ }) => {
        paste! {
            mod [<__component_ $type:lower>] {
                use super::*;

                extract_msg!{$($e)+}

                #[async_trait]
                impl Component<Msg> for $type {
                    fn update_bindings(&mut self, elems: Shared<Vec<web_sys::Element>>) {
                        extract_bindings!{self, elems, $($e)+}

                    }
                    async fn update(&mut self, msg: Msg) {
                        let this = &mut *self;
                        extract_update!{this, msg, $type, $($e)+}
                    }

                    fn view<F>(&self, f: F, bindings: Shared<Vec<web_sys::Element>>) -> web_sys::Element
                    where
                        F: Fn(Option<Msg>) + Clone + 'static
                    {
                        html! {self, f, bindings, $($e)+ }.into_element()
                    }
                }
            }
        }
    };
}
