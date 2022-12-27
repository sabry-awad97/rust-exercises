use rust_ex::split_string_on_uppercase_chars;

fn main() {
    // Test the function with a sample string
    let string = "HomeSkin CareSkin Care PacksGOOPGENES All-in-One Nourishing Skincare Kit";
    let words = split_string_on_uppercase_chars(&string);
    println!("{:?}", words);
}
