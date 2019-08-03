mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my::function()`");
}

fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_function() {
    print!("called `my::indirect_function()`, that\n ");
    private_function();
}
