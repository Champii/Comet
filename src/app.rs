use crate::prelude::*;
use std::fmt::Debug;

// #[derive(Debug)]
pub struct App<Compo, Msg>
where
    Compo: Component<Msg>,
    Msg: Debug,
{
    pub root: Compo,
    phantom: std::marker::PhantomData<Msg>,
}

impl<Compo, Msg> App<Compo, Msg>
where
    Compo: Component<Msg>,
    Msg: Debug,
{
    pub fn new(root: Compo) -> Self {
        Self {
            root,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn run(&mut self) {
        println!("{:#?}", self.root.view());
        println!("{}", self.root.view().render());
    }
}
