use std::collections::HashMap;
use std::io::{Read, Write};
use std::str;
use std::str::from_utf8;

pub struct Uri {
    pub scheme: Protocol,
    pub authority: String,
    pub path: String
}

impl Uri {
    pub fn get_absolute(&self) -> String {
        format!("{}://{}/{}", self.scheme.get(), self.authority, self.path)
    }
    //FIXME STD::IO:ERROR should disappear ASAP
    pub fn parse_url(uri: &str) -> Result<Uri, String> { 
        let mut n;
        let scheme = match uri {
             _ if "http://" == &uri[..7] => {n = 7; Protocol::Http},
             _ if "https://" == &uri[..8] => {n = 8; Protocol::Https},
             _ => return Err(uri[..9].to_string())
        };

        let mut identifier = Uri {scheme, authority: "".to_string(), path: "".to_string()};

        for (i, c) in uri[n..].chars().enumerate() {
            if c != '/' {
                identifier.authority.push(c);
            }
            else {
                n += i;
                break;
            }
        }

        for (_i, c) in uri[n..].chars().enumerate() {
            identifier.path.push(c);
        }

        Ok(identifier)
    }
}

pub enum Protocol {
    Http,
    Https
}

impl Protocol {
    fn get(&self) -> &str {
        match self {
            Protocol::Http => "http",
            Protocol::Https => "https"
        }
    }
}

pub enum HttpMessageType {
    Request {method: String, request_target: Uri},
    Response {status_code: i16, reason_phrase: String}
}

pub struct HttpMessage {
    pub message_type: HttpMessageType,
    pub http_version: Option<String>,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpMessage {
    pub fn get(&self) -> String {
        let mut message: String = String::from("");

        let http_version = match &self.http_version {
            Some(t) => t,
            None => "HTTP/1.1"
        };

        if matches!(&self.message_type, HttpMessageType::Request{method: _, request_target: _}) {
            message.push_str(&format!("{0} {1} {2}\r\n",
                                      match &self.message_type {
                                          HttpMessageType::Request{method, request_target: _} => method,
                                          _ => panic!("") //FIXME THIS SHOULD DISAPPEAR IN THE FUTURE RELEASE
                                      },
                                      match &self.message_type {
                                          HttpMessageType::Request{method: _, request_target} => request_target.get_absolute(),
                                          _ => panic!("") //FIXME THIS SHOULD DISAPPEAR IN THE FUTURE RELEASE
                                      },
                                      http_version
            ));

            for (field_name, field_value) in &self.headers {
                message.push_str(&format!("{0}:{1}\r\n", field_name, field_value));
            }
            message.push_str("\r\n");
        }

        message
    }
}

pub fn get(uri: &str) -> String {
    let uri: Uri = Uri::parse_url(uri).unwrap(); //FIXME HANDLE ERROR

    let mut headers =  HashMap::new();
    headers.insert("Host".to_string(), uri.authority.clone());

    let msg = HttpMessage {
        message_type: HttpMessageType::Request{method: String::from("GET"), request_target: uri},
        http_version: None,
        headers,
        body: String::from(""),
    };

    let buffer = msg.get();

    let mut stream = std::net::TcpStream::connect(
            match msg.message_type {
                    HttpMessageType::Request {method: _, request_target} => request_target.authority,
                    _ => panic!() //FIXME Handle error
            }
    ).unwrap(); //FIXME Handle error

    stream.write(buffer.as_bytes()); //FIXME Handle error
    let mut read_buffer = [0 as u8; 1024]; //TODO Precise max buffer size
    stream.read_exact(&mut read_buffer); //FIXME Handle error

    from_utf8(&read_buffer).unwrap().to_string()
}
