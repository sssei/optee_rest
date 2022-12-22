use std::fs::File;
use std::fs;
use std::io::{Write, Read};
use std::time::{Instant, Duration};

fn main() {
    for i in 1..62 {
        let size = i * 256;
        measure(size);
    }    
}

fn measure(size: u32) {
    let mut time_create = Duration::new(0, 0);
    let mut time_read  = Duration::new(0, 0);
    let mut time_delete = Duration::new(0, 0);        

    let iter = 100; 

    for _ in 0..iter {
        let time1 = Instant::now();
        create_file(size);
        let time2 = Instant::now();
        read_file();
        let time3 = Instant::now();    
        delete_file();
        let time4 = Instant::now();

        time_create += time2 - time1;
        time_read += time3 - time2; 
        time_delete += time4 - time3;
    }

    println!("size = {:?}", size);
    println!("{:?}", (time_create / iter).as_nanos() );
    println!("{:?}", (time_read / iter).as_nanos());
    println!("{:?}", (time_delete / iter).as_nanos());
}

fn create_file(file_size: u32){
    let body = vec![0u8; file_size.try_into().unwrap()];
    let mut file = File::create("config").unwrap();
    file.write_all(&body).unwrap();
}

fn read_file(){
    let mut data = String::new();
    let mut file = File::open("config").unwrap();
    file.read_to_string(&mut data).unwrap();
}

fn delete_file(){
    fs::remove_file("config").unwrap();
}


