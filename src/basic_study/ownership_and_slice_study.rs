/*
https://rinthel.github.io/rust-lang-book-ko/ch04-01-what-is-ownership.html
https://rinthel.github.io/rust-lang-book-ko/ch04-02-references-and-borrowing.html
https://rinthel.github.io/rust-lang-book-ko/ch04-03-slices.html
 */
fn wrong_clear_string() {
    let mut a: String = String::from("Hello from the world");
    let b = first_word(&a);
    println!("First Word :  {}", &b[..]); // first_word에서 쓰기위해 a를 불변하게 빌림.
                                          //   a.clear();
    println!("a length :  {}", a.len()); // a에 접근 하기 -> 문제 X

    // b는 결국 &a에 slice 한것과 동일.
    // 즉 여기까지 a의 불변성이 유지되어야함(&a로 불변 참조 했었음)
    // 그러나 위의 a.clear()로 인해 a가 불변하지 않게 되버림
    // -> 그러므로 컴파일 불가
    println!("b length:  {}", b.len());
}

pub fn safe_clear_string() {
    let mut a: String = String::from("Hello from the world");
    let b = first_word(&a); // first_word에서 쓰기위해 a를 불변하게 빌림
    let c = b.len(); // length를 구함
    println!("First Word :  {}", &b[..]);
    a.clear();
    println!("a length :  {}", a.len());
    println!("b length:  {}", c);
    // b가 아닌 c로 접근했기 때문에,
    // a를 가변하게 참조하는 코드 이후(a.clear)로 a를 불변하게 참조했었던 변수들이 참조하는 것은 없음
    // 고로 문제 없이 컴파일 가능
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub fn second_mutable() {
    let mut s = String::from("hello");
    println!("original : '{}'", s);
    {
        let r1 = &mut s;
        r1.clear();
    } // 스코프가 끝남에 따라 가변 참조 종료
    println!("r1 : '{}'", s);
    {
        let r2 = &mut s; // 가변 참조가 없기 때문에 새로운 가변참조 가능.
        r2.push_str("aaa");
    } // 스코프가 끝남에 따라 가변 참조 종료
    println!("r2: '{}'", s);
}
