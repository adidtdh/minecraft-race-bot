use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug)]
pub struct Checkpoint{
    x: f32,
    y: f32,
    z: f32
}

impl Checkpoint {
    pub fn new(x:f32, y:f32, z:f32) -> Checkpoint {
        Checkpoint { x, y, z }
    }

    pub fn distance_to(&self, p: &Checkpoint) -> f32 {
        ((self.x - p.x).powi(2) + (self.y - p.y).powi(2) + (self.z - p.z).powi(2)).sqrt()
    }

    pub fn plane_distance_to(self, p: &Checkpoint) -> f32 {
        ((self.x - p.x).powi(2) + (self.y - p.y).powi(2)).sqrt()
    }

    pub fn from_bytes(buf: &[u8]) -> Checkpoint {


        let mut floats: [f32; 4] = [0.0;4];

        for (i,chunk) in buf.chunks(4).enumerate(){
            floats[i] = f32::from_be_bytes(chunk.try_into().unwrap());
        }


        return Checkpoint { x: floats[0], y: floats[1], z: floats[2] };
        
    }

    pub fn to_str(self) -> String{
        format!("x:{}, y:{}, z:{}", self.x, self.y, self.z)
    }
    
}

