use slowpoke::*;

fn main() {
    TurtleArgs::default()
        .with_size(400, 400)
        .with_title("a line")
        .run(|turtle| {
            let num = turtle.numinput("This is a request...", "Gimmie a floating point number");
            turtle.forward(100);
            println!("your number was {num}");
        });
}
