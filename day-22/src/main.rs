use sim::State;

mod search;
mod sim;

fn main() {
    let start = State::start();
    let ans = search::shortest_path(start);
    dbg!(ans);
}
