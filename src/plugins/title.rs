extern crate reqwest;

use self::reqwest::*;
use haze::middleware::Middleware;

pub fn title() -> Middleware {
    Middleware::new("link title")
}
// let fn = 0;
// let client = Client::new();
// let mut resp = client.head("http://httpbin.org/bytes/3000").send()?;
// if resp.status().is_success() {
//     let len = resp.headers().get::<ContentLength>()
//                 .map(|ct_len| **ct_len)
//                 .unwrap_or(0);
//     // limit 1mb response
//     if len <= 1_000_000 {
//         let mut buf = Vec::with_capacity(len as usize);
//         let mut resp = reqwest::get("http://httpbin.org/bytes/3000")?;
//         if resp.status().is_success() {
//             ::std::io::copy(&mut resp, &mut buf)?;
//         }
//     }
// }
