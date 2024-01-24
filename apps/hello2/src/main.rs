mod context;

fn main() {
    let _app = context::App::new();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let _app = context::App::test();
    }
}
