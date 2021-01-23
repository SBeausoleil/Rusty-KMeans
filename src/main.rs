mod kmeans_clustering;
use rgb::RGB;

impl kmeans_clustering::EucledeanDistance for RGB<u8> {
    fn eucledean_distance(&self, other: &Self) -> f32 {
        let mut mixer: u16 = (other.r - self.r) as u16;
        mixer += (other.g - self.g) as u16;
        mixer += (other.b - self.b) as u16;
        (mixer as f32).sqrt()
    }
}

impl kmeans_clustering::Centroid for RGB<u8> {
    fn average(data: &Vec<&Self>) -> Self {
        let mut r = 0u32;
        let mut g = 0u32;
        let mut b = 0u32;

        for rgb in data {
            r += rgb.r as u32;
            g += rgb.g as u32;
            b += rgb.b as u32;
        }

        RGB {
            r: (r / data.len() as u32) as u8,
            g: (g / data.len() as u32) as u8,
            b: (b / data.len() as u32) as u8,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
