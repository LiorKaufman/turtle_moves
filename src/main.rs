use turtle::Turtle;
mod exercises;
fn main() {
    let mut turtle = Turtle::new();
    exercises::exercise_1(&mut turtle);
    exercises::exercise_2(&mut turtle);
}
