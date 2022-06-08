fn main() {
    if is_laptop::check() {
        println!("The device is a Laptop")
    } else {
        println!("The device is not a Laptop")
    }
}
