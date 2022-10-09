use turtle::Turtle;

fn reset_turtle(turtle: &mut Turtle) {
    turtle.reset();
}
pub fn exercise_1(turtle: &mut Turtle) {
    reset_turtle(turtle);
    turtle.forward(100.0);
    turtle.right(90.0);
    turtle.forward(100.0);
}

pub fn exercise_2(turtle: &mut Turtle) {
    turtle.right(270.0);
    turtle.forward(150.0);
    reset_turtle(turtle)
}

pub fn draw_star(turtle: &mut Turtle) {
    for _ in 0..5 {
        turtle.forward(200.0);
        turtle.right(144.0);
    }
    reset_turtle(turtle)
}

pub fn draw_stick_figure(turtle: &mut Turtle) {
    reset_turtle(turtle);
    turtle.right(45.0);
    turtle.forward(50.0);
    turtle.right(90.0);
    turtle.forward(50.0);
    turtle.left(180.0);
    turtle.forward(50.0);
    turtle.right(45.0);
    turtle.forward(75.0);
    turtle.right(75.0);
    turtle.forward(35.0);
    turtle.backward(35.0);
    turtle.set_heading(90.0);
    turtle.left(75.0);
    turtle.forward(35.0);
    turtle.backward(35.0);
    turtle.set_heading(90.0);
    turtle.forward(20.0);
    turtle.left(90.0);
    for _ in 0..360 {
        turtle.right(1.0);
        turtle.forward(0.5);
    }
}
// pub fn draw_house(turtle: &mut Turtle) {
//     reset_turtle(turtle);

// }
