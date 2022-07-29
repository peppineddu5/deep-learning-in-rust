pub fn exp(x: f64) -> f64 {
    const MAX_ITER: i32 = 200;
    let mut sum = 1.0;
    let mut term = 1.0;

    for n in 1..MAX_ITER {
        term *= x / n as f64;
        sum += term;
    }
    return sum;
}
pub enum activationFunction {
    sigmoid,
    tahn,
    relu,
    leakyrelu,
    step,
    linear,
    signum,
    bipolarSigmoid,
    hyperbolicTangent,
    cosine,
    absolute,
}

pub fn sigmoid(val: f64) -> f64 {
    return 1.0000 / (1.0000 + exp(-val));
}
pub fn step(val: f64) -> f64 {
    if val > 0.0 {
        return 1.00000;
    }
    return 0.0000;
}
pub fn signum(val: f64) -> f64 {
    if val < 0.0 {
        return -1.0;
    } else if val == 0.0 {
        return 0.0;
    } else {
        return 1.0;
    }
}
pub fn bipolarSigmoid(val: f64) -> f64 {
    return -1.0 + 2.0 / (1.0 + exp(val));
}

pub fn hyperbolicTangent(val: f64) -> f64 {
    return (1.0 - exp(-(2.0 * val))) / 1.0 + exp(-(2.0 * val));
}
