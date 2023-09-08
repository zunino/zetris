use rand::prelude::*;

pub const PLAYFIELD_HCELLS: usize = 10;
pub const PLAYFIELD_VCELLS: usize = 20;
pub const VANISH_CELLS: usize = 4;
pub const START_Y: i8 = -(VANISH_CELLS as i8 + 1);

#[derive(Clone, Copy, Debug)]
pub enum MinoColor {
    Grey = 0,
    Cyan,
    Yellow,
    Purple,
    Green,
    Red,
    Blue,
    Orange,
}

impl Default for MinoColor {
    fn default() -> Self {
        MinoColor::Grey
    }
}

pub enum Kind {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl From<u32> for Kind {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::I,
            1 => Self::O,
            2 => Self::T,
            3 => Self::S,
            4 => Self::Z,
            5 => Self::J,
            6 => Self::L,
            _ => Self::O
        }
    }
}

#[derive(Clone, Debug)]
pub struct Coord {
    pub x: i8,
    pub y: i8,
}

#[derive(Clone, Debug)]
pub struct Zetromino {
    pub minos: [Coord; 4],
    pub color: MinoColor,
}

impl Zetromino {
    pub fn new(kind: Kind) -> Self {
        match kind {
            Kind::I => {
                let x_0: i8 = PLAYFIELD_HCELLS as i8 / 2 - 2;
                let y_0: i8 = -1;
                Self {
                    minos: [
                        Coord { x: x_0, y: y_0 },
                        Coord { x: x_0 + 1, y: y_0 },
                        Coord { x: x_0 + 2, y: y_0 },
                        Coord { x: x_0 + 3, y: y_0 },
                    ],
                    color: MinoColor::Cyan,
                }
            }
            Kind::O => {
                let x_0: i8 = PLAYFIELD_HCELLS as i8 / 2 - 1;
                let y_0: i8 = -2;
                Self {
                    minos: [
                        Coord { x: x_0, y: y_0 },
                        Coord { x: x_0 + 1, y: y_0 },
                        Coord { x: x_0, y: y_0 + 1 },
                        Coord {
                            x: x_0 + 1,
                            y: y_0 + 1,
                        },
                    ],
                    color: MinoColor::Yellow,
                }
            }
            Kind::T => {
                let x_0: i8 = PLAYFIELD_HCELLS as i8 / 2 - 1;
                let y_0: i8 = -2;
                Self {
                    minos: [
                        Coord { x: x_0 + 1, y: y_0 },
                        Coord { x: x_0, y: y_0 + 1 },
                        Coord {
                            x: x_0 + 1,
                            y: y_0 + 1,
                        },
                        Coord {
                            x: x_0 + 2,
                            y: y_0 + 1,
                        },
                    ],
                    color: MinoColor::Purple,
                }
            }
            Kind::S => {
                let x_0: i8 = PLAYFIELD_HCELLS as i8 / 2 - 1;
                let y_0: i8 = -2;
                Self {
                    minos: [
                        Coord { x: x_0 + 1, y: y_0 },
                        Coord { x: x_0 + 2, y: y_0 },
                        Coord {
                            x: x_0 + 1,
                            y: y_0 + 1,
                        },
                        Coord { x: x_0, y: y_0 + 1 },
                    ],
                    color: MinoColor::Green,
                }
            }
            Kind::Z => {
                let x_0: i8 = PLAYFIELD_HCELLS as i8 / 2 - 1;
                let y_0: i8 = -2;
                Self {
                    minos: [
                        Coord { x: x_0, y: y_0 },
                        Coord { x: x_0 + 1, y: y_0 },
                        Coord {
                            x: x_0 + 1,
                            y: y_0 + 1,
                        },
                        Coord {
                            x: x_0 + 2,
                            y: y_0 + 1,
                        },
                    ],
                    color: MinoColor::Red,
                }
            }
            Kind::J => {
                let x_0: i8 = PLAYFIELD_HCELLS as i8 / 2 - 1;
                let y_0: i8 = -2;
                Self {
                    minos: [
                        Coord { x: x_0, y: y_0 },
                        Coord { x: x_0, y: y_0 + 1 },
                        Coord {
                            x: x_0 + 1,
                            y: y_0 + 1,
                        },
                        Coord {
                            x: x_0 + 2,
                            y: y_0 + 1,
                        },
                    ],
                    color: MinoColor::Blue,
                }
            }
            Kind::L => {
                let x_0: i8 = PLAYFIELD_HCELLS as i8 / 2 - 1;
                let y_0: i8 = -2;
                Self {
                    minos: [
                        Coord { x: x_0 + 2, y: y_0 },
                        Coord { x: x_0, y: y_0 + 1 },
                        Coord {
                            x: x_0 + 1,
                            y: y_0 + 1,
                        },
                        Coord {
                            x: x_0 + 2,
                            y: y_0 + 1,
                        },
                    ],
                    color: MinoColor::Orange,
                }
            }
        }
    }

    pub fn random_zetromino() -> Self {
        let mut rng = rand::thread_rng();
        let kind = Kind::from(rng.gen_range(0..7));
        Self::new(kind)
    }

    pub fn down(&mut self) {
        for mino in self.minos.iter_mut() {
            mino.y += 1;
        }
    }

    pub fn left(&mut self) {
        for mino in self.minos.iter_mut() {
            mino.x -= 1;
        }
    }

    pub fn right(&mut self) {
        for mino in self.minos.iter_mut() {
            mino.x += 1;
        }
    }

    pub fn min_y(&self) -> i8 {
        self.minos.iter().map(|m| m.y).min().unwrap()
    }

    pub fn max_y(&self) -> i8 {
        self.minos.iter().map(|m| m.y).max().unwrap()
    }

    pub fn min_x(&self) -> i8 {
        self.minos.iter().map(|m| m.x).min().unwrap()
    }

    pub fn max_x(&self) -> i8 {
        self.minos.iter().map(|m| m.x).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_y_of_new_o_zetromino_should_be_minus_1() {
        let mut o = Zetromino::new(Kind::O);
        assert_eq!(o.max_y(), -1);
    }

    #[test]
    fn test_max_y_shold_be_2_after_3_downward_moves() {
        let mut o = Zetromino::new(Kind::O);
        o.down();
        o.down();
        o.down();
        assert_eq!(o.max_y(), 2);
    }
}
