extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod sync;
pub mod responses;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
