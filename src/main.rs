extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello World!")))
    }).http("localhost:3000").unwrap();
}


#[cfg(test)]
mod test {
    #[test]
    fn green_test() {
        assert!(true);
    }

    #[test]
    #[should_panic]
    fn should_panic() {
        assert!(false);
    }
}
