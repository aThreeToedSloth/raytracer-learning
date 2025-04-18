mod circle;

const CANVAS_WIDTH: i32 = 100;
const CANVAS_HEIGHT: i32 = 100;

const VIEWPORT_WIDTH: f32 = 1.0;
const VIEWPORT_HEIGHT: f32 = 1.0;
const VIEWPORT_DISTANCE: f32 = 1.0;

const MAX_DISTANCE: f32 = 99999.0;

const NUM_OF_CIRCLES: usize = circle::NUM_OF_CIRCLES;

//loops through each pixel of the canvas
fn main() {
    let mut x:f32;
    let mut y:f32;
    let mut z:f32;

    //Creates circles
    let circles = circle::create_circles();

    for i in ((-CANVAS_HEIGHT/2 + 1)..(CANVAS_HEIGHT/2)).rev() {
        for j in (-CANVAS_WIDTH/2 + 1)..(CANVAS_WIDTH/2) {
            (x,y,z) = canvas_to_viewport(j,i);
            print!("{}", ray(x,y,z, &circles))
        }
        println!("");
    }
}

//Converts the pixels on the canvas to a 3D space on the viewport
fn canvas_to_viewport(x:i32, y:i32) -> (f32,f32,f32){
    (x as f32 * VIEWPORT_WIDTH/CANVAS_WIDTH as f32, y as f32 * VIEWPORT_HEIGHT/CANVAS_HEIGHT as f32, VIEWPORT_DISTANCE)
}

//Creates a ray, checks if it intercepts any spheres
fn ray(x:f32, y:f32, z:f32, circles: &[circle::Circle; NUM_OF_CIRCLES]) -> char {

    let mut closest_t:f32 = MAX_DISTANCE;
    let mut closest_circle: Option<circle::Circle> = None;
    
    let mut t1:f32;
    let mut t2:f32;

    let D = (x, y, z);
    let mut CO: (f32, f32, f32);
    let a = dot_product(D.0, D.1, D.2, D.0, D.1, D.2);
    let mut b: f32;
    let mut c: f32;

    for circle in circles {
        CO = (circle.centre_x *-1.0, circle.centre_y *-1.0, circle.centre_z *-1.0);

        b = 2.0 * dot_product(CO.0, CO.1, CO.2, D.0, D.1, D.2);
        c = dot_product(CO.0, CO.1, CO.2, CO.0, CO.1, CO.2) - circle.radius * circle.radius;

        (t1, t2) = quadratic_equation(a,b,c);
        if closest_t > t1 && t1 >= 1.0 {
            closest_t = t1;
            closest_circle = Some(circle.clone());
        }
        if closest_t > t2 && t2 >= 1.0 {
            closest_t = t2;
            closest_circle = Some(circle.clone());
        }
    }
    
    return match closest_circle {
        Some(x) => x.character,
        None => ' '
    }
}

//Solves the quadratic equation, given a b and c
fn quadratic_equation(a:f32, b:f32, c:f32) -> (f32, f32){
    //If there are no real answers then return the maximum distance
    if(b*b - 4.0*a*c) < 0.0{
        (MAX_DISTANCE, MAX_DISTANCE)
    }
    else {
        (-b + ( b*b - 4.0*a*c).sqrt()/(2.0*a),-b - ( b*b - 4.0*a*c).sqrt()/(2.0*a))
    }
}

//Finds the dot product of two vectors
fn dot_product(x1:f32,y1:f32,z1:f32, x2:f32,y2:f32,z2:f32) -> f32{
    x1*x2+y1*y2+z1*z2
}
