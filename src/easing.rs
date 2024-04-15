// Acknolegments and License (Credits to the original authors and license terms)
// 
// This code is ported from C++ code, based on the easing functions from the ofxEasing.h of https://github.com/arturoc/ofxEasing
// 
// 1. Original terms of use of ofxEasing.h (https://github.com/arturoc/ofxEasing/blob/master/src/easing_terms_of_use.html)
//  is based on the BSD license (http://www.opensource.org/licenses/bsd-license.php)
// 
//     Copyright © 2001 Robert Penner All rights reserved.
// 
// 2. ofxEasing and ofxTween were published under the MIT License (https://opensource.org/licenses/MIT)
// 
//     ofxEasing:
//         Copyright 2016 (c) Arturo Castro. arturocastro.net
//             ( see https://github.com/arturoc/ofxEasing/blob/master/LICENSE )
//     ofxTween:
//         Copyright 2007 (c) Erik Sjodin, eriksjodin.net
//             ( see https://github.com/arturoc/ofxTween/blob/master/LICENSE )
// 

use std::f32::consts::{FRAC_PI_2, PI};

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EasingFunction {
    #[default]
    Linear = 0,
    Sine = 1,
    Circular = 2,
    Quadratic = 3,
    Cubic = 4,
    Quartic = 5,
    Quintic = 6,
    Exponential = 7,
    Back = 8,
    Bounce = 9,
    Elastic = 10,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EasingType {
    #[default]
    In = 0,
    Out = 1,
    InOut = 2,
}

trait Easing {
    /// Calculate the easing value
    /// t: time
    /// b: beginning value (start value)
    /// c: changing value (= end value - start value)
    /// d: duration
    fn ease(t: f32, b: f32, c: f32, d: f32, easing_type: EasingType) -> f32;
}

pub fn map(v: f32, min_in: f32, max_in: f32, min_out: f32, max_out: f32, easing_function: EasingFunction, easing_type: EasingType) -> f32 {
    let t = v - min_in;
    let c = max_out - min_out;
    let d = max_in - min_in;
    let b = min_out;
    easing(t, b, c, d, easing_function, easing_type)
}

pub fn map_clamp(v: f32, min_in: f32, max_in: f32, min_out: f32, max_out: f32, easing_function: EasingFunction, easing_type: EasingType) -> f32 {
    let v = v.min(max_in).max(min_in);
    map(v, min_in, max_in, min_out, max_out, easing_function, easing_type)
}

pub struct EasingLinear;
impl Easing for EasingLinear {
    fn ease(t: f32, b: f32, c: f32, d: f32, _easing_type: EasingType) -> f32 {
        c * t / d + b
    }
}

pub struct EasingSine;
impl Easing for EasingSine {
    fn ease(t: f32, b: f32, c: f32, d: f32, easing_type: EasingType) -> f32 {
        match easing_type {
            EasingType::In => -c * (t / d * FRAC_PI_2).cos() + c + b,
            EasingType::Out => c * (t / d * FRAC_PI_2).sin() + b,
            EasingType::InOut => -c / 2.0 * ((PI * t / d).cos() - 1.0) + b,
        }
    }
}

pub struct EasingCircular;
impl Easing for EasingCircular {
    // original C++ code:
    // inline static float easeIn (float t,float b , float c, float d) {
	// 	return -c * (sqrt(1 - (t/=d)*t) - 1) + b;
	// }
	// inline static float easeOut(float t,float b , float c, float d) {
	// 	return c * sqrt(1 - (t=t/d-1)*t) + b;
	// }

	// inline static float easeInOut(float t,float b , float c, float d) {
	// 	if ((t/=d/2) < 1) return c/2 * (1 - sqrt(1 - t*t)) + b;
	// 	return c/2 * (sqrt(1 - (t-=2)*t) + 1) + b;
	// }

	// NOTE: (t/=d)*t) is equivalent to (t / d) * (t / d), because of the precedence of the operators

    fn ease(t: f32, b: f32, c: f32, d: f32, easing_type: EasingType) -> f32 {
        match easing_type {
            EasingType::In => {
                let m = t / d;
                -c * ((1.0 - m * m).sqrt() - 1.0) + b
            }
            EasingType::Out => {
                let m = t / d - 1.0;
                c * (1.0 - m * m).sqrt() + b
            }
            EasingType::InOut => {
                let m = t / d * 2.0;
                if m < 1.0 {
                    -c / 2.0 * ((1.0 - m * m).sqrt() - 1.0) + b
                } else {
                    let post_fix = m - 2.0;
                    c / 2.0 * ((1.0 - post_fix * post_fix).sqrt() + 1.0) + b
                }
            }
        }
    }
}

pub struct EasingQuadratic;
impl Easing for EasingQuadratic {
    fn ease(t: f32, b: f32, c: f32, d: f32, easing_type: EasingType) -> f32 {
        match easing_type {
            EasingType::In => c * (t / d).powi(2) + b,
            EasingType::Out => -c * (t / d) * (t / d - 2.0) + b,
            // original C++ code:
            // inline static float easeInOut(float t,float b , float c, float d) {
            //     if ((t/=d/2) < 1) return c/2*t*t + b;
            //     return -c/2 * ((--t)*(t-2) - 1) + b;
            // 
            //     /*
            // 
            //     originally return -c/2 * (((t-2)*(--t)) - 1) + b;
            //
            //     I've had to swap (--t)*(t-2) due to diffence in behaviour in
            //     pre-increment operators between java and c++, after hours
            //     of joy
            //
            //      James George:: The fix refered to above actually broke the equation,
            //      it would land at 50% all the time at the end
            //      copying back the original equation from online fixed it...
            //
            //      potentially compiler dependent.
            // */
            // }
            EasingType::InOut => {
                let m = t / d * 2.0;
                if m < 1.0 {
                    c / 2.0 * m * m + b
                } else {
                    let post_fix = m - 1.0;
                    -c / 2.0 * (post_fix * (post_fix - 2.0) - 1.0) + b
                }
            }
        }
    }
}

pub struct EasingCubic;
impl Easing for EasingCubic {
    fn ease(t: f32, b: f32, c: f32, d: f32, easing_type: EasingType) -> f32 {
        match easing_type {
            EasingType::In => c * (t / d).powi(3) + b,
            EasingType::Out => c * ((t / d - 1.0).powi(3) + 1.0) + b,
            EasingType::InOut => {
                let m = t / d * 2.0;
                if m < 1.0 {
                    c / 2.0 * m.powi(3) + b
                } else {
                    let post_fix = m - 2.0;
                    c / 2.0 * (post_fix.powi(3) + 2.0) + b
                }
            }
        }
    }
}

pub struct EasingQuartic;
impl Easing for EasingQuartic {
    fn ease(t: f32, b: f32, c: f32, d: f32, easing_type: EasingType) -> f32 {
        match easing_type {
            EasingType::In => c * (t / d).powi(4) + b,
            EasingType::Out => -c * ((t / d - 1.0).powi(4) - 1.0) + b,
            EasingType::InOut => {
                let m = t / d * 2.0;
                if m < 1.0 {
                    c / 2.0 * m.powi(4) + b
                } else {
                    let post_fix = m - 2.0;
                    -c / 2.0 * (post_fix.powi(4) - 2.0) + b
                }
            }
        }
    }
}

pub struct EasingQuintic;
impl Easing for EasingQuintic {
    fn ease(t: f32, b: f32, c: f32, d: f32, easing_type: EasingType) -> f32 {
        match easing_type {
            EasingType::In => c * (t / d).powi(5) + b,
            EasingType::Out => c * ((t / d - 1.0).powi(5) + 1.0) + b,
            EasingType::InOut => {
                let m = t / d * 2.0;
                if m < 1.0 {
                    c / 2.0 * m.powi(5) + b
                } else {
                    let post_fix = m - 2.0;
                    c / 2.0 * (post_fix.powi(5) + 2.0) + b
                }
            }
        }
    }
}

pub struct EasingExponential;
impl Easing for EasingExponential {
    fn ease(t: f32, b: f32, c: f32, d: f32, easing_type: EasingType) -> f32 {
        match easing_type {
            EasingType::In => {
                if t == 0.0 {
                    b
                } else {
                    c * 2.0f32.powf(10.0 * (t / d - 1.0)) + b
                }
            }
            EasingType::Out => {
                if t == d {
                    b + c
                } else {
                    c * (-2.0f32.powf(-10.0 * t / d) + 1.0) + b
                }
            }
            EasingType::InOut => {
                let m = t / d * 2.0;
                if t == 0.0 {
                    b
                } else if t == d {
                    b + c
                } else if m < 1.0 {
                    c / 2.0 * 2.0f32.powf(10.0 * (m - 1.0)) + b
                } else {
                    let post_fix = m - 1.0;
                    c / 2.0 * (-2.0f32.powf(-10.0 * post_fix / d) + 2.0) + b
                }
            }
        }
    }
}

pub struct EasingBack;
impl Easing for EasingBack {
    fn ease(t: f32, b: f32, c: f32, d: f32, easing_type: EasingType) -> f32 {
        match easing_type {
            EasingType::In => EasingBack::ease_in_s(t, b, c, d, 1.70158),
            EasingType::Out => EasingBack::ease_out_s(t, b, c, d, 1.70158),
            EasingType::InOut => EasingBack::ease_in_out_s(t, b, c, d, 1.70158),
        }
    }
}

impl EasingBack {
    fn ease_in_s(t: f32, b: f32, c: f32, d: f32, s: f32) -> f32 {
        let post_fix = t / d;
        c * post_fix * post_fix * ((s + 1.0) * post_fix - s) + b
    }

    fn ease_out_s(t: f32, b: f32, c: f32, d: f32, s: f32) -> f32 {
        let post_fix = t / d - 1.0;
        c * (post_fix * post_fix * ((s + 1.0) * post_fix + s) + 1.0) + b
    }

    fn ease_in_out_s(t: f32, b: f32, c: f32, d: f32, s: f32) -> f32 {
        let s = s * 1.525;
        let m = t / d * 2.0;
        if m < 1.0 {
            c / 2.0 * m * m * ((s + 1.0) * m - s) + b
        } else {
            let post_fix = m - 2.0;
            c / 2.0 * (post_fix * post_fix * ((s + 1.0) * post_fix + s) + 2.0) + b
        }
    }
    
}

pub struct EasingBounce;
impl Easing for EasingBounce {
    fn ease(t: f32, b: f32, c: f32, d: f32, easing_type: EasingType) -> f32 {
        match easing_type {
            EasingType::In => c - EasingBounce::ease(d-t, 0.0, c, d, EasingType::Out) + b,
            // original C++ code:

            // inline static float easeOut(float t,float b , float c, float d) {
            //     if ((t/=d) < (1/2.75f)) {
            //         return c*(7.5625f*t*t) + b;
            //     } else if (t < (2/2.75f)) {
            //         float postFix = t-=(1.5f/2.75f);
            //         return c*(7.5625f*(postFix)*t + .75f) + b;
            //     } else if (t < (2.5/2.75)) {
            //             float postFix = t-=(2.25f/2.75f);
            //         return c*(7.5625f*(postFix)*t + .9375f) + b;
            //     } else {
            //         float postFix = t-=(2.625f/2.75f);
            //         return c*(7.5625f*(postFix)*t + .984375f) + b;
            //     }
            // }

            EasingType::Out => {
                let m = t/d;
                if m < 1.0 / 2.75 {
                    c * (7.5625 * m * m) + b
                } else if m < 2.0 / 2.75 {
                    let p = m - 1.5 / 2.75;
                    c * (7.5625 * p * p + 0.75) + b
                } else if m < 2.5 / 2.75 {
                    let p = m - 2.25 / 2.75;
                    c * (7.5625 * p * p + 0.9375) + b
                } else {
                    let p = m - 2.625 / 2.75;
                    c * (7.5625 * p * p + 0.984375) + b
                }
            }
            EasingType::InOut => {
                if t < d / 2.0 {
                    EasingBounce::ease(t * 2.0, 0.0, c, d, EasingType::In) * 0.5 + b 
                } else {
                    EasingBounce::ease(t * 2.0 - d, 0.0, c, d, EasingType::Out) * 0.5 + c * 0.5 + b
                }
            }
        }
    }
}

pub struct EasingElastic;
impl Easing for EasingElastic {
    fn ease(t: f32, b: f32, c: f32, d: f32, easing_type: EasingType) -> f32 {
        match easing_type {
            EasingType::In => EasingElastic::ease_in_pow(t, b, c, d, 10.0),
            EasingType::Out => EasingElastic::ease_out_pow(t, b, c, d, 10.0),
            EasingType::InOut => EasingElastic::ease_in_out_pow(t, b, c, d, 10.0),
        }
    }
}

impl EasingElastic {
    fn ease_in_pow(t: f32, b: f32, c: f32, d: f32, power: f32) -> f32 {
        if t == 0.0 {
            return b;
        }
        if t == d {
            return b + c;
        }
        let p = d * 0.3;
        let a = c;
        let s = p / 4.0;
        let post_fix = a * 2.0f32.powf(power * (t - 1.0));
        -(post_fix * ((t * d - s) * (2.0 * PI) / p).sin()) + b
    }

    fn ease_out_pow(t: f32, b: f32, c: f32, d: f32, power: f32) -> f32 {
        if t == 0.0 {
            return b;
        }
        if t == d {
            return b + c;
        }
        let p = d * 0.3;
        let a = c;
        let s = p / 4.0;
        a * 2.0f32.powf(-power * t) * ((t * d - s) * (2.0 * PI) / p).sin() + c + b
    }

    fn ease_in_out_pow(t: f32, b: f32, c: f32, d: f32, power: f32) -> f32 {
        if t == 0.0 {
            return b;
        }
        if t == d {
            return b + c;
        }
        if t / d / 2.0 == 2.0 {
            return b + c;
        }
        let p = d * 0.3 * 1.5;
        let a = c;
        let s = p / 4.0;
        if t < 1.0 {
            let post_fix = a * 2.0f32.powf(power * (t - 1.0));
            -0.5 * (post_fix * ((t * d - s) * (2.0 * PI) / p).sin()) + b
        } else {
            let post_fix = a * 2.0f32.powf(-power * (t - 1.0));
            post_fix * ((t * d - s) * (2.0 * PI) / p).sin() * 0.5 + c + b
        }
    }
}

/// Calculate the easing value
/// change_value = end_value - start_value, of the output value
/// duration = total time of the easing
pub fn easing(time: f32, beginning_value: f32, changing_value: f32, duration: f32, easing_function: EasingFunction, easing_type: EasingType) -> f32 {
    let t = time;
    let b = beginning_value;
    let c = changing_value;
    let d = duration;
    match easing_function {
        EasingFunction::Linear => EasingLinear::ease(t, b, c, d, easing_type),
        EasingFunction::Sine => EasingSine::ease(t, b, c, d, easing_type),
        EasingFunction::Circular => EasingCircular::ease(t, b, c, d, easing_type),
        EasingFunction::Quadratic => EasingQuadratic::ease(t, b, c, d, easing_type),
        EasingFunction::Cubic => EasingCubic::ease(t, b, c, d, easing_type),
        EasingFunction::Quartic => EasingQuartic::ease(t, b, c, d, easing_type),
        EasingFunction::Quintic => EasingQuintic::ease(t, b, c, d, easing_type),
        EasingFunction::Exponential => EasingExponential::ease(t, b, c, d, easing_type),
        EasingFunction::Back => EasingBack::ease(t, b, c, d, easing_type),
        EasingFunction::Bounce => EasingBounce::ease(t, b, c, d, easing_type),
        EasingFunction::Elastic => EasingElastic::ease(t, b, c, d, easing_type),
    }
}

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_float_relative_eq;
    use assert_float_eq::afe_is_relative_eq;
    use assert_float_eq::afe_relative_error_msg;
    use assert_float_eq::afe_abs;

    use super::*;

    #[test]
    fn map_test() {
        let v = map(0.5, 0.0, 1.0, 0.0, 100.0, EasingFunction::Linear, EasingType::In);
        assert_eq!(v, 50.0);
    }
    
    #[test]
    fn map_clamp_test() {
        let v = map_clamp(2.0, 0.0, 1.0, 0.0, 100.0, EasingFunction::Linear, EasingType::In);
        assert_eq!(v, 100.0);
    }

    #[test]
    fn easing_linear_test() {
        let f = move |t: f32| -> f32 {
            easing(t, 0.0, 100.0, 1.0, EasingFunction::Linear, EasingType::In)
        };
        assert_eq!(f(0.0), 0.0);
        assert_eq!(f(0.5), 50.0);
        assert_eq!(f(1.0), 100.0);
    }

    #[test]
    fn easing_in_should_be_zero_at_start() {
        let easing_func_list = [
            EasingFunction::Linear,
            EasingFunction::Sine,
            EasingFunction::Circular,
            EasingFunction::Quadratic,
            EasingFunction::Cubic,
            EasingFunction::Quartic,
            EasingFunction::Quintic,
            EasingFunction::Exponential,
            EasingFunction::Back,
            EasingFunction::Bounce,
            EasingFunction::Elastic,
        ];
        for easing_func in easing_func_list.iter() {
            let f = move |t: f32| -> f32 {
                easing(t, 0.0, 100.0, 1.0, *easing_func, EasingType::In)
            };
            assert_eq!(f(0.0), 0.0);
        }
    }

    #[test]
    fn easing_out_should_be_one_at_end() {
        let easing_func_list = [
            EasingFunction::Linear,
            EasingFunction::Sine,
            EasingFunction::Circular,
            EasingFunction::Quadratic,
            EasingFunction::Cubic,
            EasingFunction::Quartic,
            EasingFunction::Quintic,
            EasingFunction::Exponential,
            EasingFunction::Back,
            EasingFunction::Bounce,
            EasingFunction::Elastic,
        ];
        for easing_func in easing_func_list.iter() {
            let f = move |t: f32| -> f32 {
                easing(t, 0.0, 100.0, 1.0, *easing_func, EasingType::Out)
            };
            assert_eq!(f(1.0), 100.0);
        }
    }
    
    #[test]
    fn easing_sine_test() {
        let f = move |t: f32| -> f32 {
            easing(t, 0.0, 100.0, 1.0, EasingFunction::Sine, EasingType::In)
        };
        assert_eq!(f(0.0), 0.0);
        assert_eq!(f(0.25), 7.612053);
        assert_eq!(f(0.5), 29.289322);
        assert_eq!(f(0.75), 61.73166);
        assert_eq!(f(1.0), 100.00001);

        let f = move |t: f32| -> f32 {
            easing(t, 0.0, 100.0, 1.0, EasingFunction::Sine, EasingType::Out)
        };

        assert_eq!(f(0.0), 0.0);
        assert_eq!(f(0.25), 38.268345);
        assert_eq!(f(0.5), 70.710686);
        assert_eq!(f(0.75), 92.38795);
        assert_eq!(f(1.0), 100.0);

        let f = move |t: f32| -> f32 {
            easing(t, 0.0, 100.0, 1.0, EasingFunction::Sine, EasingType::InOut)
        };

        assert_eq!(f(0.0), 0.0);
        assert_eq!(f(0.25), 14.644662);
        assert_eq!(f(0.5), 50.0);
        assert_eq!(f(0.75), 85.35534);
        assert_eq!(f(1.0), 100.0);
    }

    #[test]
    fn easing_circular_test() {
        let f = move |t: f32| -> f32 {
            easing(t, 0.0, 100.0, 1.0, EasingFunction::Circular, EasingType::In)
        };
        assert_eq!(f(0.0), 0.0);
        assert_eq!(f(0.25), 3.1754136);
        assert_eq!(f(0.5), 13.397461);
        assert_eq!(f(0.75), 33.85622);
        assert_eq!(f(0.85), 47.32173);
        assert_eq!(f(0.95), 68.77501);
        assert_eq!(f(1.0), 100.0);

        let f = move |t: f32| -> f32 {
            easing(t, 0.0, 100.0, 1.0, EasingFunction::Circular, EasingType::Out)
        };

        assert_eq!(f(0.0), 0.0);
        assert_eq!(f(0.1), 43.588997);
        assert_eq!(f(0.15), 52.67827);
        assert_eq!(f(0.25), 66.14378);
        assert_eq!(f(0.5), 86.60254);
        assert_eq!(f(0.75), 96.824585);
        assert_eq!(f(1.0), 100.0);
    }

    #[test]
    fn all_easings_in_out_has_symmetry(){
        let easing_func_list = [
            EasingFunction::Linear,
            EasingFunction::Sine,
            EasingFunction::Circular,
            EasingFunction::Quadratic,
            EasingFunction::Cubic,
            EasingFunction::Quartic,
            EasingFunction::Quintic,
            EasingFunction::Exponential,
            // EasingFunction::Back,
            EasingFunction::Bounce,
            // EasingFunction::Elastic,
        ];
        const precision: f32 = 0.001;
        for easing_func in easing_func_list.iter() {
            let f = move |t: f32| -> f32 {
                easing(t, 0.0, 1.0, 1.0, *easing_func, EasingType::InOut)
            };
            assert_float_relative_eq!(f(0.0), 1.0 - f(1.0), precision);
            assert_float_relative_eq!(f(0.25), 1.0 - f(0.75), precision);
            assert_float_relative_eq!(f(0.3), 1.0 - f(0.7), precision);
            assert_float_relative_eq!(f(0.5), 1.0 - f(0.5), precision);
        }
    }

    #[test]
    fn all_easings_out_offset_works(){
        let easing_func_list = [
            EasingFunction::Linear,
            EasingFunction::Sine,
            EasingFunction::Circular,
            EasingFunction::Quadratic,
            EasingFunction::Cubic,
            EasingFunction::Quartic,
            EasingFunction::Quintic,
            EasingFunction::Exponential,
            EasingFunction::Back,
            EasingFunction::Bounce,
            EasingFunction::Elastic,
        ];

        const precision: f32 = 0.001;
        const start: f32 = 10.0;
        const width: f32 = 70.0;
        const offset: f32 = 100.0;
        
        for easing_func in easing_func_list.iter() {
            let f1 = move |t: f32| -> f32 {
                easing(t, start, width, 1.0, *easing_func, EasingType::Out)
            };
            let f2 = move |t: f32| -> f32 {
                easing(t, start + offset, width, 1.0, *easing_func, EasingType::Out)
            };
            assert_float_relative_eq!(f1(0.0) + offset, f2(0.0), precision);
            assert_float_relative_eq!(f1(0.25) + offset, f2(0.25), precision);
            assert_float_relative_eq!(f1(0.5) + offset, f2(0.5), precision);
            assert_float_relative_eq!(f1(0.75) + offset, f2(0.75), precision);
        }
    }

    #[test]
    fn all_easings_in_offset_works(){
        let easing_func_list = [
            EasingFunction::Linear,
            EasingFunction::Sine,
            EasingFunction::Circular,
            EasingFunction::Quadratic,
            EasingFunction::Cubic,
            EasingFunction::Quartic,
            EasingFunction::Quintic,
            EasingFunction::Exponential,
            EasingFunction::Back,
            EasingFunction::Bounce,
            EasingFunction::Elastic,
        ];

        const precision: f32 = 0.001;
        const start: f32 = 10.0;
        const width: f32 = 70.0;
        const offset: f32 = 100.0;
        
        for easing_func in easing_func_list.iter() {
            let f1 = move |t: f32| -> f32 {
                easing(t, start, width, 1.0, *easing_func, EasingType::In)
            };
            let f2 = move |t: f32| -> f32 {
                easing(t, start + offset, width, 1.0, *easing_func, EasingType::In)
            };
            assert_float_relative_eq!(f1(0.0) + offset, f2(0.0), precision);
            assert_float_relative_eq!(f1(0.25) + offset, f2(0.25), precision);
            assert_float_relative_eq!(f1(0.5) + offset, f2(0.5), precision);
            assert_float_relative_eq!(f1(0.75) + offset, f2(0.75), precision);
        }
    }

    #[test]
    fn all_easings_in_scale_works(){
        let easing_func_list = [
            EasingFunction::Linear,
            EasingFunction::Sine,
            EasingFunction::Circular,
            EasingFunction::Quadratic,
            EasingFunction::Cubic,
            EasingFunction::Quartic,
            EasingFunction::Quintic,
            EasingFunction::Exponential,
            EasingFunction::Back,
            EasingFunction::Bounce,
            EasingFunction::Elastic,
        ];

        const precision: f32 = 0.001;
        const start: f32 = 0.0;
        const width: f32 = 70.0;
        const scale: f32 = 2.0;

        for easing_func in easing_func_list.iter() {
            let f1 = move |t: f32| -> f32 {
                easing(t, start, width, 1.0, *easing_func, EasingType::In)
            };
            let f2 = move |t: f32| -> f32 {
                easing(t, start, width * scale, 1.0, *easing_func, EasingType::In)
            };
            assert_float_relative_eq!(f1(0.0) * scale, f2(0.0), precision);
            assert_float_relative_eq!(f1(0.25) * scale, f2(0.25), precision);
            assert_float_relative_eq!(f1(0.5) * scale, f2(0.5), precision);
            assert_float_relative_eq!(f1(0.75) * scale, f2(0.75), precision);
        }
    }

    #[test]
    fn all_easings_out_scale_works(){
        let easing_func_list = [
            EasingFunction::Linear,
            EasingFunction::Sine,
            EasingFunction::Circular,
            EasingFunction::Quadratic,
            EasingFunction::Cubic,
            EasingFunction::Quartic,
            EasingFunction::Quintic,
            EasingFunction::Exponential,
            EasingFunction::Back,
            EasingFunction::Bounce,
            EasingFunction::Elastic,
        ];

        const precision: f32 = 0.001;
        const start: f32 = 0.0;
        const width: f32 = 70.0;
        const scale: f32 = 2.0;

        for easing_func in easing_func_list.iter() {
            let f1 = move |t: f32| -> f32 {
                easing(t, start, width, 1.0, *easing_func, EasingType::Out)
            };
            let f2 = move |t: f32| -> f32 {
                easing(t, start, width * scale, 1.0, *easing_func, EasingType::Out)
            };
            assert_float_relative_eq!(f1(0.0) * scale, f2(0.0), precision);
            assert_float_relative_eq!(f1(0.25) * scale, f2(0.25), precision);
            assert_float_relative_eq!(f1(0.5) * scale, f2(0.5), precision);
            assert_float_relative_eq!(f1(0.75) * scale, f2(0.75), precision);
        }
    }

    /// test minus changing value
    #[test]
    fn easing_linear_minus_changing_value_test() {
        let f = move |t: f32| -> f32 {
            easing(t, 0.0, -100.0, 1.0, EasingFunction::Linear, EasingType::In)
        };
        assert_eq!(f(0.0), 0.0);
        assert_eq!(f(0.5), -50.0);
        assert_eq!(f(1.0), -100.0);
    }

    /// test minus changing value
    #[test]
    fn easing_circ_minus_changing_value_test() {
        let f = move |t: f32| -> f32 {
            easing(t, 0.0, -100.0, 1.0, EasingFunction::Circular, EasingType::In)
        };
        const precision: f32 = 0.001;
        assert_float_relative_eq!(f(0.0), 0.0, precision);
        assert_float_relative_eq!(f(0.25), -3.1754136, precision);
        assert_float_relative_eq!(f(0.5), -13.397461, precision);
        assert_float_relative_eq!(f(0.75), -33.85622, precision);
        assert_float_relative_eq!(f(0.85), -47.32173, precision);
        assert_float_relative_eq!(f(0.95), -68.77501, precision);
        assert_float_relative_eq!(f(1.0), -100.0, precision);
    }

}