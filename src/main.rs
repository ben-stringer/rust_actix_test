use actix::prelude::*;
use actix_web::{web, get, App, Responder, HttpServer};

// Messages are structs and can contain data
struct GetCountMsg;

// Implement the Message trait, defining the result of the message
impl Message for GetCountMsg {
    type Result = usize;
}

// Actors are structs
struct GoodActor {
    count: usize
}

// Implement the Actor trait, defining Context
// I'm not sure, but I feel like this is similar to C++'s CRTP
impl Actor for GoodActor {
    type Context = Context<Self>;
}

// Implement the Handler trait for each message your actor can handle.
impl Handler<GetCountMsg> for GoodActor {
    type Result = usize;

    fn handle(&mut self, _msg: GetCountMsg, _ctx: &mut Context<Self>) -> Self::Result {
        let mut i = self.count;
        i += 1;
        self.count = i;
        i
        // Or more succinctly,
        /*
        self.count += 1;
        self.count
        */
        // I did it this way because I want to see how to make a "BadActor",
        // one that yields after getting i, then updates later.
    }
}

// Standard actix-web stuff
#[get("/")]
async fn index(actor_addr: web::Data<Addr<GoodActor>>) -> impl Responder {
    // Send a message, get back a future, await the Result, and unwrap it.
    // I suppose unwrap is bad because there could be an error in the message passing framework
    let i = actor_addr.send( GetCountMsg {} ).await.unwrap();
    // Response to the web client
    format!("count: {}", i)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    // Create an actor
    let good_actor = GoodActor {count: 0};
    // Start the actor, getting back an address to send it messages
    // Note we move the address into the App's data /and/ clone it for some reason I haven't
    // figured out yet.
    let addr = good_actor.start();

    // Standard actix-web stuff
    HttpServer::new(move|| 
        App::new()
            .data(addr.clone())
            .service(index)
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// $> for i in $(seq 1 100); do curl localhost:8080/ & echo; done
