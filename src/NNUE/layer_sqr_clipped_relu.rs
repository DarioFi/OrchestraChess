type InputType = i32;
type OutputType = i8;

const OUTPUT_SCALE: i8 = 16;
const WEIGHT_SCALE_BITS: i8 = 6;

pub(crate) struct SqrClippedReLU {}

impl SqrClippedReLU {
    pub(crate) fn new() -> SqrClippedReLU {
        SqrClippedReLU {}
    }

    pub(crate) fn propagate(&self, input: &Vec<InputType>) -> Vec<OutputType> {
        /*
        for (IndexType i = Start; i < InputDimensions; ++i)
        {
            output[i] = static_cast<OutputType>(
              // Really should be /127 but we need to make it fast so we right shift
              // by an extra 7 bits instead. Needs to be accounted for in the trainer.
              std::min(127ll, ((long long) (input[i]) * input[i]) >> (2 * WeightScaleBits + 7)));
        }
        */
        let mut output = Vec::with_capacity(input.len());
        for i in 0..input.len() {
            let mut output_value = input[i] as i64;
            output_value *= input[i] as i64;
            output_value >>= (2 * WEIGHT_SCALE_BITS + 7);
            output_value = std::cmp::min(127, output_value);
            output.push(output_value as OutputType);
        }
        output
    }
}