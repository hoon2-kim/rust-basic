// 모듈
// 필요한곳에서 mod로 불러와 사용
// pub : 외부에서 접근 가능
// 모듈안에서 기본적으로는 private
// 구조체는 데이터 타입을 정의하며 실제 메모리에 저장이된다.

use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::{Read, Write};
use std::net::TcpListener;
// use std::string::ParseError; // 내장

use crate::http::{Request, Response, StatusCode,ParseError}; // super는 상위 모듈, crate는 크레이트 루트 - server와 http/는 같은 레벨의 모듈이라서 crate 써야함

// 커스텀 트레이트 - 특정 동작이나 기능을 추상화하여 여러 타입이 공통으로 구현할 수 있게 함 
pub trait Handler {
    // 필수 메서드
    fn handle_request(&mut self,request: &Request) -> Response;

    // 기본 구현이 있는 메서드 (선택적으로 오버라이드 가능)
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String, // ip와 port를 담고있는 문자열
}

// 구조체에 구현(imple) - 구조체에 기능 추가
impl Server {
    // 구조체에 속한 함수들을 구현(구조체에 메서드와 연관 함수를 추가)

    // 연관 함수(생성자)
    // Self는 현재 구조체 타입
    // javascript 개념으로 생각해보자!
    // class Server {
    //     constructor(addr) {
    //         this.addr = addr;
    //     }
    // }
    // // new 키워드로 인스턴스 생성 + 반환
    // const server = new Server("127.0.0.1");
    pub fn new(addr: String) -> Self {
        // Self = Server
        // 인스턴스가 생성되고 반환됨
        Self {
            // addr: addr
            addr,
        }
    }

    // 인스턴스 메서드, self는 구조체의 인스턴스를 소유권으로 받음, 이 함수를 호출하면 구조체 인스턴스의 소유권이 함수로 이동
    // 즉, run 함수가 self의 소유권을 가져간다. 함수가 끝나면 구조체는 할당이 해제됨
    // 매개변수 앞에 &를 붙이면 불변 참조, &mut는 가변 참조 그리고 해제는 안된다.(참조기 떄문에)
    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap(); // 참조로 받아야 계속해서 어디서든 쓰일 수 있음
        // Result Enum은 복구가능한 오류를 다룬다.
        // 하지만 여기선 주소가 중복되서 소켓 연결 실패하는 상황을 고려해 복구 불가능한 오류로 바꿔줄거다(unwrap).

        // 무한 반복문
        loop {
            // match - 패턴 패팅을 위한 강력한 제어 구조, 모든 경우를 처리해야함, 표현식이므로 값을 반환할 수 있다.
            // 모든 경우를 처리 해야한다는 건 match뒤에 코드가 반환하는 모든 경우를 처리해야한다. _는 나머지 모든 경우 처리
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            // [..]는 전체범위, 배열을 슬라이스로 변환
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                   handler.handle_request(&request)
                                }
                                Err(e) => handler.handle_bad_request(&e)
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    };
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }

            // 새로운 연결이 있는지 확인
            let res = listener.accept(); // 새로운 연결이 있을 때 까지 여기 머무름

            if res.is_err() {
                continue;
            }

            let (stream, addr) = res.unwrap();

            // 튜플 - 여러 타입의 값을 하나로 묶는 데이터 구조, 고정된 크기(한번 생성하면 끝), 인덱스로 접근하므로 순서가 중요
            // let tup: (i32, &str, std::net::TcpListener) = (5, "a", listener);
        }
    }
}
