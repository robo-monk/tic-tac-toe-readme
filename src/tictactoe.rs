pub enum State {
    X,
    O,
    Empty,
}

pub struct Cell {
    pub state: State,
}

impl Cell {
    pub fn new(str_state: &str) -> Cell {
        let state = match str_state {
            "x" => State::X,
            "o" => State::O,
            _ => State::Empty,
        };
        Cell { state }
    }

    pub fn serialize(&self) -> String {
        String::from(match self.state {
            State::X => "x",
            State::O => "o",
            State::Empty => "",
        })
    }
}

// trait Cell {
//     state: State;
//     pub fn serialize(self) -> ();
// }
// impl State {

// }
