use std::fs::File;
use std::mem::size_of;
use crate::nnue::read_utilities::{read_i32, read_u32, read_i8, get_padded};


type InputType = i8;
type OutputType = i32;
type BiasType = OutputType;
type WeightType = i8;

pub struct AffineTransform {
    in_dims: usize,
    out_dims: usize,

    bias: Vec<BiasType>,
    weights: Vec<Vec<WeightType>>,

}

impl AffineTransform {
    pub(crate) fn read_parameters(file: &mut File, out_dims: usize, in_dims: usize) -> AffineTransform {
        let mut bias = Vec::new();
        for _ in 0..out_dims {
            bias.push(read_i32(file));
        }

        let mut weights: Vec<Vec<i8>> = Vec::new();
        for _ in 0..out_dims {
            let mut weights_inner = Vec::new();
            for _ in 0..get_padded(in_dims) {
                weights_inner.push(read_i8(file));
            }
            weights.push(weights_inner);
        }

        AffineTransform {
            in_dims,
            out_dims,
            bias,
            weights,
        }
    }

    pub(crate) fn propagate(&self, input: Vec<InputType>) -> Vec<OutputType> {
        // todo: check that no traspose is needed
        let mut output = Vec::new();
        for i in 0..self.out_dims {
            let mut sum = self.bias[i];
            for j in 0..self.weights[i].len() {
                sum += (self.weights[i][j] * input[j]) as i32;
            }
            output.push(sum);
        }
        output
    }
}