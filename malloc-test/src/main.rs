use std::sync::mpsc;
use std::thread;

static NUM_PRODUCER_CONSUMER_PAIRS: usize = 16;
static CHANNEL_BUFFER_SIZE: usize = 10;
static MESSGE_TEXT_LENGTH: usize = 50 * 1024;
static PRINT_RECEIVED_MESSAGES: bool = false;

#[derive(Debug)]
struct Message {
    text: String,
}

fn build_message() -> Message {
    let mut s = String::with_capacity(MESSGE_TEXT_LENGTH);

    for _i in 0..MESSGE_TEXT_LENGTH {
        s.push('X');
    }

    Message { text: s }
}

fn main() {
    println!("begin main");
    println!("NUM_PRODUCER_CONSUMER_PAIRS = {NUM_PRODUCER_CONSUMER_PAIRS}");
    println!("CHANNEL_BUFFER_SIZE = {CHANNEL_BUFFER_SIZE}");
    println!("MESSGE_TEXT_LENGTH = {MESSGE_TEXT_LENGTH}");
    println!("PRINT_RECEIVED_MESSAGES = {PRINT_RECEIVED_MESSAGES}");

    let mut children = Vec::with_capacity(NUM_PRODUCER_CONSUMER_PAIRS * 2);

    for id in 0..NUM_PRODUCER_CONSUMER_PAIRS {
        // channel buffer size 1
        let (tx, rx) = mpsc::sync_channel(CHANNEL_BUFFER_SIZE);

        // Each thread will send its id via the channel
        let sender = thread::spawn(move || {
            println!("start sender id = {id}");
            loop {
                tx.send(build_message()).unwrap();
            }
        });

        children.push(sender);

        // Each thread will send its id via the channel
        let receiver = thread::spawn(move || {
            println!("start receiver id = {id}");
            loop {
                let message = rx.recv().unwrap();
                if PRINT_RECEIVED_MESSAGES {
                    println!("child received message: {}", message.text);
                }
            }
        });

        children.push(receiver);
    }

    println!("main started all threads children.len {}", children.len());

    // Wait for the threads to complete any remaining work
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    println!("end main");
}
