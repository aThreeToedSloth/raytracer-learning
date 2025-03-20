pub const NUM_OF_CIRCLES: usize = 5;

pub struct Circle {
    pub centre_x: f32,
    pub centre_y: f32,
    pub centre_z: f32,
    pub radius: f32,
    pub character: char
}

impl Circle {
    pub fn clone(&self) -> Circle {
        Circle {
            centre_x: self.centre_x,
            centre_y: self.centre_y,
            centre_z: self.centre_z,
            radius: self.radius,
            character: self.character
        }
    }
}

pub fn create_circles() -> [Circle; NUM_OF_CIRCLES]{
    [
        Circle{
            centre_x: 2.0,
            centre_y: 1.0,
            centre_z: 4.0,
            radius: 1.0,
            character: '@'
        },
        Circle{
            centre_x: -3.0,
            centre_y: -3.0,
            centre_z: 30.0,
            radius: 1.0,
            character: '£'
        },
        Circle{
            centre_x: 1.0,
            centre_y: 0.0,
            centre_z: 10.0,
            radius: 1.0,
            character: '*'
        },
        Circle{
            centre_x: -2.0,
            centre_y: -0.5,
            centre_z: 15.0,
            radius: 1.0,
            character: '#'
        },
        Circle{
            centre_x: 1.0,
            centre_y: -3.0,
            centre_z: 20.0,
            radius: 1.0,
            character: '?'
        }
    ]
}