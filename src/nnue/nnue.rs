use std::fs::File;
use std::io::Read;
use crate::nnue::architecture::Architecture;
use crate::nnue::feature_transformer::FeatureTransformer;
use crate::nnue::read_utilities::read_u32;

const FILE_NAME: &str = "nn-0000000000a0.nnue";


const LAYER_STACKS: usize = 8;


pub(crate) struct Nnue {
    version: u32,
    hash_value: u32,
    size: u32,
    pub feature_transformer: FeatureTransformer,
    pub networks: Vec<Architecture>,

}


impl Nnue {
    pub fn init() -> Nnue {
        let mut nnue = Nnue::new();

        // read file
        let mut file = File::open(FILE_NAME).expect("Unable to open file");
        nnue.read_headers(&mut file);

        let header_transform_hash = read_u32(&mut file);
        println!("Header transform hash: {}", header_transform_hash);
        nnue.feature_transformer = FeatureTransformer::read_parameters(&mut file);

        // here we itarate over the layers and read the weights
        println!("The next hash should all be the same");
        for i in 0..LAYER_STACKS {
            let header_layer_hash = read_u32(&mut file);
            //todo: maybe add checks for those hashes
            println!("Layer {} hash: {}", i, header_layer_hash);
            nnue.networks.push(Architecture::read_parameters(&mut file));
        }
        nnue
    }

    fn new() -> Nnue {
        Nnue {
            version: 0,
            hash_value: 0,
            size: 0,
            feature_transformer: FeatureTransformer::new(),
            networks: Vec::new(),
        }
    }

    fn read_headers(&mut self, file: &mut File) {
        let version = read_u32(file);
        println!("Version: {}", version);
        let hash_value = read_u32(file);
        println!("Hash value: {}", hash_value);
        let size = read_u32(file);
        println!("Size: {}", size);

        self.version = version;
        self.hash_value = hash_value;
        self.size = size;
        let mut buffer = vec![0_u8; self.size as usize];
        file.read_exact(&mut buffer).expect("Unable to read file");
    }
}