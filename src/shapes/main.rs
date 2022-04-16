use std::fmt;

struct Rectangle {
    width_in_cm: f64,
    height_in_cm: f64,
}

impl Rectangle {
    fn new(width_in_cm: f64, height_in_cm: f64) -> Result<Self, RectangleError> {
        if width_in_cm < 0.0 || height_in_cm < 0.0 {
            return Err(RectangleError::new(
                1,
                String::from("width or height less than 0.0"),
            ));
        }

        Ok(Rectangle {
            width_in_cm,
            height_in_cm,
        })
    }

    fn area(&self) -> f64 {
        self.width_in_cm * self.height_in_cm
    }
}

struct RectangleError {
    code: usize,
    message: String,
}

impl RectangleError {
    fn new(code: usize, message: String) -> Self {
        Self { code, message }
    }
}

impl fmt::Display for RectangleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.code {
            1 => "Can't create rectangle!",
            _ => "Sorry, something is wrong! Please Try Again!",
        };

        write!(f, "{}", err_msg)
    }
}

// A unique format for dubugging output
impl fmt::Debug for RectangleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RectangleError {{ code: {}, message: {} }}",
            self.code, self.message
        )
    }
}

fn main() {
    let rect1 = Rectangle::new(4.0, 6.0).unwrap();
    println!("Area of rectangle: {}", rect1.area());
    let _rect2 = Rectangle::new(-4.0, 6.0).unwrap();
}
