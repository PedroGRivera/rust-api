mod api;
mod db;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    basic_tests();
}


fn basic_tests () {
    /* 
        basic tests 
        TODO: move to a test function in db.rs as a testable function
    */
    let mut db =  db::KvDb::init("test.db".to_string() ).unwrap();
    let _ = db.create("0", "asdf");
    let _ = db.create("1", "qwerty");
    let _ = db.create("2", "hello");

    //read all three
    for key in ["0","1","2"] {
        let (tmp_key, tmp_val) = db.read(key).unwrap();
        println!("key: {} val: {}", tmp_key, tmp_val );
    }
    //update 1
    let _ = db.update("0", "asdfasdf").unwrap();

    //read 1
    let (tmp_key, tmp_val) = db.read("0").unwrap();
    println!("key: {} val: {}", tmp_key, tmp_val );
    
    //delete 1
    let _ = db.delete("0").unwrap();
    // read deleted one   
    let read_res = db.read("0");
    match read_res {
        Ok((tmp_key, tmp_val)) => {
            println!("key: {} val: {}", tmp_key, tmp_val );
        }
        Err(_) => {
            println!("row not found");
        }
    }
}