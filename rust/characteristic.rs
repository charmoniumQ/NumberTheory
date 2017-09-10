fn main() {
    println!("hello world");
}

fn index(a: u16, b: u16) -> u16 { a*(a+1)/2 + b }

fn repeat<E>>(elem: E, count: isize) {
    let mut vec: Vec<E> = std::vec::Vec::with_capacity(count);
    for i in 0..isize {
        vec.push(elem);
    }
    vec.shrink_to_fit();
    vec
}

fn triangle(n: u16) {
    let length = index(n+1, 0);
    let mut char_tri = repeat(1, length)
    let mut char_tri_solved = repeat(false, length)
    let mut cohen_tris = repeat()
}
