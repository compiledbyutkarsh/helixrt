use tokio::sync::mpsc;

#[derive(Clone)]
pub enum RuntimeCommand {
    Shutdown,
    Pause,
    Resume,
}

pub struct CommandBus {
    sender: mpsc::Sender<RuntimeCommand>,
}

impl CommandBus {
    pub fn new(
        sender: mpsc::Sender<RuntimeCommand>,
    ) -> Self {
        Self { sender }
    }

    pub async fn dispatch(
        &self,
        command: RuntimeCommand,
    ) {
        let _ = self.sender.send(command).await;
    }
}