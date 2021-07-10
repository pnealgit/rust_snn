extern crate brain;
extern crate constants;
use brain::*;
use nannou::prelude::*;


#[derive(Clone, Debug)]
pub struct Mover {
   pub position: Point2,
   pub  angle_index: usize,
   pub  old_angle_index: usize,
   pub velocity_x: f32,
   pub  velocity_y: f32,
   pub  multiplier: i32,
   pub sensor_data: [[f32; 2]; constants::NUM_SENSORS],
   pub sensor_data_vector: [u8; constants::NUM_NEURONS],
   pub isdead: i32,
   pub brain_index: usize,
   pub  brain: Brain,
   pub  brains: Vec<Brain>,
}

impl Mover {
    pub fn new(x: f32, y: f32) -> Self {
        let position = pt2(x, y);
        let angle_index = random_range(0, constants::NUM_ANGLES);
        let old_angle_index = random_range(0, constants::NUM_ANGLES);
        let velocity_x = constants::ANGLES_DX[angle_index];
        let velocity_y = constants::ANGLES_DY[angle_index];
        let multiplier = 1;
        let sensor_data = [[0.0; 2]; constants::NUM_SENSORS];
        let sensor_data_vector = [0u8; constants::NUM_NEURONS];
        let isdead = 0;
        let brain_index = random_range(0, constants::NUM_BRAINS);
        let mut brains = Vec::new();
        for _ in 0..constants::NUM_BRAINS {
            brains.push(brain::Brain::new());
        }
        let brain = brain::Brain::new();

        //Floreano -- 8 bit brain

        Mover {
            position,
            angle_index,
            old_angle_index,
            velocity_x,
            velocity_y,
            multiplier,
            sensor_data,
            sensor_data_vector,
            isdead,
            brain_index,
            brain,
            brains,
        }
    } //end of Mover new

    pub fn think(&mut self) {
        //see paper cited above
        self.build_sensor_data_vector();

        //In most cases , the sensor_data_vector will be all zeros.
        //so add some bias to make something happen.
        //
        let mut knt = 0;
        for ik in 0..constants::NUM_SENSORS as usize {
            knt += self.sensor_data_vector[ik];
        }
        if knt <= 0 {
            let ridx = random_range(0, 8);
            self.sensor_data_vector[ridx] = 1;
        }

        let leaking_constant = 1;
        let mut temp_outps = [0u8; constants::NUM_NEURONS];
        let inps = self.sensor_data_vector.clone();
        let mut memb = [0u8; constants::NUM_NEURONS];
        let mut outps = [0u8; constants::NUM_NEURONS];
        let mut fire_knt = [0; constants::NUM_SENSORS];
        let settling_time = 20; //loop through settling_time times

        for _epoch in 0..settling_time {
            for nindex in 0..constants::NUM_NEURONS {
                memb[nindex] = 0;
                if outps[nindex] == 0 {
                    //not in refactory state
                    for ilink in 0..constants::NUM_NEURONS {
                        memb[nindex] += inps[nindex] * self.brain.iconn[nindex][ilink];
                    } //end of loop on ilink
                      //count from other neurons with positive sign
                    for ilink in 0..constants::NUM_NEURONS as usize {
                        let stuff = outps[nindex] * self.brain.nconn[nindex][ilink];
                        if self.brain.xsign[ilink] > 0 {
                            //positives
                            memb[nindex] += stuff;
                        }
                        if self.brain.xsign[ilink] <= 0 {
                            //negatives
                            if stuff <= memb[nindex] {
                                memb[nindex] -= stuff;
                            } else {
                                memb[nindex] = 0;
                            } //end of if on sign less than 0
                        } //end of if on <=0
                    } //end of loop on ilink
                } //end of not refactory

                //fire or not !
                let r: i32 = random_range(-2, 3);
                let thres: i32 = 3;
                if memb[nindex] as i32 >= (thres + r) {
                    temp_outps[nindex] = 1;
                    memb[nindex] = 0;
                } else {
                    temp_outps[nindex] = 0;
                }
                //leakage
                if memb[nindex] >= leaking_constant {
                    memb[nindex] -= leaking_constant;
                }
            } //end of pass through all neurons

            outps = temp_outps.clone();
            fire_knt[0] += temp_outps[0] + temp_outps[1];
            fire_knt[1] += temp_outps[3] + temp_outps[4];
            fire_knt[2] += temp_outps[6] + temp_outps[7];

            temp_outps = [0; constants::NUM_NEURONS];
        } //end of settling_time loop

        let mut min_index = 1; //go straight if nothing happens;
        let mut min_value = 99;

        //choose a direction based on sensor.
        for i in 0..constants::NUM_SENSORS {
            if fire_knt[i] <= min_value {
                min_value = fire_knt[i];
                min_index = i;
            }
            fire_knt[i] = 0;
        }

        let mut new_angle_index = self.angle_index;
        if min_index == 0 {
            new_angle_index = new_angle_index + 1;
            if new_angle_index > constants::NUM_ANGLES - 1 {
                new_angle_index = 0;
            }
        }
        if min_index == 2 {
            if new_angle_index > 0 {
                new_angle_index = new_angle_index - 1;
            } else {
                new_angle_index = constants::NUM_ANGLES - 1;
            }
        }

        self.old_angle_index = self.angle_index;
        self.angle_index = new_angle_index;
    } //end of think

