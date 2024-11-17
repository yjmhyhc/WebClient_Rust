use structopt::StructOpt;
use url::{Url, ParseError};
use reqwest::blocking;
use reqwest::blocking::Client;
use serde_json::json;
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};

/// command line arguments structure
#[derive(StructOpt, Debug)]
#[structopt(name = "WebClient")]
struct Opt {
    /// the URL that we send a HTTP request to
    url: String,

    /// the method that we send a request, can be POST
    #[structopt(short="X")]
    x: Option<String>,

    /// the data that we send when using POST method
    #[structopt(short)]
    data: Option<String>,

    /// if we are sending JSON formatted data
    #[structopt(long)]
    json: Option<String>,
}

fn main() {
    // parsing command line arguments
    let opt = Opt::from_args();

    // match the result and send different HTTP requests
    match opt.json {
        Some(json) => {
            //send POST request carrying json formatted data
            send_json(&opt.url[..], &json[..]);
        },
        None => {
            match opt.x {
                Some(_) => {
                    //send with the method POST
                    let data = opt.data.unwrap();
                    send_post(&opt.url[..], &data[..]);
                },
                None => {
                    //send with the method GET
                    send_get(&opt.url[..]);
                }
            }
        }
    }
}


///this function sends GET request to the server
fn send_get(url: &str){
    println!("Requesting URL: {}", url);
    println!("Method: GET");
    //handling URL errors
    if !validate_url(url) {
        return;
    }
    //now the URL is valid, let's send a GET request
    if let Ok(response) = blocking::get(url){
        //if there is a response, we should check the status code
        if response.status().is_success() {
            //if the status is successful
            if let Ok(response_body) = response.text() {
                println!("Response body:");
                print!("{}", response_body);
            }
        } else {
            //if the status is unsuccessful
            println!("Error: Request failed with status code: {}.", response.status().as_u16());
        }
    }else {
        //if there is no response
        println!("Error: Unable to connect to the server. Perhaps the network is offline or the server hostname cannot be resolved.");
    }
}


///this function converts "data" (a string slice) to serde_json::Value and sends POST request
fn send_post(url: &str, data: &str) {
    println!("Requesting URL: {}", url);
    println!("Method: POST");
    println!("Data: {}", data);
    //handling URL errors
    if !validate_url(url) {
        return;
    }
    //*JSON FORMATTING THE DATA
    //*.1 initialize a hashmap to store the parameters
    let mut params: HashMap<String, String> = HashMap::new();

    //*.2 divide "data" into key-value pairs
    for pair in data.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            params.insert(key.to_string(), value.to_string());
        }
    }

    //*.3 convert the hashmap to json value
    let json_value = json!(params);

    //send a post request
    post(url, &json_value);
}


///this function converts "json" (a string slice) to serde_json::Value and sends a POST request
fn send_json(url: &str, json: &str) {
    println!("Requesting URL: {}", url);
    println!("Method: POST");
    println!("JSON: {}", json);
    //JSON FORMATTING THE DATA
    match serde_json::from_str(json) {
        Ok(json_value) => {
            post(url, &json_value);
        },
        Err(e) => {
            //if format is wrong, panic
            panic!("Invalid JSON: Error({})", e);
        }
    }
}


///this function sends POST request
fn post(url: &str, json_value: &serde_json::Value) {
    let client = Client::new();
    if let Ok(response) = client.post(url).json(&json_value).send(){
        //if there is a response, we should check the status code
        if response.status().is_success() {
            //if the status is successful
            if let Ok(response_body) = response.text() {
                //sort the json value
                if let Ok(response_json) = &mut serde_json::from_str(&response_body[..]){
                    let sorted_json_str = sort_json_keys(response_json);
                    println!("Response body (JSON with sorted keys):");
                    print!("{}", sorted_json_str);
                }else {
                    println!("Response body:");
                    print!("{}",&response_body[..]);
                }
            }
        } else {
            //if the status is unsuccessful
            println!("Error: Request failed with status code: {}.", response.status().as_u16());
        }
    }else {
        //if there is no response
        println!("Error: Unable to connect to the server. Perhaps the network is offline or the server hostname cannot be resolved.");
    }
}


///this function checks if the url is valid
fn validate_url(url: &str) -> bool{
    //data:// is an exception, because it is viewed as a valid base protocol by the url crate
    if url.starts_with("data://") {
        println!("Error: The URL does not have a valid base protocol.");
        return false;
    }
    match Url::parse(url) {
        Ok(_) => {
            return true;
        },
        Err(parse_error) => {
            match parse_error {
                ParseError::RelativeUrlWithoutBase => println!("Error: The URL does not have a valid base protocol."),
                ParseError::InvalidIpv6Address => println!("Error: The URL contains an invalid IPv6 address."),
                ParseError::InvalidIpv4Address => println!("Error: The URL contains an invalid IPv4 address."),
                ParseError::InvalidPort => println!("Error: The URL contains an invalid port number."),
                _ => {}
            }
            return false;
        }
    }
}


///this function sorts a serde_json::Value according to the alphabetical order of the keys
fn sort_json_keys(value: &mut Value) -> String{
    // store the map into a BTreeMap, which automatically sort the key-value pairs
    let sorted_map: BTreeMap<&String, &Value> = value.as_object().unwrap().into_iter().collect();
    return serde_json::to_string_pretty(&json!(sorted_map)).unwrap();
}

