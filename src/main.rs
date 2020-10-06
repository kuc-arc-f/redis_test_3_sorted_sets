// Sorted Set + String type, add sample
//

extern crate redis;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use redis::{Commands};
use serde::{Deserialize, Serialize};

//
pub fn get_content(filename: String ) -> String{
    //    println!("In file {}", filename);
        let mut f = File::open(filename).expect("file not found");
    
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
    
    //    println!("With text:\n{}", contents);
        return contents;
    }

//
#[derive(Serialize, Deserialize , Debug)]
struct TaskItem {
    id: i64,
    title: String,
    content: String,
} 
#[derive(Serialize, Deserialize , Debug)]
struct TaskAddItem {
    id: String,
    title: String,
    content: String,
} 
//
fn test3() -> redis::RedisResult<()>{
    let client = redis::Client::open("redis://localhost/").expect("url error");
    let mut connection = client.get_connection().expect("connect error");

    let res_incr: usize = connection.incr("idx-post", 1).unwrap();
    let key_hd = "test2:";
    let key_sorted = "sorted_3";
    let key = format!("{}{}" , &key_hd, res_incr );
    println!("key: {}", key);
    let key_2 = String::from( &key );
    println!("key2: {}", key_2);

    let result2: u8 = connection.zadd(key_sorted , key , res_incr ).unwrap();
    let _:() = connection.set(key_2 , "value-1" ).unwrap();

    Ok(())
}
//
fn add_sorted(items: Vec<TaskItem>) -> redis::RedisResult<()>{
    let client = redis::Client::open("redis://localhost/").expect("url error");
    let mut connection = client.get_connection().expect("connect error");

    let key_hd = "task:";
    let key_sorted = "sorted_tasks";
    for row in &items {
        let res_incr: usize = connection.incr("idx-task", 1).unwrap();
        let key = format!("{}{}" , &key_hd, res_incr );
        let key_2 = String::from( &key );
        let key_3 = String::from( &key );
        let result2: u8 = connection.zadd(key_sorted , key , res_incr ).unwrap();
        let item = TaskAddItem {
             id: String::from(&key_3 ), 
             title: row.title.to_string(), 
             content: row.content.to_string(), 
        };
        let serialized = serde_json::to_string(&item ).unwrap();
        let _:() = connection.set(key_2 , &serialized ).unwrap();        
    }

    Ok(())
}
//
fn main() {
    println!("#start");
    let fname = "/home/naka/work/node/express/app7/public/tasks.json";
    let json = get_content( fname.to_string() );
    let deserialized: Vec<TaskItem> = serde_json::from_str(&json).unwrap();

    let r = add_sorted(deserialized);
}
