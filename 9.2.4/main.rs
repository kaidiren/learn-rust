fn call_me<F: Fn()>(f: F) {
    f()
}

fn function() {
    println!("i am  a fucntion");
}

fn main() {
    let c = || println!("i am a c");
    call_me(c);
    call_me(function);
}
