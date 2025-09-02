use crate::http::request;

// 같은 모듈의 위치에 있어서 상위로 올라가 찾아줘야함
use super::method::{Method, MethodError};

use super::{QueryString, QueryStringValue};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

// 읽기만 하는 경우에는 힙에 불필요한 메로리를 할당해야 하는 String 보단 &str 사용이 낫다.
// 'buf -> Request 구조체가 'buf라는 수명을 가진 참조들을 포함한다는 의미
// 밑의 두 필드는 같은 수명을 가져아 한다는 제약을 의미
#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>, // Option enum으로 값이 없을 수 있거나(None) T가 String이기 때문에 문자열이 될 수 있음
    method: Method,
}

// Getter
impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString<'buf>> {
        self.query_string.as_ref()
    }
}

// impl Request<'buf> {
//     // 메서드 - 바이트 배열을 받아서 성공하면 Request(Self)객체, 실패하면 에러 메시지 반환
//     fn from_byte_array(buf: &[u8]) -> Result<Self, String> {}
// }

// TryFrom 트레이트 구현, TryFrom 트레이트를 구현하면 try_into 자동으로 사용 가능
// TryFrom은 Rust 표준 라이브러리의 트레이트, 한 타입을 다른 타입으로 변환할 때 사용, 변환이 실패할 수 있는 경우에 사용
// 여기서의 for는 ~에 대해라는 의미의 제네릭 타입 매개변수
// 의미 : &[u8]타입에서 Request로 변환하는 TryFrom 트레이트를 구현한다.
// Nestjs의 클린 아키텍처를 생각하자! 트레이트는 인터페이스의 업그레이드 버전이라 생각!
// 입력 및 반환 모두 같은 수명을 가져야 함('buf), Request가 원본 버퍼보다 오래 살 수 없음을 보장
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        // match str::from_utf8(buf) {
        //     // request는 from_utf8의 성공 결과(바이트 배열 -> 문자열 슬라이스), 성공하면 계속 진행
        //     Ok(request) =>{

        //     }
        //     Err(_) => return Err(ParseError::InvalidEncoding), // 실패시 return으로 함수 종료
        // }

        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {}
        //     Err(e) => return Err(e)
        // }

        // 오류 구현에서 위의 두 방식과 똑같다. 하지만 이게 더 간단한다. 대신 밑에서 From을 구현 해줘야한다.
        let request = str::from_utf8(buf)?;

        // ok_or은 Option을 Result로 변환, Option<T> -> Result<T,E>
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        // parse는 FromStr트레이트 사용
        // ?는 에러가 나면 바깥 함수의 에러 타입으로 변환해 전파, MethodError -> ParseError로 변환
        let method: Method = method.parse()?;

        let mut query_string = None; // 쿼리 스트링이 없을 수도 있으니 시작값은 None으로 함
        // find는 ?의 위치를 찾는다. 있으면 Some(인덱스), 없으면 None
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        // unimplemented!() // 아직 구현하지 않았다는 의미, 컴파일은 되지만 실행하면 패닉

        Ok(Self {
            path: path,
            query_string,
            method,
        })
    }
}

// 헬퍼 함수 - 문자열에서 첫 번째 공백을 찾아서 단어를 분리
// Option은 값이 있을 수도 있고 없을 수도 있는 타입을 표현 - Some(값이 있는 경우), None(없는 경우)
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    //    let mut iter = request.chars();
    //    loop {
    //     let item = iter.next();
    //     match item {
    //         Some(c) => {}
    //         None => break
    //     }
    //    }

    // i - 인덱스, c - 문자
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            // 공백또는 캐리지를 찾으면 분리
            // "GET /users HTTP/1.1"에서
            // i = 3 (공백의 위치)
            // &request[..i] = "GET" (0부터 2까지)
            // &request[i + 1..] = "/users HTTP/1.1" (4부터 끝까지)
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

// Display 트레이트 - 사용자에게 보여줄 문자열 형식 정의
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

// Debug 트레이트 - 디버깅용 문자열 형식 정의
impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
