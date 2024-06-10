use reqwest::blocking::{Client, ClientBuilder};
use reqwest::redirect::Policy;

pub fn reqwests_test() {
    let http_client = Client::new();
    let http_result = http_client.get("https://ttrevorsullivan.net").send();

    if http_result.is_ok() {
        println!("{:#?}", http_result.ok().unwrap().text().unwrap());
    } else if http_result.is_err() {
        println!("Error occurred {:#?}", http_result.err());
    }

    let post_request = http_client
        .post("https://someurl")
        .body("{\"first_name\":\"SomeName\"}")
        .send();
    println!("{:#?}", post_request.ok().unwrap().text().unwrap());

    //redirects
    let redir_policy = Policy::limited(5);

    let http_client = ClientBuilder::new()
        .redirect(redir_policy)
        .build()
        .ok()
        .unwrap();
    let http_result = http_client
        .get("apihere")
        .send()
        .ok()
        .unwrap()
        .text()
        .unwrap();

    println!("The result {:#?}", http_result);
}

//this is incomplete needs tests with postman mockoon
