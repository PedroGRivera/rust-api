# Description

This applicaiton is a basic api written in rust using tokio, serde, axum, rusqlite, r2d2.
Though a lot of these tools are abit overkill, I just wanted a good mixture of tools hopefully learn rust quicker.

# Goals

The goals are: 
* create a basic KV store in a sqlite database
* use r2d2 to have a basic connection pool
* offer the KV interface as an api via axum
* focus on documentation
* use the built in testing framework for major features
* optimize the build in the cargo.toml file
* only use unwrap if absolutely necessary
* minimize clone commands
