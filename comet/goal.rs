use comet::prelude::*;

pub enum UserMsg {
    Click,
}

#[derive(Db)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: u8,
}

impl Component<UserMsg> for User {
    fn update(&mut self, msg: UserMsg) {
        match msg {
            UserMsg::Click => {
                self.age += 1;
            }
        }
    }

    fn view(&self) -> Element<UserMsg> {
        html! {
            div {
                span {
                    {{ self.name }}
                },
                button
                    @click: UserMsg::Click, {
                    {{ self.age }}
                }
            }
        }
    }
}

pub enum Msg {
    UserDetail(i32),
}

pub struct Main {
    show_user: Option<i32>,
}

impl Main {
    pub fn new() -> Self {
        Self { show_user: None }
    }
}

impl Component<Msg> for Main {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::UserDetail(user_id) => self.show_user = Some(user_id),
        }
    }

    fn view(&self) -> Element<Msg> {
        html! {
            div {
                ul {
                    @for user in User::all() {
                        li {
                            a
                                click: Msg::UserDetail(user.id) {
                                @user.name
                            },
                            @if let Some(user_id) = self.show_user && user_id == user.id {
                                @user
                            }
                        }
                    }
                },
            }
        }
    }
}

fn main() {
    let mut app = App::new(Main::new());

    app.run();
}
