mod client;
mod errors;
pub mod producer;
mod serde;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
