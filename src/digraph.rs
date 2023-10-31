use super::MAP_SIZE;
pub fn linear(bytes: &[u8], pixels: &mut [u8]) {
    let mut map = [0i32; MAP_SIZE * MAP_SIZE];

    for i in 0..bytes.len() - 1 {
        let x: u8 = bytes[i];
        let y: u8 = bytes[i + 1];
        map[(x as usize) * MAP_SIZE + y as usize] += 1;
    }
    let mut max = 0;
    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            let value = map[y * MAP_SIZE + x];
            if value > max {
                max = value;
            }
        }
    }

    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            let value = (map[y * MAP_SIZE + x] * 255 / max) as u8;
            let index: usize = y * MAP_SIZE + x;
            pixels[index] = value;
        }
    }
}

pub fn log(bytes: &[u8], pixels: &mut [u8]) {
    let mut map = [0i32; MAP_SIZE * MAP_SIZE];

    for i in 0..bytes.len() - 1 {
        let x: u8 = bytes[i];
        let y: u8 = bytes[i + 1];
        map[(x as usize) * MAP_SIZE + y as usize] += 1;
    }
    let mut max = 0;
    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            let value = map[y * MAP_SIZE + x];
            if value > max && value != 0 {
                max = value.ilog2() as i32
            }
        }
    }

    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            let value = map[y * MAP_SIZE + x];
            if value == 0 {
                continue;
            }
            let value = (value.ilog2() * 255 / max as u32) as u8;
            let index: usize = y * MAP_SIZE + x;
            pixels[index] = value;
        }
    }
}
