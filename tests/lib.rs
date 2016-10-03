#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        extern crate catfact;

        println!("{}", catfact::fact("1"));
    }
}
