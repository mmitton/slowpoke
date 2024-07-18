use slowpoke::*;

fn main() {
    TurtleArgs::default()
        .with_size(400, 400)
        .with_title("a triangle circle")
        .run(|turtle| {
            turtle.speed(1);
            turtle.teleport(0, 180);
            turtle.steps(3).circle(180);
        });
}
