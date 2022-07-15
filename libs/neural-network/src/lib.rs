pub struct Network {
    layers: Vec<Layer>,
}

struct Layer {
    neurons: Vec<Neuron>,
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

pub struct LayerTopology {
    pub neurons: usize,
}

impl Network {
    pub fn propagate(&self, mut input: Vec<f32>) -> Vec<f32> {
        for layer in &self.layers {
            input = layer.propagate(input);
        }

        return input;
    }
    pub fn random(layers: &[LayerTopology]) -> self {
        assert!(layers.len() > 1);

        let mut built_layers = Vec::new();

        // windows takes 2 values at a time with 1 stride
        // https://doc.rust-lang.org/stable/std/primitive.slice.html#method.windows
        for adjacent_layers in layers.windows(2){
            let input_neuron = adjacent_layers[0].neurons;
            let output_neuron = adjacent_layers[1].neurons;

            built_layers.push(Layer::random(
                input_neuron,
                output_neuron
            ));
        }

        return Self { layers : built_layers };
    }
}

impl Layer {
    fn propagate(&self, input: Vec<f32>) -> Vec<f32> {
        //Preallocate the correct size vector (More efficient)
        let mut outputs = Vec::with_capacity(self.neurons.len());

        for neuron in &self.neurons {
            let output = neuron.propagate(&input);
            outputs.push(output);
        }

        return outputs;
    }
    fn random(input_neurons: usize, output_neurons: usize) -> self {
        let mut neurons = Vec::new();

        for _ in 0..output_neurons {
            neurons.push(Neuron::random(input_neurons));
        }

        return self { neurons };
    }
}

impl Neuron {
    fn propagate(&self, inputs: &[f32]>) -> f32 {
        // Create iterator for inputs and zip it together with vector for weights, then map
        // and insure sum is f32
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();
        // Add bias and simulate ReLU
        return (self.bias + output).max(0.0);
    }
    fn random(output_size: usize) -> self {
        let mut rng = rand::thread_rng();

        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();
        
        return self { bias, weights };
    }
}