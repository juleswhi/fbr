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
            let path_spl: Vec<String> = path.split("/").map(|x| x.to_string()).skip(1).collect();
            let mut header = ("HTTP/1.1 404 NOT FOUND".to_string(), "404.html".to_string());
            let files = router::router::get_all_files();
            let spl = router::router::split_files(files);

            let pages = router::router::filter_pages(&spl);

            for page in pages.unwrap_or(vec![]) {
                let page_path = page[1..]
                    .iter()
                    .map(|x| x.split(".html").collect::<Vec<&str>>()[0])
                    .collect::<Vec<&str>>()
                    .join("/");

                let path_path = path_spl.join("/");

                if page_path == path_path {
                    let fm = format!("pages/{page_path}.html");
                    println!("{}", fm);
                    header = ("HTTP/1.1 200 OK".to_string(), fm);
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
