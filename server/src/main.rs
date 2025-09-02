#![allow(dead_code)] // #!는 파일 전체에 적용

use http::Method;
use http::Request;
use server::Server;
use website_handler::WebsiteHandler;
use std::env;


mod http;
mod server; // server.rs 파일을 server라는 이름의 모듈로 가져옴 // http/ 디렉토리를 http라는 이름의 모듈로 가져옴
mod website_handler;

fn main() {
    // enums 사용법
    // let get = Method::GET;
    // let delete = Method::DELETE;
    // let post = Method::POST;
    // let put = Method::PUT;

    // 구조체::연관 함수, 구조체는 여러 데이터를 하나의 타입으로 묶어주는 사용자 정의 타입
    // 연관 함수는 구조체와 관련된 함수로, 보통 새로운 인스턴스를 생성할 때 사용, ::를 사용해서 호출(생성자 역할)
    // String같이 이미 정의되어 있는 구조체는 구현안해도 됨
    // "127.0.0.1:8080"과 같이 하드코딩된 문자열은 str(문자열 슬라이스)이다.
    let server = Server::new("127.0.0.1:8080".to_string()); // to_string은 문자열 슬라이스를 String으로 변환시켜 줌
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    println!("public path: {}", default_path);
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    server.run(WebsiteHandler::new(public_path));

    // 참고
    // let string = String::from("127.0.0.1:8080");
    // // 문자열 슬라이스(port만 뽑아낼 경우), 하지만 이렇게 자르는건 추천되지 않는다.
    // // Rust에서는 문자열은 UTF-8이고 항상 1바이트가 보장되지 않으므로
    // let string_slice = &string[10..];
    // let string_borrow:&str = &string; // 변수 string의 전체 내용을 참조하는 문자열 슬라이스
    // let string_literal = "1234";

    // // 그냥 string으로 하면 안된다. dbg!매크로는 내부적으로 string의 소유권을 가져가버린다.
    // // 얘가 소유권을 가져가 버리면 string_slice는 유효하지 않아진다. 그래서 참조로 전달한다.
    // dbg!(&string);
    // dbg!(string_slice);
    // dbg!(string_borrow);
    // dbg!(string_literal);
}
