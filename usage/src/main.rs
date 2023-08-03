use macros::benchmark;

#[benchmark]
fn hello_world() {
    println!("5 memes");
}

fn main() {
    hello_world();
}
