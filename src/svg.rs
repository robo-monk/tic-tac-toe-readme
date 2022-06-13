// pub enum SVG {
//     X,
//     O,
//     Empty,
// }

pub struct SVG {
    pub body: String,
}
// pub struct SVGTemplate {
//     pub o: String,
// }

pub enum SVGTemplate {
    O, Empty, X
}

impl SVG {

    pub fn new(body: &str) -> SVG {
        // let state = match str_state {
        //     "x" => State::X,
        //     "o" => State::O,
        //     _ => State::Empty,
        // };
        SVG { body: body.to_string() }
    }

    pub fn new_from_template(template: SVGTemplate) -> SVG {
        let body = match template {
            SVGTemplate::O => "<svg width=\"86\" height=\"91\" viewBox=\"0 0 86 91\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M85.1705 45.3636C85.1705 55.0795 83.2813 63.2756 79.5028 69.9517C75.7244 76.5994 70.625 81.642 64.2045 85.0795C57.7841 88.4886 50.625 90.1932 42.7273 90.1932C34.7727 90.1932 27.5852 88.4744 21.1648 85.0369C14.7727 81.571 9.6875 76.5142 5.90909 69.8665C2.15909 63.1903 0.284091 55.0227 0.284091 45.3636C0.284091 35.6477 2.15909 27.4659 5.90909 20.8182C9.6875 14.142 14.7727 9.09943 21.1648 5.69034C27.5852 2.25284 34.7727 0.534088 42.7273 0.534088C50.625 0.534088 57.7841 2.25284 64.2045 5.69034C70.625 9.09943 75.7244 14.142 79.5028 20.8182C83.2813 27.4659 85.1705 35.6477 85.1705 45.3636ZM60.7955 45.3636C60.7955 40.1364 60.0994 35.733 58.7074 32.1534C57.3438 28.5455 55.3125 25.8182 52.6136 23.9716C49.9432 22.0966 46.6477 21.1591 42.7273 21.1591C38.8068 21.1591 35.4972 22.0966 32.7983 23.9716C30.1278 25.8182 28.0966 28.5455 26.7045 32.1534C25.3409 35.733 24.6591 40.1364 24.6591 45.3636C24.6591 50.5909 25.3409 55.0085 26.7045 58.6165C28.0966 62.196 30.1278 64.9233 32.7983 66.7983C35.4972 68.6449 38.8068 69.5682 42.7273 69.5682C46.6477 69.5682 49.9432 68.6449 52.6136 66.7983C55.3125 64.9233 57.3438 62.196 58.7074 58.6165C60.0994 55.0085 60.7955 50.5909 60.7955 45.3636Z\" fill=\"#FF7A00\"/></svg>",
            SVGTemplate::X => "<svg width=\"85\" height=\"88\" viewBox=\"0 0 85 88\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M28.0227 0.72727L42.3409 26.125H43.0227L57.5114 0.72727H83.9318L57.8523 44.3636L84.9545 88H57.8523L43.0227 62.0909H42.3409L27.5114 88H0.579546L27.3409 44.3636L1.43182 0.72727H28.0227Z\" fill=\"#24FF00\"/></svg>",
            _ => "<svg width=\"0\" height=\"0\" viewBox=\"0 0 0 0\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\"></svg>"
        };

        SVG { body: body.to_string() }
        // SVG { body: String::from("") }
    }

    pub fn render(&self) -> String {
        return String::from(&self.body);
    }

    // pub fn serialize(&self) -> String {
    //     String::from(match self.state {
    //         State::X => "x",
    //         State::O => "o",
    //         State::Empty => "",
    //     })
    // }
}
