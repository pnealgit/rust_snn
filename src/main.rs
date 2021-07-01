//
//Some of the ideas are from :
//Evolution of Spiking Neural Circuits in Autonomous Mobile Robots
//Dario Floreano, Yann Epars, Jean-Christophe Zufferey, Claudio Mattiussi
//INTERNATIONAL JOURNAL OF INTELLIGENT SYSTEMS, VOL. XX, 2005
//(The SNN paper)
//
//Spiking Nerural Networks with a ga are a lot cooler than
//the usual Rumelhart-McClelland feed-forward neural networks.
//It is a specific case of an extreme learning machine
//
//
//Some of the Rust code with Nannou is from
//https://github.com/nannou-org/nannou
//

//Implementation by Phillip R. Neal
//philn1984@gmail.com
//MacAir running macOS Catalina Verison 10.15.7
//
//For this project, the goal is for the
//rover to learn to keep away from the walls and
//the square in the middle.
//
//fitness is the number of times per lifetime the rover
//does not change direction....

use nannou::prelude::*;

const WIDTH: f32 = 400.0; //width and height of
const HEIGHT: f32 = 400.0; //screen
const NUM_NEURONS: usize = 8; //should be powers of 2
const NUM_BRAINS: usize = 10; //number of brains  -- in ga talk population
const NUM_ANGLES: usize = 8; //rover has 8 possible directions it can travel
                             //e,ne,n,nw,w,sw,s,se -- kind of like the unit circle in trig
const ANGLES_DX: [f32; 8] = [1.0, 1.0, 0.0, -1.0, -1.0, -1.0, 0.0, 1.0];
const ANGLES_DY: [f32; 8] = [0.0, 1.0, 1.0, 1.0, 0.0, -1.0, -1.0, -1.0];
const NUM_SENSORS: usize = 3; //number of antennae
const SENSOR_LENGTH: f32 = 60.0; //length of an antenna
const MAX_LOOP_KNT: usize = 2000; //can't let them live forever

fn main() {
    //basic spell invocation for nannou
    nannou::app(model).update(update).run();
}

struct Model {
    //this is the data and function that will be alway available
    mover: Mover,
    loop_knt: usize,
    num_epochs: usize,
}

//this whole structure can be implemented as bits.
//thus the power of 2 requirement
//
#[derive(Clone, Debug)]
struct Brain {
    fitness: f32,
    xsign: [u8; NUM_NEURONS as usize],
    iconn: [[u8; NUM_NEURONS as usize]; NUM_NEURONS as usize],
    nconn: [[u8; NUM_NEURONS as usize]; NUM_NEURONS as usize],
}

