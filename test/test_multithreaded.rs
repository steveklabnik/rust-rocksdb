use rocksdb::{Options, DB, Writable, Direction, DBResult};
use std::thread::{self, Builder};
use std::sync::Arc;

const N: usize = 100_000;

#[test]
pub fn test_multithreaded() {
    let path = "_rust_rocksdb_multithreadtest";
    {
        let db = DB::open_default(path).unwrap();
        let db = Arc::new(db);

        db.put(b"key", b"value1");

        let db1 = db.clone();
        let j1 = thread::spawn(move|| {
            for i in 1..N {
                db1.put(b"key", b"value1");
            }
        });

        let db2 = db.clone();
        let j2 = thread::spawn(move|| {
            for i in 1..N {
                db2.put(b"key", b"value2");
            }
        });

        let db3 = db.clone();
        let j3 = thread::spawn(move|| {
            for i in 1..N {
                match db3.get(b"key") {
                    DBResult::Some(v) => {
                        if &v[..] != b"value1" && &v[..] != b"value2" {
                            assert!(false);
                        }
                    }
                    _ => {
                        assert!(false);
                    }
                }
            }
        });

        j1.join();
        j2.join();
        j3.join();
    }
    assert!(DB::destroy(&Options::new(), path).is_ok());
}
