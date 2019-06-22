use std::fs;
use std::io::prelude::*;
use std::ops::*;


extern crate regex;
use regex::Regex;

#[derive(Debug)]
enum FileType {
    PlainText,
    Stream,
}

#[derive(Debug)]
enum FormDataValue {
    Text(String),
    File(FileType, String, Vec<u8>), // (filetype, filename, data)
}

#[derive(Debug)]
struct FormData {
    name: String,
    value: FormDataValue,
}

fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                std::thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(error) => println!("get stream error: {}", error),
        }
    }
}

fn save_file(save_dir: &str, file_name: &str, file_data: &Vec<u8>) {
    let mut buffer = fs::File::create(format!("{}/{}", save_dir, file_name));
    match buffer {
        Ok(mut buf) => match buf.write(file_data) {
            Ok(_size) => {}
            Err(err) => println!("write file error : {}", err),
        },
        Err(err) => println!("create file error : {}", err),
    }
}

fn write_stream(mut stream: std::net::TcpStream, response: String) {
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_connection(mut stream: std::net::TcpStream) {
    const FILE_DIR: &str = "/Users/ethles/temp";
    const BUFFER_SIZE: usize = 65536;
    const LIMIT_ALL_SIZE: usize = 50000;
    const LIMIT_SINGLE_FIELD: usize = 5000;
    const OK_STATUS_LINE: &str = "HTTP/1.1 200 OK\r\n\r\n";
    const ERR_SIZE_STATUS_LINE: &str = "HTTP/1.1 400 SIZE TOO LARGE\r\n\r\n";
    const ERR_INVALID_FIELD: &str = "HTTP/1.1 401 INVALID FIELD\r\n\r\n";

    let status_re = Regex::new(r"^(GET|POST) / HTTP/1.1$").unwrap();
    let formdata_magic_re =
        Regex::new(r"^Content-Type: multipart/form-data; boundary=(.*)").unwrap();
    let header_re = Regex::new(r"^(.*?): (.*?)").unwrap();
    let disposition_name_re = Regex::new("Content-Disposition: form-data; name=\"(.*?)\"").unwrap();
    let disposition_filename_re =
        Regex::new("Content-Disposition: form-data; .*? filename=\"(.*?)\"").unwrap();
    let content_type_re = Regex::new("^Content-Type: (.*?)").unwrap();
    let text_type_data_re =
        Regex::new("Content-Disposition: form-data; name=\".*?\"\r\n([\\S\\s]*?)--").unwrap();
    let file_type_data_re = Regex::new(
        "Content-Disposition: form-data; name=\".*?\"; filename=\".*?\"\r\n([\\S\\s]*?)\r\n--",
    )
    .unwrap();

    let mut buffer = [0; BUFFER_SIZE];
    let size = stream.read(&mut buffer).unwrap();
    if size >= LIMIT_ALL_SIZE {
        write_stream(stream, String::from(ERR_SIZE_STATUS_LINE));
        return;
    }
    let contents = String::from_utf8(buffer.to_vec()).unwrap();
    let mut lines = contents.lines();

    println!("===PROCESS REQUEST LINE===");
    let method: &str;
    let line = lines.next().unwrap();
    match status_re.captures(line) {
        Some(cap) => {
            method = cap.index(1);
            println!("METHOD : {}", method)
        }
        None => {
            write_stream(stream, String::from(ERR_INVALID_FIELD));
            return;
        }
    }

    println!("===PROCESS HEADER===");
    let mut headers: Vec<Vec<&str>> = Vec::new();
    let mut content_type_boundary = String::new();
    loop {
        let line = lines.next().unwrap();
        match line {
            content_type_line if content_type_re.is_match(content_type_line) => {
                headers.push(content_type_line.split_terminator(r": ").collect());
                match formdata_magic_re.captures(content_type_line) {
                    Some(cap) => {
                        content_type_boundary = String::from(cap.index(1));
                        println!("FORMDATA BOUNDARY : {}", content_type_boundary)
                    }
                    None => {}
                }
            }
            header_line if header_re.is_match(header_line) => {
                headers.push(header_line.split_terminator(r": ").collect())
            }
            "" => {
                println!("HEADER : {:?}", headers);
                break;
            }
            _ => {
                write_stream(stream, String::from(ERR_INVALID_FIELD));
                return;
            }
        }
    }

    println!("===PROCESS DATA===");
    for form_data_str in contents
        .split_terminator(content_type_boundary.as_str())
        .skip(2)
    {
        if form_data_str.len() > LIMIT_SINGLE_FIELD {
            write_stream(stream, String::from(ERR_SIZE_STATUS_LINE));
            return;
        }
        let mut form_data;
        let name = String::from(
            disposition_name_re
                .captures(form_data_str)
                .unwrap()
                .index(1),
        );
        match disposition_filename_re.captures(form_data_str) {
            Some(cap) => {
                let filename = String::from(cap.index(1));
                let data = file_type_data_re
                    .captures(form_data_str)
                    .unwrap()
                    .index(1)
                    .as_bytes()
                    .to_vec();
                save_file(&FILE_DIR, &filename.as_str(), &data);
                form_data = FormData {
                    name: name,
                    value: FormDataValue::File(FileType::Stream, filename, data), // keep orgin file , so skip parsing the file type
                };

            }
            None => {
                let text =
                    String::from(text_type_data_re.captures(form_data_str).unwrap().index(1));
                form_data = FormData {
                    name: name,
                    value: FormDataValue::Text(text),
                }
            }
        }
        println!("{:?}", form_data)
    }
    write_stream(stream, String::from(OK_STATUS_LINE));
}


// ===HTTP REQUEST REFERENCE===
// POST / HTTP/1.1
// Content-Type: multipart/form-data; boundary=--------------------------080905360496688834795009
// User-Agent: PostmanRuntime/7.15.0
// Accept: */*
// Cache-Control: no-cache
// Postman-Token: 6020e13b-8475-42e7-86ef-3dd30fcc2f75
// Host: localhost:8080
// accept-encoding: gzip, deflate
// content-length: 559
// Connection: keep-alive
//
// ----------------------------080905360496688834795009
// Content-Disposition: form-data; name="s"
//
// sfddsf
// ----------------------------080905360496688834795009
// Content-Disposition: form-data; name=""; filename="temp.txt"
// Content-Type: text/plain
//
// lskdfjslkdfj
// slkfjsdlfjks
// skldfjdksfj
// sdfksdfj
//
// ----------------------------080905360496688834795009
// Content-Disposition: form-data; name=""; filename="tempb"
// Content-Type: application/octet-stream
//
// lskdfjslkdfj
// slkfjsdlfjks
// skldfjdksfj
// sdfksdfj
//
// ----------------------------080905360496688834795009--