use comet::prelude::*;

#[model]
pub struct Message {
    pub channel_id: i32,
    pub sender: String,
    pub content: String,
}

#[sql]
impl Message {
    #[watch]
    pub async fn list_by_channel(channel_id: i32) -> Vec<Message> {
        use crate::schema::messages;

        messages::table
            .select(messages::all_columns)
            .filter(messages::channel_id.eq(channel_id))
    }
}

component! {
    Message {
        div {
            self.sender.to_owned() + ": " + &self.content
        }
    }
}

#[model]
pub struct Channel {
    pub name: String,
}

component! {
    Channel {
        h1 {
            self.name
        }
    }
}

#[derive(Default)]
pub struct ChannelView {
    pub channel_id: i32,
    pub sender: String,
    pub content: String,
}

impl ChannelView {
    async fn send_message(&mut self) {
        let mut message = Message {
            id: -1,
            channel_id: self.channel_id,
            sender: self.sender.clone(),
            content: self.content.clone(),
        };

        self.content = "".into();

        message.save().await.unwrap();
    }
}

component! {
    ChannelView {
        div {
            Channel::fetch(self.channel_id).await.unwrap()
            Message::list_by_channel(self.channel_id).await
            input bind: self.sender {}
            input bind: self.content {}
            button click: self.send_message().await {
                "Send"
            }
        }
    }
}

#[derive(Default)]
pub struct App {
    pub channel_name: String,
    pub channel_id: i32,
    pub current_channel: Option<Shared<ChannelView>>,
}

impl App {
    async fn new_channel(&mut self) {
        let mut channel = Channel {
            id: -1,
            name: self.channel_name.clone(),
        };

        self.channel_name = "".into();

        channel.save().await.unwrap();
    }

    pub fn update_current_channel(&mut self) {
        if self.channel_id > 0 {
            self.current_channel = Some(Shared::from(ChannelView {
                channel_id: self.channel_id,
                ..Default::default()
            }));
        } else {
            self.current_channel = None;
        }
    }
}

component! {
    App {
        div {
            self.current_channel
            select
              bind: self.channel_id
              change: self.update_current_channel() {
                option value: 0 {
                    "Select a channel"
                }
                for chan in Channel::list().await.unwrap() {
                    option value: chan.id {
                        chan.name
                    }
                }
            }
            input bind: self.channel_name {}
            button click: self.new_channel().await {
                "New Channel"
            }
        }
    }
}

comet::run!(App::default());
