use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    // Clear the background to blue.
    draw.background().color(DARKBLUE);

    // Get window size and time globals
    let win = app.window_rect();
    let t = app.time;

    // loop through y co-ordinates at increments of 30
    let mut j : f32 = win.h() * -0.5 + 10.0;
    while j < win.h() * 0.5 {
        
        // loop through the x co-ordinates at increments of 30
        let mut i : f32 = win.w() * -0.5 + 15.0;
        while i < win.w() * 0.5 {

            //Draw a circle at each co-ordinate
            draw.ellipse()
            .x_y(i, j).radius(t)
            .stroke_weight(1.5).stroke(RED)
            .rgba(1.0, 0.0, 1.0, 0.01);

            i = i + 30.0;
        }

        j = j + 30.0;
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}