    pub fn update_mover(&mut self) {
        let accel_x = self.multiplier as f32 * constants::ANGLES_DX[self.angle_index];
        let accel_y = self.multiplier as f32 * constants::ANGLES_DY[self.angle_index];

        self.velocity_x = accel_x;
        self.velocity_y = accel_y;
        self.position.x += self.velocity_x;
        self.position.y += self.velocity_y;

        //Get rewarded for going more or less straight
        if self.angle_index == self.old_angle_index {
            self.brain.fitness += 1.0;
        }
    } //end of update function

    pub fn display(&self, draw: &Draw) {
        // Display circle at x position
        if self.isdead == 0 {
            draw.rect()
                .xy(self.position)
                .w_h(5.0, 5.0)
                .rgba(0.8, 0.3, 0.3, 0.5)
                .stroke(RED)
                .stroke_weight(2.0);
        }

        if self.isdead == 1 {
            draw.rect()
                .xy(self.position)
                .w_h(5.0, 5.0)
                .rgba(0.1, 0.3, 0.3, 0.5)
                .stroke(BLACK)
                .stroke_weight(2.0);
        }

        for isensor in 0..constants::NUM_SENSORS {
            let end_point = pt2(self.sensor_data[isensor][0], self.sensor_data[isensor][1]);
            draw.line()
                .start(self.position)
                //.end(self.sensor_data[isensor][0],self.sensor_data[isensor][1])
                .end(end_point)
                .weight(2.00)
                .color(BLACK);
        }

        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(100.0, 100.0)
            .rgba(0.1, 0.3, 0.3, 0.5)
            .stroke(BLACK)
            .stroke_weight(2.0);
    }
    pub fn build_sensor_data_vector(&mut self) {
        self.sensor_data_vector = [0u8; constants::NUM_NEURONS];
        for i in 0..constants::NUM_SENSORS {
            let dx = self.sensor_data[i][0] - self.position.x;
            let dy = self.sensor_data[i][1] - self.position.y;
            let mut dist = dx.hypot(dy);
            if dist > constants::SENSOR_LENGTH {
                dist = constants::SENSOR_LENGTH;
            }
            //from paper scale is based on reflected light strength
            //so more reflection closer to wall
            //
            let junkf = 1.0 - dist / constants::SENSOR_LENGTH;

            if junkf >= 0.80 {
                if i == 0 {
                    self.sensor_data_vector[0] = 1;
                    self.sensor_data_vector[1] = 1;
                    self.sensor_data_vector[2] = 1;
                }
                if i == 1 {
                    self.sensor_data_vector[3] = 1;
                    self.sensor_data_vector[4] = 1;
                }
                if i == 2 {
                    self.sensor_data_vector[5] = 1;
                    self.sensor_data_vector[6] = 1;
                    self.sensor_data_vector[7] = 1;
                }
            } //end of if on .80

            if junkf >= 0.50 && junkf < 0.80 {
                if i == 0 {
                    self.sensor_data_vector[0] = 0;
                    self.sensor_data_vector[1] = 1;
                    self.sensor_data_vector[2] = 1;
                }
                if i == 1 {
                    self.sensor_data_vector[3] = 1;
                    self.sensor_data_vector[4] = 1;
                }
                if i == 2 {
                    self.sensor_data_vector[5] = 0;
                    self.sensor_data_vector[6] = 1;
                    self.sensor_data_vector[7] = 1;
                }
            } //end of if on .50

            if junkf >= 0.25 && junkf < 0.50 {
                if i == 0 {
                    self.sensor_data_vector[0] = 0;
                    self.sensor_data_vector[1] = 0;
                    self.sensor_data_vector[2] = 1;
                }
                if i == 1 {
                    self.sensor_data_vector[3] = 0;
                    self.sensor_data_vector[4] = 1;
                }
                if i == 2 {
                    self.sensor_data_vector[5] = 0;
                    self.sensor_data_vector[6] = 0;
                    self.sensor_data_vector[7] = 1;
                }
            } //end of if 25 - 50
              //otherwise take defaults of zeros
        } //end of loop on sensors
    } //end of build_vector

