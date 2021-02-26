#![forbid(unsafe_code)]
use regex::Regex;
use std::collections::HashMap;
use reinda::{assets, Assets, Config, Setup};
use lazy_static::lazy_static;


const ASSETS: Setup = assets! {
    // Folder which contains your assets, relative to your `Cargo.toml`.
    #![base_path = "Web"]

    // List of assets to include, with different settings.
    "index.html": { template },
    "404.html": { template },
};

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

lazy_static! {
    //Kind of verbose, but processes once and never again
    static ref GET_COMBINED_PATH_RE: Regex =  Regex::new(&format!("({}) ({})", GET_RE, PATH_RE)).unwrap();
    static ref POST_COMBINED_PATH_RE: Regex = Regex::new(&format!("({}) ({})", POST_RE, PATH_RE)).unwrap();

    static ref GET_VALID_HTTP_REQUEST_HEADER_RE: Regex = Regex::new(&format!("({}) ({}) ({})", GET_RE, PATH_RE, HTTP_VER_RE)).unwrap_or(Regex::new("").unwrap());
    static ref POST_VALID_HTTP_REQUEST_HEADER_RE: Regex = Regex::new(&format!("({}) ({}) ({})", POST_RE, PATH_RE, HTTP_VER_RE)).unwrap();
    
    static ref DATA_REGEX: Regex = Regex::new(DATA_RE).unwrap();
}

//I know making it always give back a tuple is a stupid solution, and slightly wasteful, but it
// works so I probably won't be changing it for now.

//The tuple is: (response_string, path, post req data, get req data)
pub async fn validate_request(buffer: String) -> (String, String, Option<String>, Option<String>) {            
    //This is wasteful, since it performs the Regex check twice
    return if Regex::new(GET_RE).unwrap().is_match(&buffer) {
        let get_searched_captures = 
            GET_COMBINED_PATH_RE
            .captures(&buffer)
            .unwrap();
    
        let path =
            get_searched_captures
            .get(2)
            .map_or("", |m| m.as_str());
    
        let request = GET_COMBINED_PATH_RE
            get_searched_captures
            .get(3)
            .map_or("", |m| m.as_str());

        let get_data = match request.find("?") {
            Some(i) => Some(request[i + 1..].to_string()),
            None => None,
        };

        (
            request_regex_check(&buffer, GET_RE).await,
            path.to_string(),
            None,
            get_data,
        )
    } else if Regex::new(POST_RE).unwrap().is_match(&buffer) {
        let path =
        POST_COMBINED_PATH_RE
        .captures(&buffer)
        .unwrap()
        .get(2)
        .map_or("", |m| m.as_str());
    
        let data = DATA_REGEX
            .captures(&buffer)
            .unwrap()
            .get(0)
            .map_or("", |m| m.as_str());

        (request_regex_check(&buffer, POST_RE).await, path.to_string(), Some(data.to_string()), None,)
    } else {
        (bad_request_page(&buffer).await, "/404.html".to_string(), None, None)
    };
}

async fn format_response(mut page: &str, response_code: u16) -> Result<String, Box<dyn std::error::Error> > {
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
    
    let assets = Assets::new(ASSETS, Config::default()).await?;
    
    let embedded_page_bytes = match assets.get(page).await? {
        Some(b) => b,
        None => {
            if page == "" {
                assets.get("index.html").await?.unwrap()
            } else {
                assets.get("404.html").await?.unwrap()
            }
        },
    };
    
    
    let embedded_page_str = String::from_utf8_lossy(&embedded_page_bytes).to_string();

    Ok(format!(
        "HTTP/1.1 {} {} Content-Length: {:?}\r\n\r\n{}",
        response_code,
        RESPONSE_CODE_HASHMAP.get(&response_code).unwrap(),
        embedded_page_str.len(),
        embedded_page_str
    ))
}

async fn bad_request_page(buffer: &String) -> String{
    //Since bad response is used so often in the if tree, I made it into a function
    //Not using the format::response function since the response itself changes depending on the request
    let page = format!("{}\n{}", BAD_REQUEST_RESPONSE_RE_STR, buffer);
    format!(
        "{} {:?}\r\n\r\n{}",
        BAD_REQUEST_HEADER_RE_STR,
        page.len(),
        page
    )
}

async fn request_regex_check (buffer: &String, request_type_re: &'static str) -> String {    
    let (combined_path_re, valid_http_request_header_re) = {
        if request_type_re == GET_RE {
            (&*GET_COMBINED_PATH_RE, &*GET_VALID_HTTP_REQUEST_HEADER_RE)
        } else {
            (&*POST_COMBINED_PATH_RE, &*POST_VALID_HTTP_REQUEST_HEADER_RE)
        }
    };
    
    let index: usize = 2;

    if combined_path_re.is_match(&buffer) && valid_http_request_header_re.is_match(&buffer) {
        format_response(
            combined_path_re.captures(&buffer).unwrap().get(index).map_or("", |m| m.as_str()),
            200,
        ).await.unwrap()
    } else {
        bad_request_page(&buffer).await
    }
}
