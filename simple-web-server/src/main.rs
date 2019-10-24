extern crate iron;

#[macro_use]
extern crate mime;

use iron::prelude::*;
use iron::status;


fn main() {
    println!("Serving on port 8090");
    Iron::new(get_from).http("localhost:8090").unwrap();
}

fn get_from(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
    <title>GCD calculater</titl>
    <from action = "/gcd" method="post">
    <input type="text" name="n"/>
    <input type="text" name="n"/>
    <button type="submit"> compute </button>
    </form>"#);

    Ok(response)
}

