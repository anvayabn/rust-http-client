mod client;
mod config;


fn main() {
    /* Get options from the main program  */
    let args = config::Args::new();

    let http_client = client::HttpClient::new(args); 

    println!("HTTP Client Configuration: {:?}", http_client);

    /* Run Http client with provided configuration */
    // start_client();

    // /* show stats */
    // show_stats();

    /* done */

}
