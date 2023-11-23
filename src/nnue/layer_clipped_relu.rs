type InputType = i32;
type OutputType = i8;

const WEIGHT_SCALE_BITS: u8 = 6;

pub struct ClippedRelu {}

impl ClippedRelu {
    pub(crate) fn new() -> ClippedRelu {
        ClippedRelu {}
    }

    pub(crate) fn propagate(&self, input: &Vec<InputType>) -> Vec<OutputType> {
        /*
        WeightScaleBits = 6;
        for (IndexType i = Start; i < InputDimensions; ++i)
        {
            output[i] = static_cast<OutputType>(std::clamp(input[i] >> WeightScaleBits, 0, 127));
        }*/
        // clamp vector
        input.into_iter().map(|value| {
            let scaled_value = value >> WEIGHT_SCALE_BITS;
            let resc = scaled_value.clamp(0, 127);
            resc as OutputType
        }).collect()
    }
}