    #[macro_use] extern crate rocket;
    
    
use rocket::State;

    use rocket::Response;
    use rocket::Request;
    use rocket::fairing::Info;
    use rocket::fairing::Fairing;
    use rocket_contrib::json::{Json, JsonValue};
    use rocket::fairing::Kind;
    use rocket::http::{Method,Header,ContentType};
    use std::io::Cursor;
    use log::{debug, error, log_enabled, info, Level};
    //define our own origin that is injected as managed state as this type

    #[derive(Debug)]
    pub struct Origin{//should ideally be a new type but im to dumb to figure it out
        pub origin:String
    }
    //TODO this can go when theres a proper cors support in rocket
    pub struct CORS;

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response
        }
    }
    //have on attach hook to check the orgin is in the state
    fn on_response(&self, request: &Request, response: &mut Response) {
        //use the request guardd to get the origin from the state
        let app_config = request.guard::<State<Origin>>().unwrap();
        let a = app_config.origin.clone();//TODO this needs to be better as its probably quite inefficient doing a clone on every requst

        if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON) {
            response.set_header(Header::new("Access-Control-Allow-Origin", a));
            response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, OPTIONS"));
            response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
            
        }

        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new(""));
            response.set_status(rocket::http::Status::new(204, "No Content"));
        }
    }
}




/* #[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
} */
