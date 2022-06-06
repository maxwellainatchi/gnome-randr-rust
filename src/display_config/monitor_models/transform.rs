use std::{fmt::{self}};

use bitflags::bitflags;

bitflags! {
    pub struct Orientation: u32 {
        const NORMAL = 0b000;
        const R90 = 0b001;
        const R180 = 0b010;
        const R270 = Self::R90.bits | Self::R180.bits;
    
        const FLIPPED = 0b100;
        const F90 = Self::R90.bits | Self::FLIPPED.bits;
        const F180 = Self::R180.bits | Self::FLIPPED.bits;
        const F270 = Self::R270.bits | Self::FLIPPED.bits;
    }
}



impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = if self.contains(Orientation::R270) {
            "Left"
        } else if self.contains(Orientation::R180) {
            "Inverted"
        } else if self.contains(Orientation::R90) {
            "Right"
        } else {
            "Normal"
        };

        write!(
            f,
            "{}{}",
            if self.contains(Orientation::FLIPPED) {
                "Flipped "
            } else {
                ""
            },
            display
        )
    }
}

impl std::str::FromStr for Orientation {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return s.to_lowercase().as_str().split(',').try_fold(
            Orientation::NORMAL, 
            |acc, new_cmd| {
                match new_cmd.to_lowercase().as_str() {
                    "normal" => Ok(Orientation::NORMAL | acc),
                    "left" => Ok(Orientation::R270 | acc),
                    "right" => Ok(Orientation::R90 | acc),
                    "inverted" => Ok(Orientation::R180 | acc),
                    "flipped" => Ok(Orientation::FLIPPED | acc),
                    _ => Err(std::fmt::Error)
                }
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Displacement {
    // x position
    pub x: i32,
    // y position
    pub y: i32,
    // scale
    pub scale: f64
}

impl fmt::Display for Displacement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "x: {}, y: {}, scale: {}",
            self.x, self.y, self.scale
        )
    }
}

impl std::str::FromStr for Displacement {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower_s = s.to_lowercase();
        let vals: Vec<&str> = lower_s.as_str().split(',').collect();
        if vals.len() != 3 {
            return Err(std::fmt::Error);
        } else {
            if let Ok(x) = vals[0].parse::<i32>() {
                if let Ok(y) = vals[1].parse::<i32>() {
                    if let Ok(s) = vals[2].parse::<f64>(){
                        Ok(Displacement{x: x, y: y, scale: s})
                    } else {
                        return Err(std::fmt::Error);
                    }
                } else {
                    return Err(std::fmt::Error);
                }
            } else {
                return Err(std::fmt::Error);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Transform {
    // The displacement (location and scale) of this Transform
    pub displacement: Displacement,

    // The orientation (Rotation and flip) of this Transform
    pub orientation: Orientation
}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, {}",
            self.displacement, self.orientation
        )
    }
}

impl Transform {
    pub fn from(
        x: i32,
        y: i32,
        scale: f64,
        spin: u32
    ) -> Transform { 
            Transform {
            displacement: Displacement {
                x: x,
                y: y,
                scale: scale
            },
            orientation: Orientation::from_bits_truncate(spin) 
        }
    }
}