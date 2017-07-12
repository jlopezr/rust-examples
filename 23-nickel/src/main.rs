// https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-nickelrs/

//#![feature(core_intrinsics)]

#[macro_use]
extern crate nickel;

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

// Nickel
use nickel::{Nickel, HttpRouter, MediaType, MiddlewareResult, Request, Response};
use nickel::status::StatusCode::{self, Forbidden};
use std::io::Read;

// Hyper
use hyper::header;
use hyper::header::{Authorization, Bearer};

// jwt
use std::default::Default;
use crypto::sha2::Sha256;
use jwt::{Header,Registered,Token};

// MongoDB
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::error::Result as MongoResult;

// bson
use bson::{Bson, Document};
use bson::oid::ObjectId;

// serde
extern crate serde;
extern crate serde_json;
use serde_json::{Value};

#[macro_use]
extern crate serde_derive;

extern crate jwt;
extern crate hyper;
extern crate crypto;

static AUTH_SECRET: &'static str = "some_secret_key";

#[derive(Serialize, Deserialize)]
struct User {
    firstname: String,
    lastname: String,
    email: String
}

#[derive(Serialize, Deserialize)]
struct UserLogin {
    email: String,
    password: String
}

fn main() {

    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/hello", middleware! { |_request, response|
        format!("Hello")
    });

    /*
    fn print_type_of<T>(_: &T) -> () {
        let type_name =
            unsafe {
                std::intrinsics::type_name::<T>()
            };
        println!("{}", type_name);
    }
    */

    router.post("/login", middleware! { |request|

        // Accept a JSON string that corresponds to the User struct
        //let user = request.json_as::<UserLogin>().unwrap();
        let mut body = String::new();
        request.origin.read_to_string(&mut body).unwrap();
        let user = serde_json::from_str::<UserLogin>(&body).unwrap();

        // Get the email and password
        let email = user.email.to_string();
        let password = user.password.to_string();

        // Simple password checker
        if password == "secret".to_string() {

            let header: Header = Default::default();

            // For the example, we just have one claim
            // You would also want iss, exp, iat etc
            let claims = Registered {
                sub: Some(email.into()),
                ..Default::default()
            };

            let token = Token::new(header, claims);

            // Sign the token
            let jwt = token.signed(AUTH_SECRET.as_bytes(), Sha256::new()).unwrap();

            format!("{}", jwt)

        } else {
            format!("Incorrect username or password")
        }

    });

    fn get_data_string(result: MongoResult<Document>) -> Result<Value, String> {
        match result {
            Ok(doc) => Ok(Bson::Document(doc).clone().into()),
            Err(e) => Err(format!("{}", e))
        }
    }

    router.get("/users", middleware! { |_request, mut response|

        // Connect to the database
        let client = Client::connect("localhost", 27017)
          .ok().expect("Error establishing connection.");

        // The users collection
        let coll = client.db("rust-users").collection("users");

        // Create cursor that finds all documents
        let cursor = coll.find(None, None).unwrap();

        // Opening for the JSON string to be returned
        let mut data_result = "{\"data\":[".to_owned();

        for (i, result) in cursor.enumerate() {
            match get_data_string(result) {
                Ok(data) => {
                    let string_data = if i == 0 {
                        format!("{}", data)
                    } else {
                        format!("{},", data)
                    };

                    data_result.push_str(&string_data);
                },

                Err(e) => return response.send(format!("{}", e))
            }
        }

        // Close the JSON string
        data_result.push_str("]}");

        // Set the returned type as JSON
        response.set(MediaType::Json);

        // Send back the result
        format!("{}", data_result)

    });

    router.post("/users/new", middleware! { |request, response|

        // Accept a JSON string that corresponds to the User struct
        //let user = request.json_as::<User>().unwrap();
        let mut body = String::new();
        request.origin.read_to_string(&mut body).unwrap();
        let user = serde_json::from_str::<User>(&body).unwrap();

        let firstname = user.firstname.to_string();
        let lastname = user.lastname.to_string();
        let email = user.email.to_string();

        // Connect to the database
        let client = Client::connect("localhost", 27017)
            .ok().expect("Error establishing connection.");

        // The users collection
        let coll = client.db("rust-users").collection("users");

        // Insert one user
        match coll.insert_one(doc! {
            "firstname" => firstname,
            "lastname" => lastname,
            "email" => email
        }, None) {
            Ok(_) => (StatusCode::Ok, "Item saved!"),
            Err(e) => return response.send(format!("{}", e))
        }

    });

    router.delete("/users/:id", middleware! { |request, response|

        let client = Client::connect("localhost", 27017)
            .ok().expect("Failed to initialize standalone client.");

        // The users collection
        let coll = client.db("rust-users").collection("users");

        // Get the objectId from the request params
        let object_id = request.param("id").unwrap();

        // Match the user id to an bson ObjectId
        let id = match ObjectId::with_string(object_id) {
            Ok(oid) => oid,
            Err(e) => return response.send(format!("{}", e))
        };

        match coll.delete_one(doc! {"_id" => id}, None) {
            Ok(_) => (StatusCode::Ok, "Item deleted!"),
            Err(e) => return response.send(format!("{}", e))
        }

    });

    server.utilize(authenticator);
    server.utilize(router);
    
    server.listen("127.0.0.1:9000").unwrap();
}

fn authenticator<'mw>(request: &mut Request, response: Response<'mw>, ) ->MiddlewareResult<'mw> {

  // Check if we are getting an OPTIONS request
  if request.origin.method.to_string() == "OPTIONS".to_string() {
      // The middleware should not be used for OPTIONS, so continue
      response.next_middleware()
  } else {

    // We do not want to apply the middleware to the login route
    if request.origin.uri.to_string() == "/login".to_string() {
        response.next_middleware()
    } else {
        // Get the full Authorization header from the incoming request headers
        let auth_header = match request.origin.headers.get::<Authorization<Bearer>>() {
            Some(header) => header,
            None => panic!("No authorization header found")
        };

        // Format the header to only take the value
        let jwt = header::HeaderFormatter(auth_header).to_string();

        // We don't need the Bearer part,
        // so get whatever is after an index of 7
        let jwt_slice = &jwt[7..];

        // Parse the token
        let token = Token::<Header, Registered>::parse(jwt_slice).unwrap();

        // Get the secret key as bytes
        let secret = AUTH_SECRET.as_bytes();

        // Verify the token
        if token.verify(&secret, Sha256::new()) {
            response.next_middleware()
        } else {
            response.error(Forbidden, "Access denied")
        }
    }
  }

}
