mod trigger;
// mod envelope_generator;
// mod oscillator;
// mod filter;
// mod mixer; 

use trigger::Trigger;

fn main() {
    let my_trigger = Trigger::new(60, 100);
    println!("{:?}", my_trigger);
}