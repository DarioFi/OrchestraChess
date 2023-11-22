type InputType = i32;
type OutputType = u8;

const OUTPUT_SCALE: i8 = 16;
const WEIGHT_SCALE_BITS: i8 = 6;

pub(crate) struct SqrClippedReLU {}

impl SqrClippedReLU {
    pub(crate) fn new() -> SqrClippedReLU {
        SqrClippedReLU {}
    }

    fn propagate(&self, input: Vec<InputType>) -> Vec<OutputType> {
        let mut output = Vec::with_capacity(input.len());

        let start = 0;

        for i in input.iter() {
            output.push((i * i / (1 << (2 * WEIGHT_SCALE_BITS + 7))).min(127) as OutputType);
        }

        output
    }
}