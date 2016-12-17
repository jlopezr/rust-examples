extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::{Router};

fn main() {
   let mut router = Router::new();
   router.get("/", hello_world, "hello");
   router.get("/:query", handler, "query");

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World from Iron!")))
    }

   fn handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>()
            .unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }

    println!("Listening on port 3000");
    Iron::new(router).http("localhost:3000").unwrap();
}
