use metrics_exploration::{do_stuff, log_exporter::LogRecorder};

fn main() {
    LogRecorder::init();

    do_stuff();
    do_stuff();
    do_stuff();
    do_stuff();
    do_stuff();
    do_stuff();

    println!("Hello, world!");
}
