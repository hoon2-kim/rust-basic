use std::io; // 모듈 사용 - 표준 라이브러리(내장) - 입출력


// Rust에서 함수를 선언하려면 fn을 사용하고 다음에 함수명을 넣는다.
fn main() {
    println!("Enter your weight (kg): ");
    // main 함수는 제일 먼저 실행되는 프로그램의 시작점이다.
    // Rust에서는 함수,변수명을 스네이크로 사용

    // 빈 문자열을 생성했고 이 문자열을 input이 소유한다. 
    // 컴파일 시점에 문자열의 크기를 모르기 때문에 힙에 저장된다. 스택에는 힙을 가리키는 포인터를 저장하고, 문자열의 크기같은 메타데이터를 추가로 저장
    // 크기를 모르는 이유는 String은 가변 크기(문자 수에 따라 크기가 달라지기 떄문)
    // input이 범위(스코프)를 벗어나면 힙에 있는 문자열은 해제됨 - 범위는 모르겠으면 그냥 자바스크립트 스코프 개념 떠올리자
    // 여기서 input은 main 함수가 끝나면 벗어난다.
    // 문자열은 스마트 포인터의 한 유형이다.
    let mut input = String::new(); 

    // some_fn(input);
    some_fn(&input);

    // let mut s =input; // input의 소유권을 s에게 이전, input은 이제 아무것도 소유하지 않음
    // io::stdin().read_line(&mut input); // input은 소유한게 없는데 쓰려고 해서 오류

    // Result는 Rust에서 오류 처리를 위한 열거형 Enum이다. 기본 구조로 Ok, Err이 있다.
    io::stdin().read_line(&mut input).unwrap(); // unwrap은 오류가 있으면 프로그램을 종료시킨다.

    let weight: f32= input.trim().parse().unwrap(); // parse는 문자열을 숫자로 변환, 성공하면 Ok, 실패하면 Err, 타입 추론이 필요
    dbg!(weight);


    println!("Input: {}",input);

    // 근데 스택 데이터들은 복사가 된다.
    // let a = 5;
    // let b = a;
    // 스택은 복사, 힙은 이동!

    // 변수 선언
    // Rust의 변수들은 기본적으로 불변 - 가변적으로 하려면 명시적으로 선언해야함(mut)
    let mut mars_weight = calculate_weight_on_mars(weight);
    mars_weight = mars_weight * 1000.0; // kg이 아니라 g로 하고싶다면

    // !가 있으면 함수 코드가 아니라 매크로를 의미
    // println이 매크로인 이유는 다수의 인수를 받을 수 있기 때문
    // println!("Hello, world!");
    // println!("Number: {}, String: {}", 100, "abcd");
    println!("Weight on Mars: {}kg", mars_weight);

    calculate_weight_on_mars(100.0);
}

// 예제 : 지구에서의 몸무게를 화성에서의 몸무게로 계산해주는 계산기
// 함수 매개변수에는 항상 유형을 구체적으로 명시해줘야함(스택)
// 함수의 반환타입은 -> 로 하면됨
fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711 // 표현식은 세미콜론이 필요없다.
}

// 함수도 마찬가지다. 위에서 some_fn(input)을 하면 s가 소유권을 가져가버려서 input은 아무것도 가진게 없게된다.
// fn some_fn(s:String) {

// }

// 하지만 &를 통해 참조를 사용하면 소유권은 이동안하고 참조(힙 주소)만 전달한다. 그리고 함수가 끝나면 참조만 사라지고 원본 데이터는 남아있다.
fn some_fn(s:&String) {
    // 참조도 참고로 변경이 불가능하다.
    // &mut로 하면 가변 참조로 변경이 가능하다.
    // 참조는 범위 안에서 여러개의 힙 데이터를 공유할 수 있다. 대신 가변, 불가변은 동시에 갖는건 불가능하다. 그리고 여러개의 가변 참조 동시도 불가능하다.
    // 즉, 불가변 참조 여러개 가능, 가변 참조 하나 가능, 가변과 불가변 동시 불가능, 가변 참조 여러개 불가능
}