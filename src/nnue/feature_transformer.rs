use std::fs::File;
use std::io::Read;
use crate::nnue::read_utilities::read_u32;

pub const TRANSFORMED_FEATURE_DIMENSIONS: usize = 2560;
pub const HALF_DIMENSIONS: usize = 2560;

pub const PSQT_BUCKETS: usize = 8;

pub type BiasType = i16;
pub type WeightType = i16;
pub type PSQTWeightType = i32;

//todo extracted by hand from stockfish
const INPUT_DIMENSIONS: usize = 22528;
const LEB128_MAGIC_STRING: &[u8; 17] = b"COMPRESSED_LEB128";
const LEB128_MAGIC_STRING_SIZE: usize = 17;

pub fn read_leb_128_bias_type(stream: &mut File, out: &mut Vec<BiasType>, count: usize) {
    // Check the presence of our LEB128 magic string
    let mut leb128_magic_string = [0_u8; LEB128_MAGIC_STRING_SIZE];
    let _ = stream.read_exact(&mut leb128_magic_string);
    assert_eq!(&leb128_magic_string, LEB128_MAGIC_STRING);

    // Ensure the type is signed (not implemented for unsigned types)
    // assert!(std::is_signed::<T>());

    const BUF_SIZE: usize = 4096;
    let mut buf = [0_u8; BUF_SIZE];

    let mut bytes_left = read_u32(stream);

    let mut buf_pos = BUF_SIZE;
    for _i in 0..count {
        let mut result: BiasType = Default::default();
        let mut shift = 0;

        while shift < std::mem::size_of::<BiasType>() * 8 {
            if buf_pos == BUF_SIZE {
                let bytes_to_read = std::cmp::min(bytes_left, BUF_SIZE as u32);
                let _ = stream.read_exact(&mut buf[0..bytes_to_read as usize]);
                buf_pos = 0;
            }

            let byte = buf[buf_pos];
            buf_pos += 1;
            bytes_left -= 1;

            result = result | ((byte & 0x7f) as BiasType) << shift;
            shift += 7;

            if (byte & 0x80) == 0 {
                out.push(
                    if std::mem::size_of::<BiasType>() * 8 <= shift || (byte & 0x40) == 0 {
                        result
                    } else {
                        result | !((1 << shift) - 1)
                    }
                );
                break;
            }
        }
    }

    assert_eq!(bytes_left, 0);
}

pub fn read_leb_128_psqt_type(stream: &mut File, out: &mut Vec<PSQTWeightType>, count: usize) {
    // Check the presence of our LEB128 magic string
    let mut leb128_magic_string = [0_u8; LEB128_MAGIC_STRING_SIZE];
    let _ = stream.read_exact(&mut leb128_magic_string);
    assert_eq!(&leb128_magic_string, LEB128_MAGIC_STRING);

    // Ensure the type is signed (not implemented for unsigned types)
    // assert!(std::is_signed::<T>());

    const BUF_SIZE: usize = 4096;
    let mut buf = [0_u8; BUF_SIZE];

    let mut bytes_left = read_u32(stream);

    let mut buf_pos = BUF_SIZE;
    for _i in 0..count {
        let mut result: PSQTWeightType = Default::default();
        let mut shift = 0;

        while shift < std::mem::size_of::<PSQTWeightType>() * 8 {
            if buf_pos == BUF_SIZE {
                let bytes_to_read = std::cmp::min(bytes_left, BUF_SIZE as u32);
                let _ = stream.read_exact(&mut buf[0..bytes_to_read as usize]);
                buf_pos = 0;
            }

            let byte = buf[buf_pos];
            buf_pos += 1;
            bytes_left -= 1;

            result = result | ((byte & 0x7f) as PSQTWeightType) << shift;
            shift += 7;

            if (byte & 0x80) == 0 {
                out.push(if std::mem::size_of::<PSQTWeightType>() * 8 <= shift || (byte & 0x40) == 0 {
                    result
                } else {
                    result | !((1 << shift) - 1)
                });
                break;
            }
        }
    }

    assert_eq!(bytes_left, 0);
}

// todo: check that no transpose is needed


pub struct FeatureTransformer {
    bias: Vec<BiasType>,
    weights: Vec<Vec<WeightType>>,
    psqt_weights: Vec<Vec<PSQTWeightType>>,
    pub my_acc_stack: Vec<[BiasType; TRANSFORMED_FEATURE_DIMENSIONS]>,
    pub opp_acc_stack: Vec<[BiasType; TRANSFORMED_FEATURE_DIMENSIONS]>,
    pub my_psq_acc_stack: Vec<[PSQTWeightType; PSQT_BUCKETS]>,
    pub opp_psq_acc_stack: Vec<[PSQTWeightType; PSQT_BUCKETS]>,
}


// we will also update the feature transformer here

