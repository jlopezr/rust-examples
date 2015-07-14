extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::{Router};

fn main() {
   let mut router = Router::new();
   router.get("/", hello_world);
   router.get("/:query", handler);
   
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    }

   fn handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>()
            .unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }

    Iron::new(router).http("localhost:3000").unwrap();
    println!("On 3000");
}
