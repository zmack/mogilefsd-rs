use iron::{Handler, IronResult, Request, Response};
use iron::status::Status;
use super::Storage;

pub struct StorageHandler;
// {
//     store: Storage,
// }

impl StorageHandler {
    pub fn new(_: Storage) -> StorageHandler {
        StorageHandler
        // {
        //     store: storage,
        // }
    }

    // fn handle_get(&self, request: &Request, domain: &str, key: &str) -> IronResult<Response> {
    //     let mut content = vec![];
    //     match self.store.get_content(domain, key, &mut content) {
    //         Ok(_) => {},
    //         Err(StorageError::UnknownKey) => {
    //             return Ok(Response::with((Status::NotFound, "Unknown key.\n")));
    //         },
    //         Err(StorageError::NoContent) => {
    //             return Ok(Response::with((Status::NotFound, "No content for key.\n")));
    //         },
    //         Err(e) => {
    //             let modifier = (Status::InternalServerError, format!("{}\n", e.description()));
    //             return Err(IronError::new(e, modifier));
    //         },
    //     };

    //     let mut response = Response::with((Status::Ok,));
    //     response = response.set(Header(headers::ContentLength(content.len() as u64)));

    //     if request.method == Method::Get {
    //         response = response.set(content);
    //     }

    //     Ok(response)
    // }

    // fn handle_put(&self, request: &mut Request, domain: &str, key: &str) -> IronResult<Response> {
    //     match self.store.store_content(domain, key, &mut request.body) {
    //         Ok(_) => Ok(Response::with((Status::Ok,))),
    //         Err(StorageError::UnknownKey) => {
    //             return Ok(Response::with((Status::NotFound, "Unknown key.\n")));
    //         },
    //         Err(e) => {
    //             let modifier = (Status::InternalServerError, format!("{}\n", e.description()));
    //             return Err(IronError::new(e, modifier));
    //         },
    //     }
    // }
}

impl Handler for StorageHandler {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let dk = domain_and_key_from_path(&request.url.path);

        if dk.is_err() {
            return Ok(Response::with((Status::BadRequest, format!("{}\n", dk.unwrap_err()))));
        }

        let (domain, key) = dk.unwrap();
        info!("Storage request: {:?} domain = {} key = {} from {:?}", request.method, domain, key, request.remote_addr);

        match request.method {
            // Method::Get | Method::Head => self.handle_get(request, &domain, &key),
            // Method::Put => self.handle_put(request, &domain, &key),
            _ => Ok(Response::with((Status::BadRequest, "Unknown request type.\n"))),
        }
    }
}

fn domain_and_key_from_path(path: &Vec<String>) -> Result<(String, String), String> {
    let d_index = path.iter().position(|p| p == "d");
    let k_index = path.iter().position(|p| p == "k");

    match (d_index, k_index) {
        (Some(d), Some(k)) => {
            let domain = path[(d+1)..k].connect("/");
            let key = path[(k+1)..].connect("/");
            (Ok((domain, key)))
        },
        _ => {
            Err(format!("Could not extract domain or key from path: {:?}", path))
        }
    }
}
