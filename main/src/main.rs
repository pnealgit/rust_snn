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
extern crate brain;
extern crate mover;

use mover::*;



const WIDTH: f32 = 400.0; //width and height of
const HEIGHT: f32 = 400.0; //screen
//const NUM_NEURONS: usize = 8; //should be powers of 2
const NUM_BRAINS: usize = 10; //number of brains  -- in ga talk population
//const NUM_ANGLES: usize = 8; //rover has 8 possible directions it can travel
                             //e,ne,n,nw,w,sw,s,se -- kind of like the unit circle in trig
//const ANGLES_DX: [f32; 8] = [1.0, 1.0, 0.0, -1.0, -1.0, -1.0, 0.0, 1.0];
//const ANGLES_DY: [f32; 8] = [0.0, 1.0, 1.0, 1.0, 0.0, -1.0, -1.0, -1.0];
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
        m.mover.mutate();

        m.mover.reset_mover(WIDTH,HEIGHT);

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
