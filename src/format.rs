#![forbid(unsafe_code)]
use regex::Regex;
use rust_embed::RustEmbed;
use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(RustEmbed)]
#[folder = "Public"]
struct Asset;

//All the regexes used to validate the HTTP header
const GET_RE: &str = r"^GET";
const PATH_RE: &str = r"/([A-Za-z0-9_./%?&()+:=-]*|)";
const HTTP_VER_RE: &str = r"(HTTP/1\.1)";

//POST request specific regexes
const POST_RE: &str = r"^POST";
// let content_length_re = r"\r\nContent-Length: [0-9]*\r\n";
const DATA_RE: &str = r"\r\n\r\n[A-Za-z0-9_./%?&()+:=-]*";

const BAD_REQUEST_RESPONSE_RE_STR: &str =
    "400 Error, Bad Request!, screenshot this and send it to Will:";
const BAD_REQUEST_HEADER_RE_STR: &str = "HTTP/1.1 400 BAD REQUEST Content-Length:";

//I know making it always give back a tuple is a stupid solution, and slightly wasteful, but it
// works so I probably won't be changing it for now.

//The tuple is: (response_string, path, post req data, get req data)
pub fn validate_request(buffer: String) -> (String, String, Option<String>, Option<String>) {
    //Since bad response is used so often in the if tree, I made it into a closure
    let bad_request = || -> String {
        //Not using the format::response function since the response itself changes depending on the request
        let page = format!("{}\n{}", BAD_REQUEST_RESPONSE_RE_STR, buffer);
        format!(
            "{} {:?}\r\n\r\n{}",
            BAD_REQUEST_HEADER_RE_STR,
            page.len(),
            page
        )
    };

    lazy_static! {
        //Kind of verbose, but processes once and never again
        static ref GET_COMBINED_PATH_RE: Regex =  Regex::new(&format!("({}) ({})", GET_RE, PATH_RE)).unwrap();
        static ref POST_COMBINED_PATH_RE: Regex = Regex::new(&format!("({}) ({})", POST_RE, PATH_RE)).unwrap();

        static ref GET_VALID_HTTP_REQUEST_HEADER_RE: Regex = Regex::new(&format!("({}) ({}) ({})", GET_RE, PATH_RE, HTTP_VER_RE)).unwrap_or(Regex::new("").unwrap());
        static ref POST_VALID_HTTP_REQUEST_HEADER_RE: Regex = Regex::new(&format!("({}) ({}) ({})", POST_RE, PATH_RE, HTTP_VER_RE)).unwrap();

    }

    let mut path = "";

    let mut request_regex_check = |request_type_re: &str| -> String {
        let (combined_path_re, valid_http_request_header_re) = {
            if request_type_re == GET_RE {
                path = GET_COMBINED_PATH_RE
                    .captures(&buffer)
                    .unwrap()
                    .get(2)
                    .map_or("", |m| m.as_str());
                (&*GET_COMBINED_PATH_RE, &*GET_VALID_HTTP_REQUEST_HEADER_RE)
            } else {
                path = POST_COMBINED_PATH_RE
                    .captures(&buffer)
                    .unwrap()
                    .get(2)
                    .map_or("", |m| m.as_str());
                (&*POST_COMBINED_PATH_RE, &*POST_VALID_HTTP_REQUEST_HEADER_RE)
            }
        };

        path = match path.find("?") {
            Some(i) => &path[..i],
            None => path,
        };

        if combined_path_re.is_match(&buffer) && valid_http_request_header_re.is_match(&buffer) {
            format_response(
                combined_path_re
                    .captures(&buffer)
                    .unwrap()
                    .get(2)
                    .map_or("", |m| m.as_str()),
                200,
            )
        } else {
            bad_request()
        }
    };

    //This is wasteful, since it performs the Regex check twice
    return if Regex::new(GET_RE).unwrap().is_match(&buffer) {
        let request = GET_COMBINED_PATH_RE
            .captures(&buffer)
            .unwrap()
            .get(3)
            .map_or("", |m| m.as_str());

        let get_data = match request.find("?") {
            Some(i) => Some(request[i + 1..].to_string()),
            None => None,
        };

        (
            request_regex_check(GET_RE),
            path.to_string(),
            None,
            get_data,
        )
    } else if Regex::new(POST_RE).unwrap().is_match(&buffer) {
        lazy_static! {
            static ref DATA_REGEX: Regex = Regex::new(DATA_RE).unwrap();
        }
        
        let data = DATA_REGEX
            .captures(&buffer)
            .unwrap()
            .get(0)
            .map_or("", |m| m.as_str());

        (request_regex_check(POST_RE), path.to_string(), Some(data.to_string()), None,)
    } else {
        (bad_request(), "/404.html".to_string(), None, None)
    };
}

fn format_response(mut page: &str, mut response_code: u16) -> String {
    //I decided to use a HashMap, in case this can be used later down the road
    //Only implemented a few response codes for simplicity
    lazy_static! {
        static ref RESPONSE_CODE_HASHMAP: HashMap<u16, &'static str> = {
            let mut m = HashMap::new();
            m.insert(200, "OK");
            m.insert(400, "BAD REQUEST");
            m.insert(404, "NOT FOUND");
            m.insert(500, "INTERNAL SERVER ERROR");
            m
        };
    }

    //Strip the page of its slash
    page = &page[1..];

    page = match page.find("?") {
        Some(index) => &page[..index],
        None => page,
    };

    let not_found_closure = || {
        if page == "" {
            Asset::get("page.html").unwrap()
        } else {
            response_code = 404;
            Asset::get("404.html").unwrap()
        }
    };

    //Just get the page using rust_embed
    let embedded_page = Asset::get(page).unwrap_or_else(not_found_closure);
    //Then convert that page into a string
    //The extra to_string is to solve returns a value referencing data owned by the current function
    let embedded_page_str: String = std::str::from_utf8(embedded_page.as_ref())
        .unwrap()
        .to_string();

    format!(
        "HTTP/1.1 {} {} Content-Length: {:?}\r\n\r\n{}",
        response_code,
        RESPONSE_CODE_HASHMAP.get(&response_code).unwrap(),
        embedded_page_str.len(),
        embedded_page_str
    )
}
