use rand::Rng;

#[derive(Debug, Copy, Clone)]

pub enum Pitch {
    Ball,
    Strike,
    Hit,
    Out,
    //Foul,
}

impl Pitch {
    pub fn random() -> Self {
        
        if rand::thread_rng().gen_range(0..3) == 0 {
            Pitch::Ball
        }
        else if rand::thread_rng().gen_range(0..3) == 1 {
            Pitch::Strike
        }
        else if rand::thread_rng().gen_range(0..3) == 2 {
            Pitch::Hit
        } else {
            Pitch::Out
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
    let mut hits = 0;
    let mut outs = 0;
    //let mut fouls = 0;

      

    while balls < 4 && strikes < 3 && outs == 0 && hits == 0 && at_bat < 27 {
        // println!("At Bat = {}", at_bat);
        let pitch = Pitch::random();
        match pitch {
            Pitch::Ball => balls += 1,
            Pitch::Strike => strikes += 1,
            Pitch::Hit => hits += 1,
            Pitch::Out => outs += 1,
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

    if hits > 0 {
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
