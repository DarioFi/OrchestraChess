use std::fs::File;
use crate::nnue::feature_transformer::{ TRANSFORMED_FEATURE_DIMENSIONS};
use crate::nnue::layer_affine_transform::AffineTransform;
use crate::nnue::layer_affine_transform_sparse::TransformSparse;
use crate::nnue::layer_sqr_clipped_relu::SqrClippedReLU;
use crate::nnue::layer_clipped_relu::ClippedRelu;

pub struct Architecture {
    fc_0: TransformSparse,
    ac_sqr_0: SqrClippedReLU,
    ac_0: ClippedRelu,
    fc_1: AffineTransform,
    ac_1: ClippedRelu,
    fc_2: AffineTransform,
}

pub const FC_0_OUT_DIMS: usize = 15;
pub const FC_1_OUT_DIMS: usize = 32;

pub const OUTPUT_SCALE: i32 = 16;
const WEIGHT_SCALE_BITS: i32 = 6;

impl Architecture {
    pub(crate) fn read_parameters(file: &mut File) -> Architecture {
        let fc_0 = TransformSparse::read_parameters(file, FC_0_OUT_DIMS + 1, TRANSFORMED_FEATURE_DIMENSIONS);
        let ac_sqr_0 = SqrClippedReLU::new();
        let ac_0 = ClippedRelu::new();
        let fc_1 = AffineTransform::read_parameters(file, FC_1_OUT_DIMS, FC_0_OUT_DIMS * 2);
        let ac_1 = ClippedRelu::new();
        let fc_2 = AffineTransform::read_parameters(file, 1, FC_1_OUT_DIMS);

        Architecture {
            fc_0,
            ac_sqr_0,
            ac_0,
            fc_1,
            ac_1,
            fc_2,
        }
    }

    pub fn propagate(&self, transformed_features: [i8; TRANSFORMED_FEATURE_DIMENSIONS]) -> i32 {
        let mut fc_0_out = self.fc_0.propagate(transformed_features);

        assert_eq!(fc_0_out.len(), 16);
        let remainder = fc_0_out.last().unwrap();
        assert_eq!(fc_0_out.len(), 16);

        let ac_sqr_0_out = self.ac_sqr_0.propagate(&fc_0_out);
        let ac_0_out = self.ac_0.propagate(&fc_0_out);

        let mut combined_output = ac_sqr_0_out.clone();
        for x in ac_0_out.iter() {
            combined_output.push(*x);
        }

        let fc_1_out = self.fc_1.propagate(combined_output);
        let ac_1_out = self.ac_1.propagate(&fc_1_out);

        let fc_2_out = self.fc_2.propagate(ac_1_out);
        assert_eq!(fc_2_out.len(), 1);

        let fwd_out = *remainder as i32 * (600 * OUTPUT_SCALE) / (127 * (1 << WEIGHT_SCALE_BITS));
        let output = fc_2_out[0] + fwd_out;
        output
    }
}