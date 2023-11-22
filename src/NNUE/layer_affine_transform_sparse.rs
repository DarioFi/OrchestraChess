use std::fs::File;
use crate::NNUE::read_utilities::{get_padded, read_i32, read_i8};

type BiasType = i32;
type InputType = i8;
type WeightType = i8;
type OutputType = i32;

pub struct TransformSparse {
    in_dims: usize,
    out_dims: usize,
    biases: Vec<BiasType>,
    weights: Vec<Vec<WeightType>>,
}


impl TransformSparse {
    pub(crate) fn read_parameters(file: &mut File, out_dims: usize, in_dims: usize) -> TransformSparse {
        let mut biases = Vec::new();
        for i in 0..out_dims {
            biases.push(read_i32(file));
        }

        let mut weights: Vec<Vec<i8>> = Vec::new();
        for i in 0..out_dims {
            let mut weights_inner = Vec::new();
            for j in 0..get_padded(in_dims) {
                weights_inner.push(read_i8(file));
            }
            weights.push(weights_inner);
        }

        TransformSparse {
            in_dims,
            out_dims,
            biases,
            weights,
        }
    }
}
