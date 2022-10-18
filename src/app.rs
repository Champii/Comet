use std::{cell::RefCell, rc::Rc};

use crate::prelude::*;

pub struct App<Comp, Msg>
where
    Comp: Component<Msg>,
    Msg: Clone,
{
    pub root: Rc<RefCell<Comp>>,
    phantom: std::marker::PhantomData<Msg>,
}

impl<Comp, Msg> App<Comp, Msg>
where
    Comp: Component<Msg>,
    Msg: Clone,
{
    pub fn new(root: Rc<RefCell<Comp>>) -> Self {
        Self {
            root,
            phantom: std::marker::PhantomData,
        }
    }
}
