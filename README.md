# Description

This applicaiton is a basic api written in rust using tokio, serde, axum, rusqlite, r2d2.
Though a lot of these tools are abit overkill, I just wanted a good mixture of tools hopefully learn rust quicker.

# Goals

The goals are: 
- [X] create a basic KV store in a sqlite database
- [X] use r2d2 to have a basic connection pool
- [X] offer the KV interface as an api via axum
- [ ] focus on documentation
- [X] use the built in testing framework for major features (NOTE: I did not use this for testing the api since I felt that I got a good deal of practice with the first test in the db.rs file)
- [X] optimize the build in the cargo.toml file
- [X] only use unwrap if absolutely necessary (there were a few cases such as initializing the server that I deemed it necessary for it to panic so I left the unwraps. I suspect in more rigorous projects, I should choose to not unrwap at all.)
- [ ] minimize clone commands


# Notes
* I think that the verbosity of Rust is nice because it allows for you to be very specific when handeling each error case. That being said, I think I could abstract a lot of the repeated code into sub-functions. For this task I have left it with a lot of repeated code so that I can gain that muscle memory of writing rust, but I imagine in the future I will need to balance readability and abstraction given that it seems pretty easy for the complexity to grow pretty quickly. 

