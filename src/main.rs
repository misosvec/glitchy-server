use sha2::{Digest, Sha256};
use std::io::{BufRead, BufReader, Read, Write, stdin, stdout};
use std::net::TcpStream;

fn send_request(
    host: &str,
    port: &str,
    range_start: usize,
    range_end: usize,
) -> Result<TcpStream, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(format!("{host}:{port}"))?;
    let request = format!(
        "GET / HTTP/1.1\r\n\
         Accept: */*\r\n\
         Host: {host}\r\n\
         Range: bytes={range_start}-{range_end}\r\n\r\n"
    );
    stream.write_all(request.as_bytes())?;
    Ok(stream)
}

fn parse_response(stream: &TcpStream) -> Result<(u16, usize, Vec<u8>), Box<dyn std::error::Error>> {
    let mut reader = BufReader::new(stream);
    let mut headers = String::new();

    // read headers until an empty line is encountered
    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        headers.push_str(&line);
        if line.trim().is_empty() {
            break;
        }
    }

    // retrieve the status code from the headers
    let status_code = headers
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|code| code.parse::<u16>().ok())
        .ok_or("Failed to parse status code")?;
    
    // retrieve content length from headers
    let requested_content_length = headers
        .lines()
        .find(|line| line.to_lowercase().starts_with("content-length"))
        .and_then(|line| line.split(':').nth(1))
        .and_then(|value| value.trim().parse::<usize>().ok())
        .ok_or("Failed to parse Content-Length")?;

    // read the body of the response
    let mut body = Vec::new();
    reader.read_to_end(&mut body)?;

    Ok((status_code, requested_content_length, body))
}

fn get_data(
    host: &str,
    port: &str,
    expected_hash: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

    // we are trying to download the entire data in a single request
    // it should be fine, as the data is only 1MB
    // if the range exceeds the actual data size, the server will return all available data
    const MAX_DATA_SIZE: usize = 1024 * 1024;

    let mut data: Vec<u8> = Vec::new();

    loop {
        let range_start = data.len();
        let range_end = MAX_DATA_SIZE;

        let stream = send_request(host, port, range_start, range_end)?;
        let (status_code, requested_content_length, body) = parse_response(&stream)?;
        
        if status_code != 200 && status_code != 206 {
            return Err(format!("Error occurred, status code {}", status_code).into());
        }
        
        println!("Content-Length: {}", requested_content_length);
        println!("Downloaded bytes of data: {} ", body.len());
        data.extend_from_slice(&body);

        if body.len() == requested_content_length {
            break;
        }
    }

    let mut hasher = Sha256::new();
    hasher.update(&data);
    let computed_hash: String = hasher
        .finalize()
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect();

    println!("Downloaded {} bytes of data in total", data.len());
    if computed_hash == expected_hash {
        println!("The hashes are the same.");
    } else {
        println!("The hashes are different.");
    }
    Ok(data)
}

fn main() {
    let host = "127.0.0.1";
    let port = "8080";

    // ask the user for the hash input
    println!("Enter SHA-256 hash of the data:");
    stdout().flush().unwrap();
    let mut hash = String::new();
    stdin().read_line(&mut hash).unwrap();

    match get_data(host, port, hash.trim()) {
        Ok(data) => data,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
}