mod models;

pub fn run() {
    println!("binary tree");
    let tree = models::Node {
        value: 1,
        left: Some(Box::new({
            models::Node {
                value: 5,
                left: None,
                right: None,
            }
        })),
        right: None,
    };

    println!(
        "Has left: {}, value: {}",
        tree.has_left(),
        tree.left_value()
    );
    println!("Has right: {}", tree.has_right());
    println!("Value: {}", tree.value);
}
