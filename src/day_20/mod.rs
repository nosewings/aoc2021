use combine::Parser;
use itertools::Itertools;
use ndarray::Array2;

pub struct Image {
    pub pixels: Array2<bool>,
    pub fill: Array2<bool>,
    offset: isize,
}

impl Image {
    pub fn from_pixels(pixels: Array2<bool>) -> Self {
        let fill = Array2::from_elem([3, 3], false);
        let offset = 0;
        Self {
            pixels,
            fill,
            offset,
        }
    }

    #[must_use]
    pub fn enhance(&self, key: &[bool]) -> Self {
        let (h, w) = self.pixels.dim();
        let mut pixels = Array2::from_elem([h + 4, w + 4], false);
        for (i, j) in (0..h + 4).cartesian_product(0..w + 4) {
            Image::enhance_from(
                |i, j| self.get_pixel(i - 2, j - 2),
                &mut pixels,
                key,
                i as isize,
                j as isize,
            );
        }
        let mut fill = Array2::from_elem(self.fill.dim(), false);
        for (i, j) in (0..3).cartesian_product(0..3) {
            Image::enhance_from(
                |i, j| self.get_fill_pixel(i, j),
                &mut fill,
                key,
                i as isize,
                j as isize,
            );
        }
        let offset = (self.offset - 2) % 3;
        Image {
            pixels,
            fill,
            offset,
        }
    }

    fn get_fill_pixel(&self, i: isize, j: isize) -> bool {
        let i = (i.rem_euclid(3)) as usize;
        let j = (j.rem_euclid(3)) as usize;
        self.fill[(i, j)]
    }

    fn get_pixel(&self, i: isize, j: isize) -> bool {
        let (h, w) = self.pixels.dim();
        let h = h as isize;
        let w = w as isize;
        if i < 0 || i >= h || j < 0 || j >= w {
            self.get_fill_pixel(i + self.offset, j + self.offset)
        } else {
            self.pixels[(i as usize, j as usize)]
        }
    }

    fn enhance_from<F>(src: F, dst: &mut Array2<bool>, key: &[bool], i: isize, j: isize)
    where
        F: Fn(isize, isize) -> bool,
    {
        let n = [i + 1, i, i - 1]
            .into_iter()
            .cartesian_product([j + 1, j, j - 1].into_iter())
            .enumerate()
            .fold(0, |n, (k, (i, j))| n | ((src(i, j) as usize) << k));
        let i = i as usize;
        let j = j as usize;
        dst[(i, j)] = key[n];
    }

    fn array_to_string(a: &Array2<bool>) -> String {
        let iter = a.rows().into_iter().map(|row| {
            row.iter()
                .copied()
                .map(|b| if b { '#' } else { '.' })
                .collect()
        });
        Itertools::intersperse(iter, "\n".to_string()).collect()
    }

    pub fn pixels_to_string(&self) -> String {
        Image::array_to_string(&self.pixels)
    }

    pub fn fill_to_string(&self) -> String {
        Image::array_to_string(&self.fill)
    }
}

pub struct Input {
    pub key: Vec<bool>,
    pub image: Image,
}

impl Input {
    pub fn enhance(&mut self) {
        self.image = self.image.enhance(&self.key);
    }
}

pub fn parse_input<'a>() -> impl Parser<&'a str, Output = Input> {
    use combine::parser::char::*;
    use combine::*;

    fn bool<'a>() -> impl Parser<&'a str, Output = bool> {
        choice!(
            char('.').map(|_| false),
            char('#').map(|_| true),
            unexpected_any("non-boolean char")
        )
    }

    fn key<'a>() -> impl Parser<&'a str, Output = Vec<bool>> {
        many1(bool()).skip(newline())
    }

    fn image<'a>() -> impl Parser<&'a str, Output = Image> {
        sep_end_by1(many1(bool()), newline()).map(|lines: Vec<Vec<bool>>| {
            let shape = (lines.len(), lines[0].len());
            let pixels =
                Array2::from_shape_vec(shape, lines.iter().flatten().copied().collect()).unwrap();
            Image::from_pixels(pixels)
        })
    }

    key()
        .skip(newline())
        .and(image())
        .skip(eof())
        .map(|(key, image)| Input { key, image })
}
