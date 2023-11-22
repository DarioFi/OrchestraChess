use std::fs::File;
use std::io::Read;
use std::process::exit;
use rand::distributions::BernoulliError::InvalidProbability;
use crate::nnue::read_utilities::read_u32;
use crate::r#move::Move;

pub const TRANSFORMED_FEATURE_DIMENSIONS: usize = 2560;
pub const HALF_DIMENSIONS: usize = TRANSFORMED_FEATURE_DIMENSIONS;
pub const PSQT_BUCKETS: usize = 8;

pub type BiasType = i16;
pub type WeightType = i16;
pub type PSQTWeightType = i32;

//todo extracted by hand from stockfish
pub const INPUT_DIMENSIONS: usize = 22528;
const LEB128_MAGIC_STRING: &[u8; 17] = b"COMPRESSED_LEB128";
const LEB128_MAGIC_STRING_SIZE: usize = 17;

pub fn read_leb_128_bias_type(stream: &mut File, out: &mut Vec<BiasType>, count: usize) {
    // Check the presence of our LEB128 magic string
    let mut leb128_magic_string = [0_u8; LEB128_MAGIC_STRING_SIZE];
    stream.read_exact(&mut leb128_magic_string);
    assert_eq!(&leb128_magic_string, LEB128_MAGIC_STRING);

    // Ensure the type is signed (not implemented for unsigned types)
    // assert!(std::is_signed::<T>());

    const BUF_SIZE: usize = 4096;
    let mut buf = [0_u8; BUF_SIZE];

    let mut bytes_left = read_u32(stream);

    let mut buf_pos = BUF_SIZE;
    for i in 0..count {
        let mut result: BiasType = Default::default();
        let mut shift = 0;

        while shift < std::mem::size_of::<BiasType>() * 8 {
            if buf_pos == BUF_SIZE {
                let bytes_to_read = std::cmp::min(bytes_left, BUF_SIZE as u32);
                stream.read_exact(&mut buf[0..bytes_to_read as usize]);
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
// todo: if those function work it is a miracle, transpiled with gpt to have a blueprint

pub fn read_leb_128_psqt_type(stream: &mut File, out: &mut Vec<PSQTWeightType>, count: usize) {
    // Check the presence of our LEB128 magic string
    let mut leb128_magic_string = [0_u8; LEB128_MAGIC_STRING_SIZE];
    stream.read_exact(&mut leb128_magic_string);
    assert_eq!(&leb128_magic_string, LEB128_MAGIC_STRING);

    // Ensure the type is signed (not implemented for unsigned types)
    // assert!(std::is_signed::<T>());

    const BUF_SIZE: usize = 4096;
    let mut buf = [0_u8; BUF_SIZE];

    let mut bytes_left = read_u32(stream);

    let mut buf_pos = BUF_SIZE;
    for i in 0..count {
        let mut result: PSQTWeightType = Default::default();
        let mut shift = 0;

        while shift < std::mem::size_of::<PSQTWeightType>() * 8 {
            if buf_pos == BUF_SIZE {
                let bytes_to_read = std::cmp::min(bytes_left, BUF_SIZE as u32);
                stream.read_exact(&mut buf[0..bytes_to_read as usize]);
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
    PSQTWeights: Vec<Vec<PSQTWeightType>>,
    previous_features: Vec<[i8; TRANSFORMED_FEATURE_DIMENSIONS]>,
}

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

        for i in 0..HALF_DIMENSIONS {
            let mut temp: Vec<WeightType> = Vec::new();
            for j in 0..INPUT_DIMENSIONS {
                temp.push(weights_linear[i * HALF_DIMENSIONS + j]);
            }
            weights.push(temp);
        }
        for i in 0..PSQT_BUCKETS {
            let mut temp2: Vec<PSQTWeightType> = Vec::new();
            for j in 0..INPUT_DIMENSIONS {
                temp2.push(psqtweight[i * TRANSFORMED_FEATURE_DIMENSIONS + j]);
            }
            psqtweights.push(temp2);
        }

        FeatureTransformer {
            bias,
            weights,
            PSQTWeights: psqtweights,
            previous_features: Vec::new(),
        }
    }

    pub fn new() -> FeatureTransformer {
        FeatureTransformer {
            bias: Vec::new(),
            weights: Vec::new(),
            PSQTWeights: Vec::new(),
            previous_features: Vec::new(),
        }
    }

    pub(crate) fn transform(&self) -> (i32, [i8; TRANSFORMED_FEATURE_DIMENSIONS]) {
        todo!()
    }

    pub(crate) fn update_transform(&mut self, mov: &Move) {
        todo!()
    }

    pub(crate) fn add_fresh_transform(&mut self) {
        todo!()
    }

    pub(crate) fn get_current_transform(&self) -> [i8; TRANSFORMED_FEATURE_DIMENSIONS] {
        self.previous_features[self.previous_features.len() - 1]
    }

    pub(crate) fn unmake_move(&mut self) {
        self.previous_features.pop();
    }
}
