pub fn send_bytes<T: AsRef<[u8]>>(req: http::Request<T>) -> Result<http::Response<Vec<u8>>, ureq::Error>{
    let (http_parts, body) = req.into_parts();
    let reader_req = http::Request::from_parts(http_parts, std::io::Cursor::new(body));
    send_reader(reader_req)
}

pub fn send_reader<R: std::io::Read>(req: http::Request<R>) -> Result<http::Response<Vec<u8>>, ureq::Error>{
    eprintln!("Requesting {}", req.uri().to_string());
    let (http_parts, body) = req.into_parts();
    let request: ureq::Request = http_parts.into();
    Ok(request.send(body)?.into())
}
