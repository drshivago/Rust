#[allow(unused)]
fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}
fn second_word(s: &str) -> &str{

    let first = get_index(&s);
    let second = first + get_index(&s[first..]);

    &s[first..first + second]

}
fn get_index(s: &str) -> usize{

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i
        }
    }
    s.len()
}
fn main(){
    let s = String::from("Aids and penus");
    let t = first_word(&s);
    let u = second_word(&s);
    println!("{},{}", t, u);
}