    pub fn check_dead(&mut self, rect: Rect) {
        if self.position.x > rect.right() {
            self.position.x = rect.right();
            self.isdead = 1;
            println!("DEAD ON WALL");
            return;
        }

        if self.position.x < rect.left() {
            self.position.x = rect.left();
            self.isdead = 1;
            println!("DEAD ON WALL");
            return;
        }

        if self.position.y < rect.bottom() {
            self.position.y = rect.bottom();
            self.isdead = 1;
            println!("DEAD ON WALL");
            return;
        }

        if self.position.y > rect.top() {
            self.position.y = rect.top();
            self.isdead = 1;
            println!("DEAD ON WALL");
            return;
        }

        //check against middle box
        if self.position.x >= -50.0
            && self.position.x <= 50.0
            && self.position.y <= 50.0
            && self.position.y >= -50.0
        {
            self.isdead = 1;
            println!("DEAD ON ROCKS");
            return;
        }
    } //end of check_dead

    pub fn check_collisions(&mut self, xpos: f32, ypos: f32, rect: Rect) -> u32 {
        //this is only for sensors... no dying

        if xpos > rect.right() {
            return 1;
        }

        if xpos < rect.left() {
            return 1;
        }

        if ypos < rect.bottom() {
            return 1;
        }

        if ypos > rect.top() {
            return 1;
        }

        //check against middle box
        //hardwired at the moment.
        if xpos >= -50.0 && xpos <= 50.0 && ypos <= 50.0 && ypos >= -50.0 {
            return 1;
        }
        return 0;
    } //end of check_dead

    pub fn get_sensor_data(&mut self, rect: Rect) {
        //this looks stupid. Testing for intersection
        //of lines would be better...maybe
        //
        let _knt = 0;
        for isensor in 0..constants::NUM_SENSORS {
            let mut sensor_ai: i32 = 99;
            //let _dist = 0.0;

            if isensor == 0 {
                sensor_ai = self.angle_index as i32 + 1;
            }
            if isensor == 1 {
                sensor_ai = self.angle_index as i32;
            }
            if isensor == 2 {
                sensor_ai = self.angle_index as i32 - 1;
            }
            //adjust for outside range
            if sensor_ai > constants::NUM_ANGLES as i32 - 1 {
                sensor_ai = sensor_ai % constants::NUM_ANGLES as i32;
            }
            if sensor_ai < 0 {
                sensor_ai = constants::NUM_ANGLES as i32 - 1;
            }
            let mut xpos = self.position.x;
            let mut ypos = self.position.y;
            for _step in 0..constants::SENSOR_LENGTH as u32 {
                xpos = xpos + constants::ANGLES_DX[sensor_ai as usize];
                ypos = ypos + constants::ANGLES_DY[sensor_ai as usize];
                let hit = self.check_collisions(xpos, ypos, rect);
                if hit == 1 {
                    break;
                }
                let fdx = xpos - self.position.x;
                let fdy = ypos - self.position.y;
                if fdx.hypot(fdy) > constants::SENSOR_LENGTH {
                    break;
                }
            } //end of loop on step

            self.sensor_data[isensor][0] = xpos;
            self.sensor_data[isensor][1] = ypos;
        } //end of sensor loop
    } //end of get_sensor_data

    pub fn reset_mover(&mut self, width:f32,height:f32)  {
        self.brain.fitness = 0.0;
        self.isdead = 0;
        let start_x = width / 2.0 - constants::SENSOR_LENGTH + 10.0;
        let start_y = (height / 2.0) - constants::SENSOR_LENGTH;
        self.position = pt2(start_x, start_y);
        self.angle_index = random_range(0, constants::NUM_ANGLES);
        self.multiplier = 1;
        self.velocity_x = constants::ANGLES_DX[self.angle_index];
        self.velocity_y = constants::ANGLES_DY[self.angle_index];
    }
    pub fn mutate(&mut self) {

        //start mutations here ...

        let mutidx = random_range(0, constants::NUM_NEURONS as usize);
        if self.brain.xsign[mutidx] == 0 {
            self.brain.xsign[mutidx] = 1;
        } else {
            self.brain.xsign[mutidx] = 0;
        }

        let mutidx = random_range(0, constants::NUM_NEURONS as usize);
        let ilink = random_range(0, constants::NUM_NEURONS);
        if self.brain.nconn[mutidx][ilink] == 0 {
            self.brain.nconn[mutidx][ilink] = 1;
        } else {
            self.brain.nconn[mutidx][ilink] = 0;
        }

        //might not want to do this.
        // lets keep all input signals
        /*
        let mutidx = random_range(0,constants::NUM_NEURONS as usize);
        let ilink = random_range(0,constants::NUM_NEURONS);
        if self.brain.iconn[mutidx][ilink] == 0 {
            self.brain.iconn[mutidx][ilink] = 1;
        } else {
            self.brain.iconn[mutidx][ilink] = 0;
        }
        */

    } //end of mutate

} //end of impl




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
