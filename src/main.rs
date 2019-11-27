
use std::time::Duration;
use tokio::prelude::*;
use tokio::time;
use futures::future::{join, join3};

#[tokio::main]
async fn main() {
    join3(print_letter_a(), print_letter_b(), print_letter_c()).await;
}

async fn print_letter_a() {
    time::delay_for(Duration::from_secs(4)).await;
    println!("A");
}

async fn print_letter_b() {
    time::delay_for(Duration::from_secs(2)).await;
    println!("B");

}

async fn print_letter_c() {
    time::delay_for(Duration::from_secs(1)).await;
    println!("C");
}