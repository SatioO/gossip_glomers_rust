use std::collections::HashSet;

#[derive(Debug, Default)]
pub(crate) struct Messages(pub(crate) HashSet<u64>);

#[derive(Debug, Default)]
pub(crate) struct Storage {
    pub(crate) messages: Messages,
}

impl Storage {
    pub(crate) fn add_message(&mut self, message: u64) {
        if !self.messages.0.contains(&message) {
            self.messages.0.insert(message);
        }
    }

    pub(crate) fn get_messages(&self) -> Vec<u64> {
        self.messages.0.iter().cloned().collect()
    }
}