impl FeatureTransformer {
    pub(crate) fn read_parameters(stream: &mut File) -> FeatureTransformer {
        let mut bias: Vec<BiasType> = Vec::new();
        read_leb_128_bias_type(stream, &mut bias, HALF_DIMENSIONS);

        let mut weights_linear: Vec<WeightType> = Vec::new();
        read_leb_128_bias_type(stream, &mut weights_linear, HALF_DIMENSIONS * INPUT_DIMENSIONS);

        let mut psqtweight: Vec<PSQTWeightType> = Vec::new();
        read_leb_128_psqt_type(stream, &mut psqtweight, PSQT_BUCKETS * INPUT_DIMENSIONS);


        let mut weights: Vec<Vec<WeightType>> = Vec::new();
        let mut psqtweights: Vec<Vec<PSQTWeightType>> = Vec::new();


        // what are we doing exactly here ???
        // check dimensions and whether we are reading in the right order (i dont think so)

        for i in 0..INPUT_DIMENSIONS {
            let mut temp: Vec<WeightType> = Vec::new();
            for j in 0..HALF_DIMENSIONS {
                temp.push(weights_linear[i * HALF_DIMENSIONS + j]);
            }
            weights.push(temp);
        }
        assert_eq!(weights.len(), INPUT_DIMENSIONS);
        assert_eq!(weights[0].len(), HALF_DIMENSIONS);

        for i in 0..INPUT_DIMENSIONS {
            let mut temp2: Vec<PSQTWeightType> = Vec::new();
            for j in 0..PSQT_BUCKETS {
                temp2.push(psqtweight[i * PSQT_BUCKETS + j]);
            }
            psqtweights.push(temp2);
        }

        FeatureTransformer {
            bias,
            weights,
            psqt_weights: psqtweights,
            my_acc_stack: vec![],
            opp_acc_stack: vec![],
            my_psq_acc_stack: vec![],
            opp_psq_acc_stack: vec![],
        }
    }

    pub fn new() -> FeatureTransformer {
        FeatureTransformer {
            bias: Vec::new(),
            weights: Vec::new(),
            psqt_weights: Vec::new(),
            my_acc_stack: vec![],
            opp_acc_stack: vec![],
            my_psq_acc_stack: vec![],
            opp_psq_acc_stack: vec![],
        }
    }

    pub(crate) fn transform(&self, _bucket: i32) -> (i32, [i8; HALF_DIMENSIONS]) {
        let mut result = [0_i8; HALF_DIMENSIONS];
        // my accumulation part
        let my_acc = self.my_acc_stack.last().unwrap();
        for i in 0..(HALF_DIMENSIONS / 2) {
            let sum0 = my_acc[i];
            let sum1 = my_acc[i + HALF_DIMENSIONS / 2];
            let c0 = sum0.clamp(0, 127);
            let c1 = sum1.clamp(0, 127);
            result[i] = (c0 * c1 / 128) as i8;
        }

        let opp_acc = self.opp_acc_stack.last().unwrap();
        for i in 0..(HALF_DIMENSIONS / 2) {
            let sum0 = opp_acc[i];
            let sum1 = opp_acc[i + HALF_DIMENSIONS / 2];
            let c0 = sum0.clamp(0, 127);
            let c1 = sum1.clamp(0, 127);
            result[i + HALF_DIMENSIONS / 2] = (c0 * c1 / 128) as i8;
        }

        let my_psq = self.my_psq_acc_stack.last().unwrap();
        let opp_psq = self.opp_psq_acc_stack.last().unwrap();
        let x = (my_psq[_bucket as usize] - opp_psq[_bucket as usize]) / 2;
        (x, result)
    }


    pub(crate) fn add_to_accumulator(&self, index: usize, acc: &mut [BiasType; HALF_DIMENSIONS]) {
        for i in 0..TRANSFORMED_FEATURE_DIMENSIONS {
            acc[i] += self.weights[index][i];
        }
    }

    pub(crate) fn subtract_from_accumulator(&self, index: usize, acc: &mut [BiasType; HALF_DIMENSIONS]) {
        for i in 0..TRANSFORMED_FEATURE_DIMENSIONS {
            acc[i] -= self.weights[index][i];
        }
    }

    pub(crate) fn get_bias(&self) -> [BiasType; TRANSFORMED_FEATURE_DIMENSIONS] {
        let mut bias = [0; TRANSFORMED_FEATURE_DIMENSIONS];
        for i in 0..TRANSFORMED_FEATURE_DIMENSIONS {
            bias[i] = self.bias[i];
        }
        bias
    }
    pub(crate) fn add_to_accumulator_psq(&self, index: usize, acc: &mut [PSQTWeightType; PSQT_BUCKETS]) {
        for i in 0..PSQT_BUCKETS {
            acc[i] += self.psqt_weights[index][i];
        }
    }

    pub(crate) fn subtract_from_accumulator_psq(&self, index: usize, acc: &mut [PSQTWeightType; PSQT_BUCKETS]) {
        for i in 0..PSQT_BUCKETS {
            acc[i] -= self.psqt_weights[index][i];
        }
    }
}
