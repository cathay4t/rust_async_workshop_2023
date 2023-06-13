// SPDX-License-Identifier: Apache-2.0

// use tokio::time::sleep;
use std::thread::sleep;
use std::time::Duration;

use futures::future::join;

async fn foo() {
    // sleep(Duration::from_secs(3)).await;
    sleep(Duration::from_secs(3));
    println!("foo() slept 3");
}

async fn bar() {
    // sleep(Duration::from_secs(1)).await;
    sleep(Duration::from_secs(1));
    println!("bar() slept 1");
}

#[tokio::main]
async fn main() {
    join(foo(), bar()).await;
}
