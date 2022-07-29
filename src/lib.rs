/* 
use math::activationFunction;
//use neuralNetwork::Neuron;

mod math;

pub struct Neuron<'a> {
    pub weights: &'a [f64],
}
impl Neuron<'_> {
    pub fn Neuron(&mut self, input: &[f64], activationFunction: activationFunction) -> f64 {
        let mut pa: f64 = 0.0000;
        for (i, e) in input.iter().enumerate() {
            pa += e * self.weights[i];
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

    pub fn learn(&mut self, x: &[f64], target: f64, activationFunction: activationFunction) {
        let out = Neuron::<'_>::Neuron(&mut self, x, activationFunction);
        let error = target - out;

        println!("input {:?} target {} error {}", x, target, error);
        let learningRate = 0.600;
        for i in 0..self.weights.len() {
            self.weights[i] += learningRate * error * x[i]
        }
    }
}
*/