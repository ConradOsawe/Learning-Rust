// if_let keyword
enum State {
    Idle,
    Running, //variants not used
    Waiting, //variants not used
}

fn print_state(state: State) {
    if let State::Idle = state {
        println!("The system is idle");
    }
}

fn main() {
    let state = State::Idle;
    print_state(state);
}

// ChatGPT generated code for clarification of matching
// fn print_state(state: State) {
//     match state {
//         State::Idle => println!("The system is idle"),
//         State::Running => println!("The system is running"),
//         State::Waiting => println!("The system is waiting"),
//     }
// }

// fn main() {
//     let state = State::Running; // Use a different variant
//     print_state(state);
// }
