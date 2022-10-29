use comet::prelude::*;

#[model]
#[derive(Default, Debug)]
pub struct Todo {
    pub title: String,
    pub completed: bool,
}

#[rpc]
impl Todo {
    pub async fn toggle(&mut self) {
        self.completed = !self.completed;

        self.save().await.unwrap();
    }
}

component! {
    Todo {
        div {
            { self.title }
            { self.completed }
            button @click: { self.toggle().await } {
                { "Toggle" }
            }
        }
    }
}

#[derive(Default)]
pub struct App {
    title: String,
    list: Vec<Shared<Todo>>,
}

impl App {
    pub async fn new() -> Self {
        let list = Todo::list()
            .await
            .unwrap()
            .into_iter()
            .map(Shared::from)
            .collect::<Vec<_>>();

        Self {
            list,
            title: "".into(),
        }
    }

    pub async fn new_todo(&mut self) {
        self.list.push(
            Todo {
                id: -1,
                title: self.title.clone(),
                completed: false,
            }
            .create()
            .await
            .unwrap()
            .into(),
        );

        self.title = "".into();
    }
}

component! {
    App {
        div {
            for todo in (&self.list) {
                div {
                    @{todo}
                }
            }
            input ={ self.title } {}
            button @click: { self.new_todo().await } {
                { "Add" }
            }
        }
    }
}

comet::run!(App::new().await);
