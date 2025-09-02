use std::collections::HashMap; // 해시맵

// a=1&b=2&c&d=&e===&d=7&d=abc

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new(); // 빈 HashMap 생성

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            // and_modify() - 키가 이미 존재하면 수정, or_insert() - 키가 없으면 새로 삽입
            // and_modify()는 클로저를 받는다. 클로저는 익명 함수고 이름이 없는 함수다. ||안에 들어가고 ||를 {}의 일을 해줘라는거다.
            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        *existing = Value::Multiple(vec![prev_val, val]);
                    }
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val));
        }

        QueryString { data } // QueryString 구조체에 data(해시맵)를 포장
    }
}

// Vec<T>는 동적 배열이다. 크기가 변경 가능한 컨테이너로 힙에 저장된다.