impl Brain {
    fn new() -> Self {
        let fitness = 0.0;
        let mut xsign = [0; NUM_NEURONS];
        for ix in 0..NUM_NEURONS {
            xsign[ix] = random_range(0, 2) as u8;
        }
        let iconn = [[1; NUM_NEURONS as usize]; NUM_NEURONS as usize];
        let mut nconn = [[0; NUM_NEURONS as usize]; NUM_NEURONS as usize];
        for ix in 0..NUM_NEURONS {
            for iy in 0..NUM_NEURONS {
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

#[derive(Clone, Debug)]
struct Mover {
    position: Point2,
    angle_index: usize,
    old_angle_index: usize,
    velocity_x: f32,
    velocity_y: f32,
    multiplier: i32,
    sensor_data: [[f32; 2]; NUM_SENSORS],
    sensor_data_vector: [u8; NUM_NEURONS],
    isdead: i32,
    brain_index: usize,
    brain: Brain,
    brains: Vec<Brain>,
}

impl Mover {
    fn new(x: f32, y: f32) -> Self {
        let position = pt2(x, y);
        let angle_index = random_range(0, NUM_ANGLES);
        let old_angle_index = random_range(0, NUM_ANGLES);
        let velocity_x = ANGLES_DX[angle_index];
        let velocity_y = ANGLES_DY[angle_index];
        let multiplier = 1;
        let sensor_data = [[0.0; 2]; NUM_SENSORS];
        let sensor_data_vector = [0u8; NUM_NEURONS];
        let isdead = 0;
        let brain_index = random_range(0, NUM_BRAINS);
        let mut brains = Vec::new();
        for _ in 0..NUM_BRAINS {
            brains.push(Brain::new());
        }
        let brain = Brain::new();
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

    fn think(&mut self) {
        //see paper cited above
        self.build_sensor_data_vector();

        //In most cases , the sensor_data_vector will be all zeros.
        //so add some bias to make something happen.
        //
        let mut knt = 0;
        for ik in 0..NUM_SENSORS as usize {
            knt += self.sensor_data_vector[ik];
        }
        if knt <= 0 {
            let ridx = random_range(0, 8);
            self.sensor_data_vector[ridx] = 1;
        }

        let leaking_constant = 1;
        let mut temp_outps = [0u8; NUM_NEURONS];
        let inps = self.sensor_data_vector.clone();
        let mut memb = [0u8; NUM_NEURONS];
        let mut outps = [0u8; NUM_NEURONS];
        let mut fire_knt = [0; NUM_SENSORS];
        let settling_time = 20; //loop through settling_time times

        for _epoch in 0..settling_time {
            for nindex in 0..NUM_NEURONS {
                memb[nindex] = 0;
                if outps[nindex] == 0 {
                    //not in refactory state
                    for ilink in 0..NUM_NEURONS {
                        memb[nindex] += inps[nindex] * self.brain.iconn[nindex][ilink];
                    } //end of loop on ilink
                      //count from other neurons with positive sign
                    for ilink in 0..NUM_NEURONS as usize {
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

            temp_outps = [0; NUM_NEURONS];
        } //end of settling_time loop

        let mut min_index = 1; //go straight if nothing happens;
        let mut min_value = 99;

        //choose a direction based on sensor.
        for i in 0..NUM_SENSORS {
            if fire_knt[i] <= min_value {
                min_value = fire_knt[i];
                min_index = i;
            }
            fire_knt[i] = 0;
        }

        let mut new_angle_index = self.angle_index;
        if min_index == 0 {
            new_angle_index = new_angle_index + 1;
            if new_angle_index > NUM_ANGLES - 1 {
                new_angle_index = 0;
            }
        }
        if min_index == 2 {
            if new_angle_index > 0 {
                new_angle_index = new_angle_index - 1;
            } else {
                new_angle_index = NUM_ANGLES - 1;
            }
        }

        self.old_angle_index = self.angle_index;
        self.angle_index = new_angle_index;
    } //end of think

    fn update_mover(&mut self) {
        let accel_x = self.multiplier as f32 * ANGLES_DX[self.angle_index];
        let accel_y = self.multiplier as f32 * ANGLES_DY[self.angle_index];

        self.velocity_x = accel_x;
        self.velocity_y = accel_y;
        self.position.x += self.velocity_x;
        self.position.y += self.velocity_y;

        //Get rewarded for going more or less straight
        if self.angle_index == self.old_angle_index {
            self.brain.fitness += 1.0;
        }
    } //end of update function

    fn display(&self, draw: &Draw) {
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

        for isensor in 0..NUM_SENSORS {
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
    fn build_sensor_data_vector(&mut self) {
        self.sensor_data_vector = [0u8; NUM_NEURONS];
        for i in 0..NUM_SENSORS {
            let dx = self.sensor_data[i][0] - self.position.x;
            let dy = self.sensor_data[i][1] - self.position.y;
            let mut dist = dx.hypot(dy);
            if dist > SENSOR_LENGTH {
                dist = SENSOR_LENGTH;
            }
            //from paper scale is based on reflected light strength
            //so more reflection closer to wall
            //
            let junkf = 1.0 - dist / SENSOR_LENGTH;

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

    fn check_dead(&mut self, rect: Rect) {
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

    fn check_collisions(&mut self, xpos: f32, ypos: f32, rect: Rect) -> u32 {
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

    fn get_sensor_data(&mut self, rect: Rect) {
        //this looks stupid. Testing for intersection
        //of lines would be better...maybe
        //
        let _knt = 0;
        for isensor in 0..NUM_SENSORS {
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
            if sensor_ai > NUM_ANGLES as i32 - 1 {
                sensor_ai = sensor_ai % NUM_ANGLES as i32;
            }
            if sensor_ai < 0 {
                sensor_ai = NUM_ANGLES as i32 - 1;
            }
            let mut xpos = self.position.x;
            let mut ypos = self.position.y;
            for _step in 0..SENSOR_LENGTH as u32 {
                xpos = xpos + ANGLES_DX[sensor_ai as usize];
                ypos = ypos + ANGLES_DY[sensor_ai as usize];
                let hit = self.check_collisions(xpos, ypos, rect);
                if hit == 1 {
                    break;
                }
                let fdx = xpos - self.position.x;
                let fdy = ypos - self.position.y;
                if fdx.hypot(fdy) > SENSOR_LENGTH {
                    break;
                }
            } //end of loop on step

            self.sensor_data[isensor][0] = xpos;
            self.sensor_data[isensor][1] = ypos;
        } //end of sensor loop
    } //end of get_sensor_data
} //end of impl

fn model(app: &App) -> Model {
    let rect = Rect::from_w_h(WIDTH, HEIGHT);
    app.new_window()
        .size(rect.w() as u32, rect.h() as u32)
        .view(view)
        .build()
        .unwrap();

    let start_x = WIDTH / 2.0 - SENSOR_LENGTH + 10.0;
    let start_y = (HEIGHT / 2.0) - SENSOR_LENGTH;

    let mover = Mover::new(start_x, start_y);
    let loop_knt = 0;
    let num_epochs = 0;
    Model {
        mover,
        loop_knt,
        num_epochs,
    }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    m.mover.check_dead(app.window_rect());
    if m.mover.isdead == 0 {
        m.mover.get_sensor_data(app.window_rect());
        m.mover.think();
        m.mover.update_mover();
        m.loop_knt += 1;
    }

    if m.mover.isdead == 1 || m.loop_knt > MAX_LOOP_KNT {
        //do mutations and updates here
        //
        println!("END OF LIFE FITNESS WAS: {}", m.mover.brain.fitness);
        //get fitnesses for the population before choosing
        //who to breed/mutate.
        //
        if m.num_epochs < NUM_BRAINS {
            //store old results
            m.mover.brains[m.mover.brain_index] = m.mover.brain.clone();
            //get new brain
            m.mover.brain_index = m.num_epochs;
            m.mover.brain = m.mover.brains[m.mover.brain_index].clone();
        } else {
            //don't want to do this sort but it makes things cleaner.
            //use the technique in the paper next time.

            m.mover
                .brains
                .sort_by(|d2, d1| d1.fitness.partial_cmp(&d2.fitness).unwrap());

            let mut sum_fit = 0.0;
            for ix in 0..NUM_BRAINS {
                println!("IX: {} FITNESS: {} ", ix, m.mover.brains[ix].fitness as u32);
                sum_fit += m.mover.brains[ix].fitness;
            }

            //pick a new brain
            //Since already sorted,
            //pick a random number between 0.0 and 1.0
            //run down the sorted fitnesses until the sum
            //of the fitnesses is greater than the random number
            //
            let mut this_fit = 0.0;
            let goal = random_range(0.0, 1.0);

            let mut goal_index = 0;
            for ix in 0..NUM_BRAINS {
                this_fit += m.mover.brains[ix].fitness / sum_fit;
                if this_fit > goal {
                    goal_index = ix;
                    break;
                }
            }
            //before replacing brain , see if it should be stored in the
            //population.
            if m.mover.brain.fitness >= m.mover.brains[NUM_BRAINS - 1].fitness {
                m.mover.brains[NUM_BRAINS - 1] = m.mover.brain.clone();
            }
            m.mover.brain_index = goal_index;
            m.mover.brain = m.mover.brains[m.mover.brain_index].clone();
        } //end of if-else

        m.loop_knt = 0;

        //start mutations here ...

        let mutidx = random_range(0, NUM_NEURONS as usize);
        if m.mover.brain.xsign[mutidx] == 0 {
            m.mover.brain.xsign[mutidx] = 1;
        } else {
            m.mover.brain.xsign[mutidx] = 0;
        }

        let mutidx = random_range(0, NUM_NEURONS as usize);
        let ilink = random_range(0, NUM_NEURONS);
        if m.mover.brain.nconn[mutidx][ilink] == 0 {
            m.mover.brain.nconn[mutidx][ilink] = 1;
        } else {
            m.mover.brain.nconn[mutidx][ilink] = 0;
        }

        //might not want to do this.
        // lets keep all input signals
        /*
        let mutidx = random_range(0,NUM_NEURONS as usize);
        let ilink = random_range(0,NUM_NEURONS);
        if m.mover.brain.iconn[mutidx][ilink] == 0 {
            m.mover.brain.iconn[mutidx][ilink] = 1;
        } else {
            m.mover.brain.iconn[mutidx][ilink] = 0;
        }
        */

        //reset  mover
        m.mover.brain.fitness = 0.0;
        m.mover.isdead = 0;
        let start_x = WIDTH / 2.0 - SENSOR_LENGTH + 10.0;
        let start_y = (HEIGHT / 2.0) - SENSOR_LENGTH;
        m.mover.position = pt2(start_x, start_y);
        m.mover.angle_index = random_range(0, NUM_ANGLES);
        m.mover.multiplier = 1;
        m.mover.velocity_x = ANGLES_DX[m.mover.angle_index];
        m.mover.velocity_y = ANGLES_DY[m.mover.angle_index];
        m.num_epochs += 1;
        println!("NUM EPOCHS: {} ", m.num_epochs);
    } //end of if on dead or frames done
} //end of update

fn view(app: &App, m: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);

    m.mover.display(&draw);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
