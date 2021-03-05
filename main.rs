mod simulation;
use rand::{Rng, thread_rng};

use crate::simulation::Sim;

fn main() {
    let mut score: f32 = 0.0;
    let mut win_rate :f32 = 0.0;
    let mut bowl_one =  simulation::FBowl::default();
    let mut bowl_two =  simulation::SBowl::default();
    let iterations = 10000000;

    bowl_one.new();
    bowl_two.new();

 

    for _i in 0..iterations {

        if pick(bowl_one,bowl_two) == 0 {
            score += 0.0;
        }
        else {
            score +=1.0;
        }


    }

    win_rate = (score/iterations as f32)*100.0;

    print!("{}%",win_rate)
}

pub fn pick(mut f_bowl:  simulation::FBowl , mut  s_bowl: simulation::SBowl) -> i32{
    let mut rng = rand::thread_rng();

    let n1:i32 = rng.gen_range(0..2);
    
    match n1 {
        1 => return f_bowl.pop() , 
        0 => return s_bowl.pop(),
        x => panic!("Unexpected number {:?}", x) // The borrow checker & compiler guides me through the valley of death.
    } 
    
}
