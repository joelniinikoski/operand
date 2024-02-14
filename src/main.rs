use operand::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "operand".to_owned(),
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut startnum = 0;
    let mut endnum = get_endnum();
    let mut lastnum = 0;

    let mut moves = 0;
    let mut lastmoves = 0;

    let mut timer = 0.0;
    let mut lasttime = 0.0;

    //left hand controls number
    let mut onum: &str;
    //right hand controls operand
    let mut oper: &str;

    //lock inputs after operation if buttons not released
    let mut lock = false;
    let mut spacelock = false;
    //locked inputs, can release input lock
    let mut lockpressed = vec![KeyCode::Space, KeyCode::Space];

    loop {
        //logic phase
        timer += get_frame_time();

        //check if one locked button has been released
        if lock {
            for k in lockpressed.iter() {
                if is_key_released(k.to_owned()) {
                    lock = false;
                }
            }
        }
        
        if !lock {
            //set operating number active as 2,3,5 or 7; 0 means inactive.
            if is_key_down(KeyCode::W) {
                onum = "7";
                lockpressed[0] = KeyCode::W;
            } else if is_key_down(KeyCode::A) {
                onum = "2";
                lockpressed[0] = KeyCode::A;
            } else if is_key_down(KeyCode::S) {
                onum = "3";
                lockpressed[0] = KeyCode::S;
            } else if is_key_down(KeyCode::D) {
                onum = "5";
                lockpressed[0] = KeyCode::D;
            } else {
                onum = "0";
            }

            //set active operand as *, /, + or -; "." means inactive.
            if is_key_down(KeyCode::J) {
                oper = "+";
                lockpressed[1] = KeyCode::J;
            } else if is_key_down(KeyCode::I) {
                oper = "*";
                lockpressed[1] = KeyCode::I;
            } else if is_key_down(KeyCode::K) {
                oper = "/";
                lockpressed[1] = KeyCode::K;
            } else if is_key_down(KeyCode::L) {
                oper = "-";
                lockpressed[1] = KeyCode::L;
            } else {
                oper = ".";
            }

            //if both hands active => do calculation
            if onum != "0" && oper != "." {
                match oper {
                    "*" => startnum *= onum.parse::<i32>().unwrap(),
                    "/" => startnum /= onum.parse::<i32>().unwrap(),
                    "+" => startnum += onum.parse::<i32>().unwrap(),
                    "-" => startnum -= onum.parse::<i32>().unwrap(),
                    _ => ()
                }
                lock = true;
                moves += 1;
            }
        }

        //remove spacebar lock
        if is_key_released(KeyCode::Space) {
            spacelock = false;
        }
        //spacebar adds 50% to number
        if !spacelock {
            if is_key_down(KeyCode::Space) {
                spacelock = true;
                moves += 1;
                startnum = startnum + startnum/2;
            }
        }

        if startnum > 30000 {
            startnum = 30000;
        }

        //next number
        if startnum == endnum {
            lastnum = endnum;
            lasttime = timer;
            timer = 0.0;
            lastmoves = moves;
            moves = 0;
            endnum = get_endnum();
            startnum = 0;
        }

        //====================================================================
        //drawing phase
        clear_background(BLACK);
        let textstart = format!("{}", startnum);
        let textend = format!("{}", endnum);
        let textmoves = format!("Your moves: {}", moves);
        let textlastmoves = format!("Moves last problem: {}", lastmoves);
        let texttimer = format!("Time: {:.2}", timer);
        let textlasttime = format!("Time last problem: {:.2}", lasttime);
        let textlastnum = format!("Last problem: {}", lastnum);
        if lastmoves != 0 {
            draw_text_centered(&textlastmoves, screen_width()/2., screen_height()-60.,20.0, YELLOW, None);
        }
        if lasttime != 0.0 {
            draw_text_centered(&textlasttime, screen_width()/2., screen_height()-30.,20.0, YELLOW, None);
        }
        if lastnum != 0 {
            draw_text_centered(&textlastnum, screen_width()/2., screen_height()/2.+screen_height()/4.,40.0, GREEN, None);
        }
        draw_text_centered(&textstart, screen_width()/2., screen_height()/2.,40.0, WHITE, None);
        draw_text_centered(&textend, screen_width()/2., screen_height()/2.-30.,25.0, WHITE, None);
        draw_text_centered(&textmoves, screen_width()/2., screen_height()/2.+30.,25.0, WHITE, None);
        draw_text_centered(&texttimer, screen_width()/2., screen_height()/2.+60.,25.0, RED, None);
        next_frame().await
    }
}
