use crate::prelude::*;

pub struct App<Comp, Msg>
where
    Comp: Component<Msg>,
{
    pub root: Comp,
    phantom: std::marker::PhantomData<Msg>,
}

impl<Comp, Msg> App<Comp, Msg>
where
    Comp: Component<Msg>,
{
    pub fn new(root: Comp) -> Self {
        Self {
            root,
            phantom: std::marker::PhantomData,
        }
    }
}
