//! telegram model — messaging app.

#[derive(Debug, Clone)]
pub struct Chat {
    pub id: String,
    pub name: String,
    pub type_: String,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub id: String,
    pub chat: String,
    pub sender: String,
    pub body: String,
    pub read: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub chats: Vec<Chat>,
    pub messages: Vec<Message>,
    pub next_id: u64,
}

impl Default for Model {
    fn default() -> Self { Self { chats: vec![], messages: vec![], next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_chat(&mut self, chat: Chat) { self.chats.push(chat); }

    pub fn send(&mut self, chat: &str, sender: &str, body: &str) -> bool {
        if self.chats.iter().any(|c| c.id == chat) {
            let id = format!("{:x}", self.next_id);
            self.next_id += 1;
            self.messages.push(Message { id, chat: chat.into(), sender: sender.into(), body: body.into(), read: true });
            true
        } else { false }
    }

    pub fn list_messages(&self) -> &[Message] { &self.messages }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chat_send() {
        let mut t = Model::new();
        t.add_chat(Chat { id: "c1".into(), name: "Group".into(), type_: "group".into() });
        assert!(t.send("c1", "alice", "hi"));
        assert_eq!(t.list_messages().len(), 1);
    }
}
