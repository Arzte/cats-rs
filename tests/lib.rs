#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        extern crate cat;

        println!("{}", cat::fact("1"));
    }
}
