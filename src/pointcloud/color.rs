use nalgebra::Point3;

/// Structure representing a collection of colors (in RGB) in the collection the points.
pub struct Color {
    data: Vec<PointColor>,
}

impl Color {
    /// Creating a new collection of color with specific data
    pub fn new(data: Vec<PointColor>) -> Self {
        Color { data }
    }

    /// Get a data under the borrow type
    pub fn get_borrow_data(&self) -> &Vec<PointColor> {
        &self.data //self.data.into_iter().map(|coord| coord.get_point3()).collect()
    }
}

/// Structure representing the colors (in RGB) of one point.
#[derive(Debug, Clone)]
pub struct PointColor {
    /// red
    pub red: u8,
    /// green
    pub green: u8,
    /// blue
    pub blue: u8,
}

impl PartialEq for PointColor {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}

impl PointColor {
    /// Return a white `PointColor` (R = G = B = 0)
    pub fn new_default() -> Self {
        PointColor {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    /// Return a `PointColor` with specific RGB
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        PointColor { red, green, blue }
    }

    /// Return the `Point3` type of the `PointColor` for rendering
    pub fn get_point3(&self) -> Point3<f32> {
        Point3::new(
            self.red as f32 / 256.0,
            self.green as f32 / 256.0,
            self.blue as f32 / 256.0,
        )
    }

    /// Return a average `PointColor` of two `PointColor`s
    pub fn get_average(
        &self,
        another_point: &PointColor,
        prev_weight: f32,
        next_weight: f32,
    ) -> PointColor {
        // let lab_of_self = self.to_lab();
        // let lab_of_another = another_point.to_lab();
        // let lab_of_average = Lab {
        //     l: (lab_of_self.l + lab_of_another.l) / 2.0,
        //     a: (lab_of_self.a + lab_of_another.a) / 2.0,
        //     b: (lab_of_self.b + lab_of_another.b) / 2.0,
        // };

        // PointColor::new_with_array(lab_of_average.to_rgb())

        // PointColor::new(
        //     ((self.red as usize + another_point.red as usize) / 2) as u8,
        //     ((self.green as usize + another_point.green as usize) / 2) as u8,
        //     ((self.blue as usize + another_point.blue as usize) / 2) as u8,
        // )

        PointColor::new(
            ((self.red as f32 * prev_weight) + (another_point.red as f32 * next_weight)) as u8,
            ((self.green as f32 * prev_weight) + (another_point.green as f32 * next_weight)) as u8,
            ((self.blue as f32 * prev_weight) + (another_point.blue as f32 * next_weight)) as u8,
        )
    }

    /// Return the difference between two `PointColor`s
    pub fn get_color_delta(&self, another_point: &PointColor) -> f32 {
        // let lab_of_self = self.to_lab();
        // let lab_of_another = another_point.to_lab();

        // (lab_of_self.l - lab_of_another.l)
        //     .hypot(lab_of_self.a - lab_of_another.a)
        //     .hypot(lab_of_self.b - lab_of_another.b)
        (self.red as f32 - another_point.red as f32)
            .hypot(self.green as f32 - another_point.green as f32)
            .hypot(self.blue as f32 - another_point.blue as f32)
    }
}
