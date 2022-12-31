use hyper::HeaderMap;

pub async fn custom_header(headers: HeaderMap) -> String {
    headers.get("Hoge").unwrap().to_str().unwrap().to_owned()
}
