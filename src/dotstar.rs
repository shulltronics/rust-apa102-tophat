use smart_leds::{RGB8};

pub const NUM_PX: usize = 39;

pub struct DotStar_Pulse {
    strip: [RGB8; NUM_PX],
    color: RGB8,
    px_counter: u8,
    descending: bool,
}

impl DotStar_Pulse {

    pub fn new(color: RGB8) -> DotStar_Pulse {
        Self {
            strip: [RGB8::new(0,0,0); NUM_PX],
            color: color,
            px_counter: 0, 
            descending: false,
        }
    }

    pub fn clear(&mut self) {
        for px in self.strip.iter_mut() {
            *px = RGB8::new(0,0,0)
        }
    }

    pub fn set(&mut self, color: RGB8) {
        for px in self.strip.iter_mut() {
            *px = color;
        }
    }    

    pub fn to_list(&self) -> [RGB8; NUM_PX] {
        self.strip
    }

    pub fn iter() {

    }

    pub fn iter_mut() {
    }

}

impl Iterator for DotStar_Pulse {
    type Item = [RGB8; NUM_PX];
    fn next(&mut self) -> Option<Self::Item> {
        /*
        if self.px_counter == (NUM_PX - 1) as u8 {
            self.px_counter = 0;
        } else {
            self.px_counter = self.px_counter + 1;
        }
        self.clear();
        self.strip[px_counter as usize] = RGB8::new(200,0,200);
        */
        if self.px_counter <= 10 {
            self.descending = false;
        } else if self.px_counter >= 200 {
            self.descending = true;
        }
        if self.descending == true {
            self.px_counter = self.px_counter - 1;
        } else {
            self.px_counter = self.px_counter + 1;
        }

        self.set(RGB8::new(self.px_counter, 0, self.px_counter + 10u8 )); 

        Some(self.to_list())
    }
}

pub struct DotStar_Static {
    strip: [RGB8; NUM_PX],
    color: RGB8,
}

impl DotStar_Static {
    pub fn new(_color: RGB8) -> DotStar_Static {
        Self {
            strip: [_color; NUM_PX],
            color: _color,
        }
    }

    pub fn to_list(&self) -> [RGB8; NUM_PX] {
        self.strip
    }
}

pub struct DotStar_Wheel {
    strip: [RGB8; NUM_PX],
    wheel_pos: u8,
    delta: u8,
    divisor: u8,
}

impl DotStar_Wheel {
    pub fn new() -> DotStar_Wheel {
        Self {
            strip: [RGB8::new(0,0,0); NUM_PX],
            wheel_pos: 0,
            delta: 1,
            divisor: 10,
        }
    }

    pub fn set(&mut self, color: RGB8) {
        for px in self.strip.iter_mut() {
            *px = color;
        }
    }   

    pub fn to_list(&self) -> [RGB8; NUM_PX] {
        self.strip
    }

}

impl Iterator for DotStar_Wheel {
    type Item = [RGB8; NUM_PX];
    fn next(&mut self) -> Option<Self::Item> {
        self.wheel_pos = self.wheel_pos.wrapping_add(self.delta);
        self.set(wheel(self.wheel_pos) / self.divisor); 

        Some(self.to_list())
    }

}

pub struct DotStar_Beacon {
    strip: [RGB8; NUM_PX],
    color: RGB8,
    px_pos: u8,
    delta: u8,
}

impl DotStar_Beacon {
    pub fn new(_color: RGB8) -> DotStar_Beacon {
        Self {
            strip: [_color; NUM_PX],
            color: _color,
            px_pos: 1,
            delta: 1,
        }
    }

    pub fn to_list(&self) -> [RGB8; NUM_PX] {
        self.strip
    }
}

impl Iterator for DotStar_Beacon {
    type Item = [RGB8; NUM_PX];
    fn next(&mut self) -> Option<Self::Item> {
        if self.px_pos == (NUM_PX-1) as u8 {
            self.px_pos = 0;
        } else {
            self.px_pos = self.px_pos + self.delta;
        }

        for (i, px) in self.strip.iter_mut().enumerate() {
            if i as u8 == self.px_pos {
                *px = self.color;
            } else {
                *px = RGB8::new(0,0,0);
            }
        }       
        
        Some(self.to_list())
    }

}

// Returns a color based on a number 0-255
fn wheel(mut wheel_pos: u8) -> RGB8 {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        // No green in this sector - red and blue only
        (255 - (wheel_pos * 3), 0, wheel_pos * 3).into()
    } else if wheel_pos < 170 {
        // No red in this sector - green and blue only
        wheel_pos -= 85;
        (0, wheel_pos * 3, 255 - (wheel_pos * 3)).into()
    } else {
        // No blue in this sector - red and green only
        wheel_pos -= 170;
        (wheel_pos * 3, 255 - (wheel_pos * 3), 0).into()
    }
}
