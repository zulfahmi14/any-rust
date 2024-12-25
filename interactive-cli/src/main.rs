use dialoguer::Input;

fn main() {
    loop {
        let input: String = Input::new()
            .with_prompt("Enter your name")
            .interact()
            .unwrap();

        println!("Hello, {}", input);
    }
}
