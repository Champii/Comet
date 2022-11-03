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

#[sql]
impl Todo {
    #[watch]
    pub async fn list_watch() -> Vec<Todo> {
        use crate::schema::todos;

        todos::table.order(todos::id.desc())
    }
}

component! {
    Todo {
        div {
            self.title.clone()
            self.completed.to_string()
            button click: self.toggle().await {
                "Toggle"
            }
        }
    }
}

#[derive(Default)]
pub struct App {
    title: String,
}

impl App {
    pub async fn new() -> Self {
        Self { title: "".into() }
    }

    pub async fn new_todo(&mut self) {
        Todo {
            id: -1,
            title: self.title.clone(),
            completed: false,
        }
        .create()
        .await
        .unwrap();

        self.title = "".into();
    }
}

component! {
    App {
        div {
            Todo::list_watch().await
            button click: self.new_todo().await {
                "Add"
            }
        }
    }
}

comet::run!(App::new().await);
