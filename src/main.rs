use structopt::StructOpt;
use url::{Url, ParseError};
use reqwest::blocking;

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

fn send_post(url: &str, data: &str) {
    println!("post sent, url:{}, data:{}", url, data)
}

fn send_json(url: &str, json: &str) {
    println!("json sent, url:{}, json:{}", url, json)
}

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
