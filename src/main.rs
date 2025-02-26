mod api;
mod db;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    api::start_api_server().await;
}


/*
    API TESTS
    //TODO: convert to an actual test that can be called

Create test:
curl -X POST "http://localhost:3000/kv/create" \
     -H "Content-Type: application/json" \
     -d '{"key": "test entry 1", "value": "this is an entry added via curl"}'  

Read test:
curl -X GET  "http://localhost:3000/kv/read" \
     -H "Content-Type: application/json" \
     -d '{"key": "test entry 1"}'                                       

Update test: 
curl -X PUT "http://localhost:3000/kv/update" \
     -H "Content-Type: application/json" \
     -d '{"key": "test entry 1", "value": "this is an entry added via curl 2"}'

Delete test:
curl -X DELETE "http://localhost:3000/kv/delete" \
     -H "Content-Type: application/json" \
     -d '{"key": "test entry 1"}'   

*/