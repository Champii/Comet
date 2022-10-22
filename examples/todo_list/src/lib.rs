use comet::prelude::*;

#[derive(Default)]
pub struct Todo {
    pub title: String,
    pub completed: bool,
}

component! {
    Todo,
    div {
        { self.title }
        { self.completed }
        button @click: { self.completed = !self.completed } {
            { "Toggle" }
        }
    }
}

#[derive(Default)]
pub struct App {
    title: String,
    list: Vec<Shared<Todo>>,
}

impl App {
    pub fn new() -> Self {
        let mut list = Vec::new();

        list.push(
            Todo {
                title: "Hello".into(),
                completed: false,
            }
            .into(),
        );

        Self {
            list,
            ..Default::default()
        }
    }

    pub fn new_todo(&mut self) {
        self.list.push(
            Todo {
                title: self.title.clone(),
                completed: false,
            }
            .into(),
        );

        self.title = "".into();
    }
}

component! {
    App,
    div {
        for ((todo) in self.list.iter()) {
            div {
                @{todo}
            }
        }
        input {} // todo
        button @click: { self.new_todo() } {
            { "Add" }
        }
    }
}

comet!(App::new());
