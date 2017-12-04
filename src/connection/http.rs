/**
    @author anlaakso (Andrew)
    @license GNU/GPLv3
*/


use std::str;

use futures::{ Future, Stream };
use connection::response::{ SuccessResponse };
use connection::params::ConnectionParams;
use connection::verbs::Verbs;
use config::ArangoConfig;
use generics::connect::{ Connect };
use hyper::client::HttpConnector;
use tokio_core::reactor::Core;
use hyper::{ Client, Method, Request , Uri };
use hyper::header::{ ContentType };

pub struct HttpConnection {
    core: Core,
    client: Client<HttpConnector>,
}

impl Connect for HttpConnection {
    
    fn new() -> Self {
        let core = Core::new().unwrap();
        let client = Client::new(&core.handle());

        HttpConnection {
            core,
            client,
        }
    }

    fn connect(&mut self, config: &ArangoConfig, params: &ConnectionParams) -> Result<SuccessResponse, String> {

        let &ArangoConfig { host, port } = config;
        let url: Uri = format!("http://{}:{:?}/_api/{}", host, port, params.path).parse().unwrap();
        let verb: Method = match params.method {
            Verbs::GET => Method::Get,
            Verbs::POST => Method::Post,
            _ => Method::Get
        };

        let mut req = Request::new(verb, url);
        let json = params.json.unwrap().to_owned();
        
        req.headers_mut().set(ContentType::json());
        req.set_body(json);

        let task = self.client
            .request(req)
            .and_then(|response| {
                response.body().concat2()
            })
            .map(|body| {
                str::from_utf8(&body).unwrap().to_owned()
            })
            .map(|body| {
                Ok::<String, String>(body)
            })
            .map_err(|err| {
                println!("{:?}", err);
                Err::<String, String>(err.to_string())
            });

        let result = self.core.run(task).unwrap();

        println!("{:?}", result);

        Ok(SuccessResponse { code: 200, json: "{}".to_owned() })
    }
}
