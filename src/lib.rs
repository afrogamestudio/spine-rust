extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;

pub mod spine;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
