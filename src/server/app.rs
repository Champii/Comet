use crate::prelude::{App, Component};

impl<Comp, Msg> App<Comp, Msg>
where
    Comp: Component<Msg>,
    Msg: Clone,
{
    pub fn run(&mut self) {
        println!("Running");
        // TODO:
        // setup db
        // run static http server
        // run websocket server
    }
}
