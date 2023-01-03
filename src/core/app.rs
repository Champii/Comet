use crate::prelude::*;

#[derive(Debug)]
pub struct App<Comp, Msg>
where
    Comp: Component<Msg>,
    Msg: Clone + 'static,
{
    pub root: Shared<Comp>,
    phantom: std::marker::PhantomData<Msg>,
}

impl<Comp, Msg> App<Comp, Msg>
where
    Comp: Component<Msg>,
    Msg: Clone,
{
    pub fn new(root: Shared<Comp>) -> Self {
        Self {
            root,
            phantom: std::marker::PhantomData,
        }
    }
}
