#![no_main]
#![no_std]

use core::ops::{Add, AddAssign};
use core::str::FromStr;
use fixed::types::I16F16;
use mcinterface::*;

type FIXED = I16F16;

#[derive(Copy, Clone)]
struct Complex {
    re: FIXED,
    im: FIXED,
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl Complex {
    fn zero() -> Self {
        Self {
            re: FIXED::ZERO,
            im: FIXED::ZERO,
        }
    }

    fn abs_s(&self) -> FIXED {
        self.re * self.re + self.im * self.im
    }

    fn square(self) -> Self {
        Self {
            re: self.re * self.re - self.im * self.im,
            im: FIXED::from_num(2) * self.re * self.im,
        }
    }
}

fn block_from_iters(iters: i32) -> Block {
    match iters {
        0 => Block::Air,
        1 => Block::Andesite,
        2 => Block::Cobblestone,
        3 => Block::Diamond,
        4 => Block::Diorite,
        5 => Block::Dirt,
        6 => Block::Emerald,
        7 => Block::Gold,
        8 => Block::Granite,
        9 => Block::Iron,
        10 => Block::Lapis,
        11 => Block::OakLog,
        12 => Block::Redstone,
        _ => Block::Air,
    }
}

macro_rules! env_i32_default {
    ($var:literal,$default:literal) => {
        i32::from_str(option_env!($var).unwrap_or($default)).unwrap()
    }
}

#[no_mangle]
pub extern fn _start() -> i32 {
    let iters  = env_i32_default!("MAX_ITERATIONS", "12");
    let width = env_i32_default!("WIDTH", "80");

    let init_x = env_i32_default!("POS_X", "-50");
    let init_y = env_i32_default!("POS_Y", "128");
    let init_z = env_i32_default!("POS_Z", "-50");

    let init_c = Complex {
        re: FIXED::from_num(-2),
        im: FIXED::from_num(2),
    };

    let mut c = init_c;

    let inc = FIXED::ONE / FIXED::from_num(width / 4);

    let mut x = init_x;
    let mut y = init_y;
    let     z = init_z;
    turtle_pos(x, y, z);

    for _ in 0..width {
        for _ in 0..width {
            x += 1;
            turtle_x(x);

            let mut z = Complex::zero();
            let mut i = 0;
            for _ in 0..iters {

                z = z.square() + c;

                if z.abs_s() > FIXED::from_num(4) {
                    break;
                }

                i += 1;
            }

            turtle_set(block_from_iters(i));

            c += Complex { re: inc, im: FIXED::ZERO };
        }
        x = init_x;
        y -= 1;
        turtle_x(x);
        turtle_y(y);
        c.re = init_c.re;
        c += Complex { re: FIXED::ZERO, im: -inc };
    }

    return 0;
}