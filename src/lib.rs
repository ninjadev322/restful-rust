pub mod routes;
mod schema;
mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn shall_pass() {
        assert_eq!(2 + 1, 3);
    }
}
