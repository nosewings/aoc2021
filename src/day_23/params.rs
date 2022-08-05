#[derive(Debug)]
pub struct Params {
    pub width: u32,
    pub height: u32,
    pub coeffs: Vec<u32>,
}

impl Params {
    pub fn new(width: u32, height: u32) -> Self {
        Params {
            width,
            height,
            coeffs: (0..width).into_iter().map(|i| 10_u32.pow(i)).collect(),
        }
    }

    pub fn get_coeff(&self, piece: u32) -> u32 {
        self.coeffs[piece as usize]
    }
}
