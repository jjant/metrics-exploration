pub mod log_exporter;

use metrics::counter;

pub fn do_stuff() {
    counter!("dogs.frobnicated", 1);
}
