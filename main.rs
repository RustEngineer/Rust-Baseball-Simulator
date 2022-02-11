use rand::Rng;

#[derive(Debug, Copy, Clone)]

pub enum PitchResult {
    Ball,
    Strike,
    Out,
    Single,
    Double,
    Triple,
    HomeRun,

    //Foul,
}


impl PitchResult {
    pub fn random() -> Self {
        
        if rand::thread_rng().gen_range(0..6) == 0 {
            PitchResult::Ball
        }
        else if rand::thread_rng().gen_range(0..6) == 1 {
            PitchResult::Strike
        }
        else if rand::thread_rng().gen_range(0..6) == 2 {
            PitchResult::Single
             }
        else if rand::thread_rng().gen_range(0..6) == 3 {
            PitchResult::Double
        }
        else if rand::thread_rng().gen_range(0..6) == 4 {
            PitchResult::Triple
             }
        else if rand::thread_rng().gen_range(0..6) == 5 {
            PitchResult::HomeRun
        } else {
            PitchResult::Out
        }
        //else {
           // Pitch::Foul
        //}
        

    }
}

fn main() {
    
    println!("*****Game Start*****");
    let mut at_bat = 0;
    'counting_up: loop {
    let mut balls = 0;
    let mut strikes = 0;
    let mut single = 0;
    let mut double = 0;
    let mut triple = 0;
    let mut homerun = 0;
    let mut outs = 0;
    //let mut fouls = 0;

      

    while balls < 4 && strikes < 3 && outs == 0 && single == 0 && double == 0 && triple == 0 && homerun == 0 && at_bat < 27 {
        // println!("At Bat = {}", at_bat);
        let pitch = PitchResult::random();
        match pitch {
            PitchResult::Ball => balls += 1,
            PitchResult::Strike => strikes += 1,
            PitchResult::Single => single += 1,
            PitchResult::Double => double += 1,
            PitchResult::Triple => triple += 1,
            PitchResult::HomeRun => homerun += 1,
            PitchResult::Out => outs += 1,
            //Pitch::Foul => fouls += 1,

        }
        
        println!("The pitch is a: {:?}", pitch);

    if outs > 0 {
        
        at_bat += 1;
        println!("Total Outs = {}", at_bat);
        if at_bat < 27 {
        println!("*****New Batter*****");
        }
        continue
    }

    if single > 0 {
        println!("*****New Batter*****");
        continue
    }

     if double > 0 {
        println!("*****New Batter*****");
        continue
    }

    if triple > 0 {
        println!("*****New Batter*****");
        continue
    }

     if homerun > 0 {
        println!("*****New Batter*****");
        continue
    }


    if strikes == 3 {
        println!("Strike Out!");
        at_bat += 1;
        println!("Total Outs = {}", at_bat);
        println!("*****New Batter*****");
        continue
    }

    if balls == 4 {
        println!("Batter Walked!");
        println!("*****New Batter*****");
        continue
    }

    if balls > 0 || balls < 4 || strikes > 0 || strikes < 3 {
        println!("The count is: {} - {}", balls, strikes);
    }

     if balls == 3 && strikes == 2 {
        println!("Full Count!");
    }
 
     if balls == 3 && strikes == 0 {
        println!("Hitter's Count!");
    }

     if balls == 0 && strikes == 2 {
        println!("Pitcher's Count!");
    }

    }      
    if at_bat == 27 {
        println!("That's the Ballgame Folks!");
        break 'counting_up;
    }

}
    
}
