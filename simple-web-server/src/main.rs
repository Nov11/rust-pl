extern crate iron;
extern crate router;
extern crate urlencoded;

#[macro_use]
extern crate mime;

use iron::prelude::*;
use iron::status;
use router::Router;
//not knowing how to import my crate..
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn main() {
    let mut router = Router::new();
    router.get("/", get_from, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("Serving on port 8090");
    Iron::new(router).http("localhost:8090").unwrap();
}

fn get_from(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
    <html>
    <head></head>
    <title>GCD calculater</title>
    <body>
    <form action = "/gcd" method="post">
    <input type="text" name="n"/>
    <input type="text" name="n"/>
    <button type="submit"> compute </button>
    </form>
    </body>
    </html>
    "#);

    Ok(response)
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            return bad(format!("error parsing param {:?}", e).as_str());
        }
        Ok(map) => map
    };

    let unparsed_number = match form_data.get("n") {
        None => {
            return bad(format!("from data has no 'n' param").as_str());
        }
        Some(value) => value
    };

    let mut numbers = Vec::new();
    for up in unparsed_number {
        match u64::from_str(&up) {
            Err(_) => {
                return bad(format!("param {:?} not a number", up).as_str());
            }
            Ok(n) => { numbers.push(n); }
        }
    }

    if numbers.len() != 2 {
        return bad("need 2 params");
    }
    let d = gcd(numbers[0], numbers[1]);
    good(d.to_string().as_str())
}

fn bad(s: &str) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::BadRequest);
    response.set_mut(s);
    return Ok(response);
}

fn good(s: &str) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html;Charset=Utf8));
    response.set_mut(s);
    return Ok(response);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a < b {
        let t = a;
        a = b;
        b = t;
    }

    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}
