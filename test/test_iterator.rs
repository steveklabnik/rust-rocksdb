use rocksdb::{Options, RocksDB, Writable, Direction};
use std;

fn cba(input: &Box<[u8]>) -> Box<[u8]> {
    input.iter().cloned().collect::<Vec<_>>().into_boxed_slice()
}

#[test]
pub fn test_iterator() {
    let path = "_rust_rocksdb_iteratortest";
    {
        let k1:Box<[u8]> = b"k1".to_vec().into_boxed_slice();
        let k2:Box<[u8]> = b"k2".to_vec().into_boxed_slice();
        let k3:Box<[u8]> = b"k3".to_vec().into_boxed_slice();
        let k4:Box<[u8]> = b"k4".to_vec().into_boxed_slice();
        let v1:Box<[u8]> = b"v1111".to_vec().into_boxed_slice();
        let v2:Box<[u8]> = b"v2222".to_vec().into_boxed_slice();
        let v3:Box<[u8]> = b"v3333".to_vec().into_boxed_slice();
        let v4:Box<[u8]> = b"v4444".to_vec().into_boxed_slice();
        let mut db = RocksDB::open_default(path).unwrap();
        let p = db.put(&*k1, &*v1);
        assert!(p.is_ok());
        let p = db.put(&*k2, &*v2);
        assert!(p.is_ok());
        let p = db.put(&*k3, &*v3);
        assert!(p.is_ok());
        let mut view1 = db.iterator();
        let expected = vec![(cba(&k1), cba(&v1)), (cba(&k2), cba(&v2)), (cba(&k3), cba(&v3))];
        {
            let mut iterator1 = view1.from_start();
            assert_eq!(iterator1.collect::<Vec<_>>(), expected);
        }
        // Test that it's reusable a few times
        {
            let mut iterator1 = view1.from_start();
            assert_eq!(iterator1.collect::<Vec<_>>(), expected);
        }
        {
            let mut iterator1 = view1.from_start();
            assert_eq!(iterator1.collect::<Vec<_>>(), expected);
        }
        {
            let mut iterator1 = view1.from_start();
            assert_eq!(iterator1.collect::<Vec<_>>(), expected);
        }
        // Test it in reverse a few times
        {
            let mut iterator1 = view1.from_end();
            let mut tmp_vec = iterator1.collect::<Vec<_>>();
            tmp_vec.reverse();
            assert_eq!(tmp_vec, expected);
        }
        {
            let mut iterator1 = view1.from_end();
            let mut tmp_vec = iterator1.collect::<Vec<_>>();
            tmp_vec.reverse();
            assert_eq!(tmp_vec, expected);
        }
        {
            let mut iterator1 = view1.from_end();
            let mut tmp_vec = iterator1.collect::<Vec<_>>();
            tmp_vec.reverse();
            assert_eq!(tmp_vec, expected);
        }
        {
            let mut iterator1 = view1.from_end();
            let mut tmp_vec = iterator1.collect::<Vec<_>>();
            tmp_vec.reverse();
            assert_eq!(tmp_vec, expected);
        }
        {
            let mut iterator1 = view1.from_end();
            let mut tmp_vec = iterator1.collect::<Vec<_>>();
            tmp_vec.reverse();
            assert_eq!(tmp_vec, expected);
        }
        // Try it forward again
        {
            let mut iterator1 = view1.from_start();
            assert_eq!(iterator1.collect::<Vec<_>>(), expected);
        }
        {
            let mut iterator1 = view1.from_start();
            assert_eq!(iterator1.collect::<Vec<_>>(), expected);
        }

        let mut view2 = db.iterator();
        let p = db.put(&*k4, &*v4);
        assert!(p.is_ok());
        let mut view3 = db.iterator();
        let expected2 = vec![(cba(&k1), cba(&v1)), (cba(&k2), cba(&v2)), (cba(&k3), cba(&v3)), (cba(&k4), cba(&v4))];
        {
            let mut iterator1 = view1.from_start();
            assert_eq!(iterator1.collect::<Vec<_>>(), expected);
        }
        {
            let mut iterator1 = view3.from_start();
            assert_eq!(iterator1.collect::<Vec<_>>(), expected2);
        }
        {
            let mut iterator1 = view3.from(b"k2", Direction::forward);
            let expected = vec![(cba(&k2), cba(&v2)), (cba(&k3), cba(&v3)), (cba(&k4), cba(&v4))];
            assert_eq!(iterator1.collect::<Vec<_>>(), expected);
        }
        {
            let mut iterator1 = view3.from(b"k2", Direction::reverse);
            let expected = vec![(cba(&k2), cba(&v2)), (cba(&k1), cba(&v1))];
            assert_eq!(iterator1.collect::<Vec<_>>(), expected);
        }
    }
    let opts = Options::new();
    assert!(RocksDB::destroy(&opts, path).is_ok());
}
