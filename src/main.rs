use tiny_http::{Server, Request, Response, Header, Method, StatusCode};
use std::fs::File;

fn serve_file_type(request: Request, file_path: &str, content_type: &str) -> Result<(), ()> {
    let content_type_header = Header::from_bytes(
        "Content-Type", content_type 
        ).unwrap();

    let file = File::open(&file_path).map_err(|err| {
        eprintln!("ERROR: Could not serve file {file_path}: {err}");
    })?;

    let response = Response::from_file(file).with_header(content_type_header);
    request.respond(response).map_err(|err| {
        eprintln!("ERROR: could not serve a request: {err}");
    })?;

    Ok(())
}

fn serve_404(request: Request) -> Result<(), ()> {
    request.respond(Response::from_string("404")
                    .with_status_code(StatusCode(404)))
                    .map_err(|err| {
        eprintln!("ERROR: could not serve a request: {err}");
    })?;
    Ok(())
}

fn serve_request(mut request: Request) -> Result<(), ()> {
    println!("INFO: request received: method: {:?}, url: {:?}",
        request.method(),
        request.url(),
    );

    match (request.method(), request.url()) {
        (Method::Post, "/api/search") => {
            let mut buf = Vec::new();
            request.as_reader().read_to_end(&mut buf).unwrap();
            let body = std::str::from_utf8(&buf).map_err(|err| {
                eprintln!("ERROR: could not interpret body as UTF-8 string: {err}");
            })?;
            println!("Search: {body}");
            request.respond(Response::from_string("ok")).map_err(|err| {
                eprintln!("ERROR: could not serve request: {err}");
            })?;
        }
        (Method::Get, "/edit-dst-831f930dc6943d00_bg.wasm") => {
            serve_file_type(
                request, 
                "dist/edit-dst-831f930dc6943d00_bg.wasm", 
                "application/wasm"
            )?;
        }
        (Method::Get, "/edit-dst-831f930dc6943d00.js") => {
            serve_file_type(
                request, 
                "dist/edit-dst-831f930dc6943d00.js", 
                "text/javascript"
            )?;
        }
        (Method::Get, "/index-a0fde7b4d2554044.css") => {
            serve_file_type(
                request,
                "dist/index-a0fde7b4d2554044.css",
                "text/css; charset=utf-8"
            )?;
        }
        (Method::Get, "/") | (Method::Get, "/index.html") => {
            serve_file_type(
                request,
                "dist/index.html",
                "text/html; charset=utf-8"
            )?;
        }
        _ => {
            serve_404(request)?;
        }
    }


    Ok(())
}

fn main() {
    let address = "127.0.0.1:6969".to_string();
    let server = Server::http(&address).map_err(|err| {
        eprintln!("ERROR: could not start HTTP server at {address}: {err}");
    }).unwrap();
    println!("INFO: Running on http://{address}");
    for request in server.incoming_requests() {
        
        serve_request(request).unwrap();
    }
    println!("Hello, world!");
}
