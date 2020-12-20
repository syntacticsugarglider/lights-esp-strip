use futures::{pin_mut, StreamExt};
use lights_esp_strip::listen;

fn main() {
    smol::block_on(async move {
        let stream = listen(5000);
        pin_mut!(stream);
        while let Some(Ok(mut light)) = stream.next().await {
            light.set_color((255, 0, 0)).await.unwrap();
        }
    });
}
