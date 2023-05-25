use metrics::counter;

pub fn do_stuff() {
    counter!("dogs.frobnicated", 13);
}
