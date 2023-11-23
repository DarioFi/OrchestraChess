use std::fs::File;
use crate::nnue::feature_transformer::{TRANSFORMED_FEATURE_DIMENSIONS};
use crate::nnue::read_utilities::{get_padded, read_i32, read_i8};

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
        for _i in 0..out_dims {
            biases.push(read_i32(file));
        }

        let mut weights: Vec<Vec<i8>> = Vec::new();
        for _i in 0..out_dims {
            let mut weights_inner = Vec::new();
            for _j in 0..get_padded(in_dims) {
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

    pub fn propagate(&self, input: [InputType; TRANSFORMED_FEATURE_DIMENSIONS]) -> Vec<OutputType> {
        /*
        std::memcpy(output, biases, sizeof(std::int32_t) * OutputDimensions);

        // Traverse weights in transpose order to take advantage of input sparsity
        for (IndexType i = 0; i < InputDimensions; ++i)
            if (input[i])
            {
                const std::int8_t* w  = &weights[i];
                const int          in = input[i];
                for (IndexType j = 0; j < OutputDimensions; ++j)
                    output[j] += w[j * PaddedInputDimensions] * in;
            }
             */

        //transpile this code
        let mut output = self.biases.clone();

        for i in 0..self.in_dims {
            if input[i] != 0 {
                for j in 0..self.out_dims {
                    output[j] += self.weights[j][i] as i32 * input[i] as i32;
                }
            }
        }
        output
    }
}
