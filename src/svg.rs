// pub enum SVG {
//     X,
//     O,
//     Empty,
// }

pub struct SVG {
    pub body: String,
}

pub struct SVGTemplate {
}

impl SVG {

    pub fn new(body: &str) -> SVG {
        // let state = match str_state {
        //     "x" => State::X,
        //     "o" => State::O,
        //     _ => State::Empty,
        // };
        SVG { body }
    }

    pub fn new_from_template(template: SVGTemplate) -> SVG {
        SVG { body: String::from("") }
    }

    pub fn render(&self) -> String {
        "svg"
    }

    // pub fn serialize(&self) -> String {
    //     String::from(match self.state {
    //         State::X => "x",
    //         State::O => "o",
    //         State::Empty => "",
    //     })
    // }
}
