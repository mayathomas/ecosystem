use std::{
    thread::{self},
    time::Duration,
};

use tokio::{fs, runtime::Builder, time::sleep};

fn main() {
    let handle = thread::spawn(|| {
        //execute future
        let rt = Builder::new_current_thread().enable_all().build().unwrap();
        rt.spawn(async {
            println!("Future 1");
            // io读取文件时，runtime会找其他available future执行
            let content = fs::read("Cargo.toml").await.unwrap();
            println!("Content-length: {}", content.len());
        });

        rt.spawn(async {
            println!("Future 2");
            let ret = expensive_block_task("Future 2".to_string());
            println!("result: {}", ret);
        });

        // block_on会立即执行当前future，但由于sleep，它会被放入wait queue，再去找其他available future执行
        rt.block_on(async {
            println!("Future 3");
            sleep(Duration::from_millis(900)).await
        });
    });
    handle.join().unwrap();
}

fn expensive_block_task(s: String) -> String {
    thread::sleep(Duration::from_millis(800));
    blake3::hash(s.as_bytes()).to_string()
}
