// https://www.youtube.com/watch?v=A8qTRBc8Bws&t=24s



pub enum Compound {
    N2, // step 1: atmospheric
    NH4, // step 2: fixation (positive charge)
    NO2, // step 3:Nitrification (negitive charge)
    NO3, // step 4: more Nitrification (negitive charge)
}

pub enum BacteriaType {
	Step2,
	Step3,
	Step4,
}

pub struct Nitrogen {
    pub compound: Compound,
    pub comsumion_rate: i32,
}


pub struct Bacteria {
    backType: BacteriaType,
    conversion_rate: i32
}

pub struct Soil {
    BacteriaAmount: Vec<Bacteria>,
    NitrogenAmount: Vec<Nitrogen>
}

