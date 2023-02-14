use comet::prelude::*;

#[model]
#[derive(Default, Debug)]
pub struct Todo {
    pub title: String,
    pub completed: bool,
}

impl Todo {
    pub async fn new(title: &str) -> Self {
        Self {
            id: -1,
            title: title.into(),
            completed: false,
        }
        .create()
        .await
        .unwrap()
    }
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

        todos::table.order(todos::id)
    }
}

component! {
    Todo {
        div {
            self.id
            self.title.clone()
            self.completed.to_string()
            button click: self.toggle().await {
                "Toggle"
            }
            button click: Todo::delete(self.id).await.unwrap() {
                "Delete"
            }
        }
    }
}

#[derive(Default)]
pub struct App {
    title: String,
}

impl App {
    pub async fn new_todo(&mut self) {
        Todo::new(&self.title).await;

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
            input value: self.title.clone() {}
            self.title
        }
    }
}

comet::run!(App::default());
