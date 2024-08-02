use std::{thread, time::Duration};

use anyhow::Result;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel(32);
    let handle = workder(rx);
    tokio::spawn(async move {
        let mut i = 0;
        loop {
            i += 1;
            println!("Send task: {i}");
            tx.send(format!("task {i}")).await?;
        }
        #[allow(unreachable_code)]
        Ok::<(), anyhow::Error>(())
    });

    handle.join().unwrap();
    Ok(())
}

fn workder(mut rx: mpsc::Receiver<String>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        //在tokio上下文要使用recv.await，它是non-blocking的，这里是在线程中使用，要blocking_recv
        while let Some(s) = rx.blocking_recv() {
            println!("result: {}", expensive_block_task(s));
        }
    })
}

fn expensive_block_task(s: String) -> String {
    thread::sleep(Duration::from_millis(800));
    blake3::hash(s.as_bytes()).to_string()
}
