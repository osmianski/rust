pub struct App {
}

pub struct Cli<'a> {
    pub app: &'a App,
}

pub struct Http<'a> {
    pub app: &'a App,
}

impl App {
    pub fn new() -> App {
        App {}
    }

    pub fn test() -> App {
        App {}
    }
}