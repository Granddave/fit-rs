use super::MAP_SIZE;

fn rot(n: i32, x: &mut i32, y: &mut i32, rx: i32, ry: i32) {
    if ry == 0 {
        if rx == 1 {
            *x = n - 1 - *x;
            *y = n - 1 - *y;
        }

        std::mem::swap(x, y);
    }
}

fn d2xy(total: i32, distance: i32) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut t = distance;
    let mut s = 1;

    while s < total {
        let rx = 1 & (t / 2);
        let ry = 1 & (t ^ rx);
        rot(s, &mut x, &mut y, rx, ry);

        x += s * rx;
        y += s * ry;
        t /= 4;
        s *= 2;
    }

    (x, y)
}

pub fn linear(bytes: &[u8], pixels: &mut [u8]) {
    for (ix, byte) in bytes.iter().enumerate() {
        let (x, y) = d2xy(MAP_SIZE as i32, ix as i32);
        pixels[y as usize * MAP_SIZE + x as usize] = *byte;
    }
}
