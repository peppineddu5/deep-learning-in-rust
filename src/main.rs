use math::activationFunction;
//use neuralNetwork::Neuron;

mod math;

fn Neuron(weights: &[f64], input: &[f64], activationFunction: activationFunction) -> f64 {
    let mut pa: f64 = 0.0000;
    for (i, e) in input.iter().enumerate() {
        pa += e * weights[i];
    }
    //pa = math::sigmoid(pa);

    pa = match activationFunction {
        activationFunction::sigmoid => math::sigmoid(pa),
        activationFunction::tahn => pa.tanh(),
        activationFunction::relu => pa.max(0.0),
        activationFunction::leakyrelu => (pa * 0.1).max(pa),
        activationFunction::step => math::step(pa),
        activationFunction::linear => pa,
        activationFunction::signum => math::signum(pa),
        activationFunction::bipolarSigmoid => math::bipolarSigmoid(pa),
        activationFunction::hyperbolicTangent => math::hyperbolicTangent(pa),
        activationFunction::cosine => pa.cos(),
        activationFunction::absolute => pa.abs(),
    };

    return pa;
}
fn learn(weights: &mut [f64], x: &[f64], target: f64, activationFunction: activationFunction) {
    let out = Neuron(&weights, x, activationFunction);
    let error = target - out;

    println!("input {:?} target {} error {}", x, target, error);
    let learningRate = 0.600;
    for i in 0..weights.len() {
        weights[i] += learningRate * error * x[i]
    }
}
fn main() {
    /* let pi = Neuron {
        weights: &[4.0, 5.0],
    };
    pi.output(); */

    let mut weights: [f64; 2] = [4.0, 5.0];
    //let input = [1.0000, 0.0000, -1.0000];
    //println!("{}", Neuron(&weights, &input));
    let x = [
        [1.0000, 1.0000],
        [1.0000, 0.0000],
        [0.0000, 1.0000],
        [0.0000, 0.0000],
    ];

    let y = [0.0000, 1.0000, 0.0000, 0.0000];
    for epoch in 0..100 {
        println!("Epoch: {}", epoch);
        for i in 0..x.len() {
            let target = y[i];
            learn(&mut weights, &x[i], target, activationFunction::step);
        }
    }
    println!("{:?}", weights);
}
