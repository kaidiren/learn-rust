mod my;
fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();
    function();
    my::indirect_function();
    my::nested::function();
}
