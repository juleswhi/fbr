use std::io::BufRead;
use std::io::Write;
use std::{
    fs,
    io::BufReader,
    net::{TcpListener, TcpStream},
};

use crate::router::{self};

pub fn run() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_conn(stream);
    }
}

fn handle_conn(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let binding = buf_reader.lines().next().unwrap().unwrap();

    let req_line: Vec<&str> = binding.split(" ").collect();

    let (status_line, filename) = match req_line[1] {
        "/" => ("HTTP/1.1 200 OK".to_string(), "root.html".to_string()),
        path => {
            println!("Path: {}", path);
            let path_spl: Vec<String> = path.split("/").map(|x| x.to_string()).collect();
            let mut header = ("HTTP/1.1 404 NOT FOUND".to_string(), "404.html".to_string());
            let files = router::router::get_all_files();
            let spl = router::router::split_files(files);

            let pages = router::router::filter_pages(&spl);

            for page in pages.unwrap_or(vec![]) {
                println!("Page 0: {}, Path 0: {}", page[1], path_spl[1]);
                let _l = page.len();
                let _i: usize = 0;

                if page[1].split(".html").collect::<Vec<&str>>()[0] == path_spl[1] {
                    let cl = page.clone();
                    let name = cl.join("/");
                    header = ("HTTP/1.1 200 OK".to_string(), name.clone());
                }
            }

            header
        }
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
