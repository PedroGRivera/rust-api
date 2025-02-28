use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;



pub struct SqliteConnPool {
    pool: Pool<SqliteConnectionManager>
}

impl SqliteConnPool {
    //create db file
    fn create_new_db(file_name: String) -> Result<SqliteConnPool, String> {
        let sql_file =  SqliteConnectionManager::file(file_name);
        let tmp_pool = Pool::new(sql_file);
        match tmp_pool {
            Ok(pool) => {
                Ok(SqliteConnPool{ pool })
            }
            Err (err) => {
                Err(format!("The database was not created correctly. Please see the following error: {}", err).to_string())
            }
        }
    }

    fn get_conn (&self) -> Result<PooledConnection<SqliteConnectionManager>, String> {
        let tmp_conn = self.pool.get();
        match tmp_conn {
            Ok(conn) => {
                Ok(conn)
            }
            Err (err) => {
                Err(format!("The database could not be accessed. Please see the following error: {}", err).to_string())
            }
        }
    }


    //create tables inline
    fn create_table (&self, sql_table:&str) -> Result<bool, String> {
        let tmp_conn = &self.get_conn();
        match tmp_conn {
            Ok(conn) => {
                let create_table_res = conn.execute(sql_table, []);
                match create_table_res {
                    Ok(_) => {
                        Ok(true)
                    }
                    Err(err) => {
                        Err(format!("{}",err).to_string())
                    }
                }
            }
            Err (err) => {
                Err(err.clone())
            }
        }
    }
}

#[allow(dead_code)] //TODO: Figure out if the sql_file_name is needed or can we get away without it?
pub struct KvDb {
    pub sql_file_name: String,
    conn_pool: SqliteConnPool,
}

impl KvDb {

    pub fn init (sql_file_name: String) -> Result<KvDb, String> {
        let sql_db = r#"
        CREATE TABLE IF NOT EXISTS kv (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key TEXT UNIQUE NOT NULL,
            value TEXT NOT NULL
        );"#;
        //create new db conn pool
        let new_db = SqliteConnPool::create_new_db(sql_file_name.clone());
        match new_db {
            Ok(db_conn) => {
                //TODO: make wal configuration
                //create db tables
                let _ = db_conn.create_table(sql_db).unwrap();
                Ok(KvDb{sql_file_name: sql_file_name.clone(), conn_pool: db_conn})
            }
            Err(err) =>{
                Err(err)
            }
        } 

    }
    //create
    pub fn create (&mut self, key: &str, val: &str) -> Result<bool, String>{
        let pool_res = self.conn_pool.get_conn();
        match pool_res {
            Ok(pool) => {
                let insert_res = pool.execute("INSERT INTO kv (key, value) VALUES(?1, ?2);", [key, val]);
                match insert_res {
                    Ok (s) => {
                        Ok (s > 0) //if it is successful the responding usize should be 1 if not then it will be 0
                    }
                    Err(err) => {
                        Err(format!("The Database was unable to insert the new row. Please see the following error: {}", err).to_string())
                    }
                }
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    //read 
    pub fn read (&mut self, key: &str) -> Result<(String,String), String>{

        struct Kv {
            key: String,
            val: String
        }

        let pool_res = self.conn_pool.get_conn();
        match pool_res {
            Ok(pool) => {
                let mut statement = pool.prepare("SELECT key, value FROM kv WHERE key = ?1").unwrap();
                let query_res = statement.query_row([key], |row|{
                    Ok(Kv { key: row.get(0).unwrap(), val:row.get(1).unwrap()} )
                });
                match query_res {
                    Ok(res) => {
                        Ok((res.key,res.val))
                    }
                    Err(err) => {
                        Err(format!("There was an error in the read query. Please see the following error: {}", err).to_string())
                    }
                }
            }
            Err(err) => {
                Err(format!("{}", err).to_string())
            }
        }
    }
    //update
    pub fn update (&mut self, key: &str, val: &str) -> Result<bool, String>{
        let pool_res = self.conn_pool.get_conn();
        match pool_res {
            Ok(pool) => {
                let update_res = pool.execute("UPDATE kv SET value = ?1 WHERE key = ?2 ;", [val, key]);
                match update_res {
                    Ok (s) => {
                        Ok (s > 0) //if it is successful the responding usize should be 1 if not then it will be 0
                    }
                    Err(err) => {
                        Err(format!("The Database was unable to update the row. Please see the following error: {}", err).to_string())
                    }
                }
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    //delete
    pub fn delete (&mut self, key: &str) -> Result<bool, String>{
        let pool_res = self.conn_pool.get_conn();
        match pool_res {
            Ok(pool) => {
                let del_res = pool.execute("DELETE FROM kv WHERE key = ?1 ;", [key]);
                match del_res {
                    Ok (s) => {
                        Ok (s > 0) //if it is successful the responding usize should be 1 if not then it will be 0
                    }
                    Err(err) => {
                        Err(format!("The Database was unable to delete the row. Please see the following error: {}", err).to_string())
                    }
                }
            }
            Err(err) => {
                Err(err)
            }
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn basic_tests () {
        /* 
            basic tests 
            TODO: remove the test.db at the end of test;
        */
        let mut db : KvDb;
        let db_res =  KvDb::init("test.db".to_string() );
        match db_res {
            Ok(res) => {
                db = res;
                assert!(true);
            }
            Err(_) => {
                assert!(false, "The db was not able to be created");
                return;
            }
        }
        let _ = db.create("0", "asdf");
        let _ = db.create("1", "qwerty");
        let _ = db.create("2", "hello");

        //read all three
        for (key,val) in [("0", "asdf"),("1","qwerty"),("2","hello")] {
            let first_read_res = db.read(key);
            match first_read_res {
                Ok((read_key, read_val)) => {
                    let read_res = (read_key == key.to_string()) && (read_val == val.to_string());
                    if read_res {
                        assert!(true);
                    }
                    else {
                        assert!(false, "the read test after an update does not have the correct values. \nExpected key={}, val={} \nReturned key={} val={} ", key,val,read_key, read_val);
                    }
                }
                Err(_) => {
                    assert!(false, "the first read test failed with an error");
                }
            }
        }
        //update 1
        let update_res = db.update("0", "asdfasdf");
        match update_res {
            Ok(res) => {
                if res {
                    assert!(true);
                }
                else {
                    assert!(false, "the update did not successfully execute");
                }
            }
            Err(_) => {
                assert!(false,"the update returned an error");
            }
        }

        //read 1
        let read_test_res = db.read("0");
        match read_test_res {
            Ok((read_key, read_val)) => {
                let read_res = (read_key == "0".to_string()) && (read_val == "asdfasdf".to_string());
                if read_res {
                    assert!(true);
                }
                else {
                    assert!(false, "the read test after an update does not have the correct values. \nExpected key=0, val=asdfasdf \nReturned key={} val={} ", read_key, read_val);
                }
            }
            Err(_) => {
                assert!(false, "the second read test failed with an error");
            }
        }
        
        
        //delete 1
        let delete_test_res = db.delete("0");
        match delete_test_res {
            Ok(res) =>{
                if res {
                    assert!(true);
                }
                else {
                    assert!(false, "the delete test was unable to remove the row from the database");
                }
            }
            Err(_) =>{
                assert!(false, "");
            }
        }
        // read deleted one   
        let read_res = db.read("0");
        match read_res {
            Ok((_, _)) => {
                assert!(false, "The row shouldn't exist in the database anymore");
            }
            Err(_) => {
                assert!(true);
            }
        }
    }

}