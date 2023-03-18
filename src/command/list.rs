pub fn list_command(content: Vec<&str>) {
    content.into_iter().for_each(|c| {
        println!("{}", c);
    });
}
