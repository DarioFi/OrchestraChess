type InputType = i32;
type OutputType = u8;

pub struct ClippedRelu {
}

impl ClippedRelu {
    pub(crate) fn new() -> ClippedRelu {
        ClippedRelu {
        }
    }

    // todo: check that shifts are ok
    fn propagate(&self, input: Vec<InputType>) -> Vec<OutputType> {
        // clamp vector
        input.into_iter().map(|value| {
            let scaled_value = (value >> 8) as OutputType; // Adjust shift amount as needed
            scaled_value.clamp(0, 127)
        }).collect()
    }

}