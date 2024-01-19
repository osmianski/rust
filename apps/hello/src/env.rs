pub fn load(path: &str) {
    if let Ok(contents) = std::fs::read_to_string(path) {
        load_contents(contents);
    }
}

enum State {
    Newline,
    Comment,
    Name(usize),
    Value(usize, usize),
}

fn load_contents(contents: String) {
    let mut state = State::Newline;
    let mut offset: usize = 0;

    for c in contents.chars() {
        match state {
            State::Newline => {
                match c {
                    '\r' | '\n' => {},
                    '#' => state = State::Comment,
                    _ => state = State::Name(offset),
                }
            },
            State::Comment => {
                match c {
                    '\n' => state = State::Newline,
                    _ => {},
                }
            },
            State::Name(name_offset) => {
                match c {
                    '=' => state = State::Value(name_offset, offset),
                    '\r' | '\n' => state = State::Newline,
                    _ => {},
                }
            },
            State::Value(name_offset, eq_offset) => {
                match c {
                    '\r' => {},
                     '\n' => {
                        state = State::Newline;

                        let name = &contents[name_offset..eq_offset];
                        let value = &contents[eq_offset + 1..offset];

                        std::env::set_var(name, value);
                    },
                    _ => {},
                }
            },
            
        }

        offset += c.len_utf8();
    }

    match state {
        State::Value(name_offset, eq_offset) => {
            let name = &contents[name_offset..eq_offset];
            let value = &contents[eq_offset + 1..offset];

            std::env::set_var(name, value);
        },
        _ => {},
    }
}