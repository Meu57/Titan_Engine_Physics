use std::ops::{Add, Sub, Mul, Div};

// Calculates (a * b) - (c * d) with reduced cancellation error.
#[inline(always)]
pub fn difference_of_products(a: f32, b: f32, c: f32, d: f32) -> f32 {

    let cd = c * d;
    let err = (-c).mul_add(d, cd); 
    let dop = a.mul_add(b, -cd);   

    dop + err
}

// Robust floating point comparison.
#[inline]
pub fn approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
    if a == b { return true; }
    let diff = (a - b).abs();
    if diff <= epsilon { return true; }
     let abs_a = a.abs();
    let abs_b = b.abs();
    let largest = if abs_a > abs_b { abs_a } else { abs_b };
    diff <= largest * epsilon
}

// Safe Comparison for Sorting (Total Ordering).
// Handles NaNs consistently 
//(unlike partial_cmp which panics on unwrap or returns None).
#[inline]
pub fn safe_cmp(a: f32, b: f32) -> std::cmp::Ordering {
    // total_cmp was stabilized in Rust 1.62
    a.total_cmp(&b)
}

// Smoothly dampens a value towards a target using an exponential decay.
pub fn smooth_damp(current: f32, target: f32, smoothness: f32, dt: f32) -> f32 {
    let t = 1.0 - (-smoothness * dt).exp();
    current + (target - current) * t
}

// A "Not NaN" Float Wrapper.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct F32(f32);

impl F32 {
    pub fn new(val: f32) -> Option<Self> {
        if val.is_nan() { None } else { Some(F32(val)) }
    }

    pub fn new_checked(val: f32) -> Self {
        // Changing !val.is_nan() to val.is_finite()
        // is_finite() returns false for both NaN AND Infinity
        debug_assert!(val.is_finite(), "F32 Error: Expected finite number, got {}", val);
        F32(val)
    } 

    pub fn val(&self) -> f32 { self.0 }
}

impl Add for F32 {
    type Output = F32;
    fn add(self, rhs: F32) -> F32 { F32::new_checked(self.0 + rhs.0) }
}

impl Sub for F32 {
    type Output = F32;
    fn sub(self, rhs: F32) -> F32 { F32::new_checked(self.0 - rhs.0) }
}
impl Mul for F32 {
    type Output = F32;
    fn mul(self, rhs: F32) -> F32 { F32::new_checked(self.0 * rhs.0) }
}

impl Div for F32 {
    type Output = F32;
    fn div(self, rhs: F32) -> F32 { 
        let res = self.0 / rhs.0;
        debug_assert!(!res.is_nan(), "NaN generated in F32 division");
        F32(res) 
    }
}