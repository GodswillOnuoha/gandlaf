fn main() {
    let result = run_app();
    println!("{result}");
}

fn run_app() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use crate::run_app;

    #[test]
    fn it_works() {
        assert_eq!(run_app(), "Hello, world!");
    }
}
