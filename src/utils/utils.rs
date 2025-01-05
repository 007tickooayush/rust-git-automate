

pub fn get_generated_name() -> String {
    let mut generator = names::Generator::default();
    let generated_name = generator.next().unwrap();
    generated_name
}