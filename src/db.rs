use std::clone;

use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::ffi::SQLITE_DBCONFIG_NO_CKPT_ON_CLOSE;



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


    fn configure_wal (&self) -> Result<bool, String> {
        let tmp_conn = self.get_conn();
        match tmp_conn {
            Ok(conn) => {
                let config_res =  conn.execute("PRAGMA journal_mode=WAL;", []);
                match config_res {
                    Ok( _) => {
                        Ok(true)
                    }
                    Err (err) => {
                        Err(format!("The configuration could not be made. Please see the following error: {}", err).to_string())
                    }
                }
            }
            Err (err) => {
                Err(err)
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


pub struct KvDb {
    sql_file_name: String,
    sql_tables: Vec<String>,
    conn_pool: SqliteConnPool,
}

impl KvDb {
    fn init (mut self) -> Result<bool, String> {
        let sql_db = r#"CREATE TABLE IF NOT EXISTS kv (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key TEXT UNIQUE NOT NULL,
            value TEXT NOT NULL
        );"#;
        //create new db conn pool
        let mut new_db = SqliteConnPool::create_new_db(self.sql_file_name);
        match new_db {
            Ok(db_conn) => {
                //make wal configuration
                let _ = db_conn.configure_wal().unwrap();
                //create db tables
                let _ = db_conn.create_table(sql_db).unwrap();
                Ok(true)
            }
            Err(err) =>{
                Err(err)
            }
        } 

    }
    //create
    //read 
    //update
    //delete
}