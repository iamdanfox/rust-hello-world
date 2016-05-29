extern crate iron;
extern crate bodyparser;

use iron::prelude::*;
use iron::status;

fn handler(req: &mut Request) -> IronResult<Response> {
    println!("Received request to url {}", req.url);

    let body = req.get::<bodyparser::Raw>();
    match body {
        Ok(Some(body)) => println!("Read body: '{}'", body),
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err)
    }

    Ok(Response::with((status::Ok, "Hello World!")))
}

fn main() {
    Iron::new(handler).http("localhost:3000").unwrap();
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
