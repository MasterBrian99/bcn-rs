mod util;

use crate::util::block::{block_valid, Block};
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize};
use std::sync::Mutex;

pub struct AppState {
    blockchain: Mutex<Vec<Block>>, // <- Mutex is necessary to mutate safely across threads
}

#[get("/")]
async fn get_block_chain(data: web::Data<AppState>) -> Result<impl Responder, Error> {
    // HttpResponse::Ok().body("Hello world!")
    // Ok(web::Json(&data.blockchain))
    let json = serde_json::to_string(&data.blockchain);
    Ok(json)
}

#[derive(Deserialize)]
struct Message {
    bpm: u64,
}
#[post("/")]
async fn write_block(message: web::Json<Message>,app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let mut block=Block::new();

    let mut blockchain=app_state.blockchain.lock().unwrap();
    block= block.generate_block(&blockchain[&blockchain.len()-1], message.bpm);
    let is_valid=block_valid(&block,&blockchain[&blockchain.len()-1]);
    if is_valid {
        println!("check is valid");
        blockchain.push(block.clone());
    }
    // if blockchain.len()>0 {
    //     &block.generate_block(&blockchain[blockchain.len()-1],message.bpm);
    // }else{
    //     let  old_block=Block::new();
    //     &block.generate_block(&old_block,message.bpm);
    // }
    // println!("block {:?}",block);
    Ok(HttpResponse::Created().body(format!("Hey there! {:?}", serde_json::to_string(&block))))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut  initial_vec:Vec<Block>=Vec::new();
    initial_vec.push(Block::new());
    let blockchain = web::Data::new(AppState {
        blockchain: Mutex::new(initial_vec),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(blockchain.clone())
            .service(write_block)
            .service(get_block_chain)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
