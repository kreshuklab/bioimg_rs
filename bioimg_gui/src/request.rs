
pub fn send_bytes<T: AsRef<[u8]>>(req: http::Request<T>) -> Result<http::Response<Vec<u8>>, String>{
    let (http_parts, body) = req.into_parts();
    let reader_req = http::Request::from_parts(http_parts, std::io::Cursor::new(body));
    send_reader(reader_req)
}

pub fn send_reader<R: std::io::Read>(req: http::Request<R>) -> Result<http::Response<Vec<u8>>, String>{
    println!("Requesting {}", req.uri().to_string());
    let (http_parts, body) = req.into_parts();
    let request: ureq::Request = http_parts.into();
    let resp: http::Response<Vec<u8>> = request.send(body)
        .map_err(|err| {
            eprintln!("Ok, so something went wrong!!!!!!!!!!!!!!!!!!!!!!!");
            let ureq::Error::Status(status, resp) = err else {
                eprintln!(">>>>>>> iit was transport error");
                return "Some transport error".to_owned();
            };
            let mut reader = resp.into_reader();
            let mut buf = vec![];
            reader.read_to_end(&mut buf).unwrap();
            let s = String::from_utf8_lossy(&buf);
            eprintln!(">>>>>>> iit was something else {status}: {s}");
            s.to_string()
        })?
        .into();
    if !resp.status().is_success(){
        let payload = String::from_utf8_lossy(resp.body());
        eprintln!("Error!!\n{}", payload);
        return Err(format!("Request failed with result {}\n{payload}", resp.status()))
    }
    Ok(resp)
}
