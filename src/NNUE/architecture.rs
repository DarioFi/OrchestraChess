use std::fs::File;
use crate::NNUE::feature_transformer::TRANSFORMED_FEATURE_DIMENSIONS;
use crate::NNUE::layer_affine_transform::AffineTransform;
use crate::NNUE::layer_affine_transform_sparse::TransformSparse;
use crate::NNUE::layer_sqr_clipped_relu::SqrClippedReLU;
use crate::NNUE::layer_clipped_relu::ClippedRelu;

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


impl Architecture {
    pub(crate) fn read_parameters(file: &mut File) -> Architecture {

        let fc_0 = TransformSparse::read_parameters(file, TRANSFORMED_FEATURE_DIMENSIONS, FC_0_OUT_DIMS);
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
}