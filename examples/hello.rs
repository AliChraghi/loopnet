use loopnet;
fn main() {
    let server = loopnet::HttpServer::new([127, 0, 0, 1], 8020);
    server.listen();
}
