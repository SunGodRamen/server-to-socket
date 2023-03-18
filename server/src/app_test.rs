mod tests {
    use crate::{get_handler, post_handler};
    use actix_web::{http, test, web, App};
    use std::io::Read;
    use std::net::{TcpListener, TcpStream};
    use std::sync::{Arc, Mutex};

    fn setup_test_server() -> (Arc<Mutex<TcpStream>>, TcpListener) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let socket = TcpStream::connect(address).unwrap();

        (Arc::new(Mutex::new(socket)), listener)
    }

    #[actix_rt::test]
    async fn test_get_handler() {
        let (socket, _listener) = setup_test_server();
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::clone(&socket)))
                .service(get_handler),
        )
        .await;
        let req = test::TestRequest::get().uri("/?key=value").to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let mut buffer = Vec::new();
        socket.lock().unwrap().read_to_end(&mut buffer).unwrap();
        assert_eq!(buffer, b"key=value");
    }

    #[actix_rt::test]
    async fn test_post_handler() {
        let (socket, _listener) = setup_test_server();
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::clone(&socket)))
                .service(post_handler),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .set_payload("key=value")
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let mut buffer = Vec::new();
        socket.lock().unwrap().read_to_end(&mut buffer).unwrap();
        assert_eq!(buffer, b"key=value");
    }
}
