use futures_util::SinkExt;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

pub type TelemetryChannel =
    broadcast::Sender<String>;

pub struct WebSocketServer;

impl WebSocketServer {
    pub async fn start(
        channel: TelemetryChannel,
    ) {
        let listener =
            TcpListener::bind(
                "127.0.0.1:9001"
            )
            .await
            .unwrap();

        println!(
            "websocket telemetry listening on 9001"
        );

        loop {
            let (stream, _) =
                listener.accept().await.unwrap();

            let mut receiver =
                channel.subscribe();

            tokio::spawn(async move {
                let mut ws_stream =
                    accept_async(stream)
                        .await
                        .unwrap();

                loop {
                    if let Ok(payload) =
                        receiver.recv().await
                    {
                        let _ =
                            ws_stream
                                .send(
                                    Message::Text(
                                        payload
                                            .into(),
                                    ),
                                )
                                .await;
                    }
                }
            });
        }
    }
}