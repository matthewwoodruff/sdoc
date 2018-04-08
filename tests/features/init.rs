use features::common::{EDIT_USAGE, HELP_TEXT, run};
use tempfile;
use std::result::Result;
use std::thread;

//#[test]
//fn do_it() {
//
//    // Create temp directory
//// execute binary with temp directory as working directory
//// execute newly created binary
//    let temp_dir = tempfile::tempdir().unwrap();
//
//    println!("{:?}", temp_dir.path());
//
//
//
//    vanilla()
//        .inside(temp_dir.path())
//        .input("y\ntest-cli\n")
//        .output("s")
//        .succeeds();
//
//    thread::sleep_ms(10000);
//
//}