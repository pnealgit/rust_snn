
use nannou::rand::random_range;
extern crate constants;

#[derive(Clone, Debug)]
pub struct Brain {
    pub fitness: f32,
    pub xsign: [u8; constants::NUM_NEURONS as usize],
    pub iconn: [[u8; constants::NUM_NEURONS as usize]; constants::NUM_NEURONS as usize],
    pub nconn: [[u8; constants::NUM_NEURONS as usize]; constants::NUM_NEURONS as usize],
}

impl Brain {
    pub fn new() -> Self {
        let fitness = 0.0;
        let mut xsign = [0; constants::NUM_NEURONS];
        for ix in 0..constants::NUM_NEURONS {
            xsign[ix] = random_range(0, 2) as u8;
        }
        let iconn = [[1; constants::NUM_NEURONS as usize]; constants::NUM_NEURONS as usize];
        let mut nconn = [[0; constants::NUM_NEURONS as usize]; constants::NUM_NEURONS as usize];
        for ix in 0..constants::NUM_NEURONS {
            for iy in 0..constants::NUM_NEURONS {
                nconn[ix][iy] = random_range(0, 2);
            }
        }
                Brain {
            fitness,
            xsign,
            iconn,
            nconn,
        }   
    } //end of new
} //end of impl Brain


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
