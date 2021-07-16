pub const WIDTH: f32 = 400.0; //width and height of
pub const HEIGHT: f32 = 400.0; //screen
pub const NUM_NEURONS: usize = 8; //should be powers of 2
pub const NUM_BRAINS: usize = 10; //number of brains  -- in ga talk population
pub const NUM_ANGLES: usize = 8; //rover has 8 possible directions it can travel
                             //e,ne,n,nw,w,sw,s,se -- kind of like the unit circle in trig
pub const ANGLES_DX: [f32; 8] = [1.0, 1.0, 0.0, -1.0, -1.0, -1.0, 0.0, 1.0];
pub const ANGLES_DY: [f32; 8] = [0.0, 1.0, 1.0, 1.0, 0.0, -1.0, -1.0, -1.0];
pub const NUM_SENSORS: usize = 3; //number of antennae
pub const SENSOR_LENGTH: f32 = 60.0; //length of an antenna
pub const MAX_LOOP_KNT: usize = 2000; //can't let them live forever


