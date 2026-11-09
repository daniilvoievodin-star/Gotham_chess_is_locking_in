use rand::RngExt;


pub fn Combat(opponent:String){
    
    let mut Gotham_heath = 2300;
    let mut DA_ROOK = 100;
    let mut Caro_Can = 20;
    let mut Trompowski = 30;
    let mut Milking_Magnus = 45 ;
    let mut Hikaru_heath = 2800;
    let mut takes_takes_takes = 50;
    let mut I_literally_dont_care = 20;
    let mut I_will_beat_you_blindfolded = 30;
    let mut Domain_expantion_blitz_world_champion = 40;
    let mut Magnus_heath = 2800;
    let mut Perfect_calculation = 60;
    let mut twenty_world_cups = 30;
    let mut Positional_sification = 40;
    let mut Magnus_Domain = 35;
    let mut Martin_heath = 250;
    let mut I_just_here_for_fun = 10;
    let mut oh_no_my_queen = -50;
    let mut You_are_pretty_good_do_you_wanna_coach_my_kids = 5;
    let mut I_love_chess = 25;
    let mut Tal_heath = 2700;
    let mut Giant_cigarrete = 45;
    let mut aggresive_playstle = 30;
    let mut fear = 60;
    let mut sacrifice = 70;
    let mut Capablanca_heath = 2870;
    let mut chess_fundumentals = 60;
    let mut Positional_sification = 40;
    let mut slow_but_steady = 50;
    
    let mut Lasker_heath = 2720;
    let mut Greek_gift = 80;
    let mut the_longest_world_champion=50;
    
    let mut aggresive_playstyle=40;
    let mut Bobby_Fisher_heath = 2780;
    let mut I_hate_chess =30;
    let mut aggresive_playstle = 30;
    
    let mut aura = 40;
    let mut rng = rand::rng();
    let [_hikaru,_magnus,_martin,_tal,_capablanca,_lasker,_bobby_fisher]= ["Hikaru".to_string(),"Magnus".to_string(),"Martin".to_string(),"Tal".to_string(),"Capablanca".to_string(),"Lasker".to_string(),"Bobby_Fisher".to_string()];
    
    
    
    loop{
        let roll: i32 = rng.random_range(1..=2);
        
        let mut _attack  = String::new();
        if opponent == _hikaru{ 
            
            
        
                if roll== 1{

                    
                    println!("Your turn to attack.");
                    crate::stats::gothamches::Da_rook::Gotham_stats();

                    let attack = choose_attack();
                    if attack == 1{
                        Hikaru_heath -= DA_ROOK;
                        println!("And then he sacrifices THE ROOOOOK!");
                    }else if attack == 2{
                        Hikaru_heath -= Trompowski;
                        println!("Gotham uses Tromposki one of his favorite openigs");
                    }else if attack == 3{
                        Hikaru_heath-=Caro_Can;
                        println!("Gotham uses Caro-Can one of his favorite openigs");
                    }else if attack ==4{
                        Hikaru_heath-=Milking_Magnus;
                        println!("Gotham uses forbidden techique and uses Magnus for content");
                    }else{
                        println!("Wrong. Chose another one");
                    }
                }
                else{
                    let enemy_attack: i32 = rng.random_range(1..=4);
                    println!("Hikarus turn to attack.");
                    if enemy_attack == 1 {
                        Gotham_heath -= takes_takes_takes ;
                        println!("Hikaru uses secret technique and sees 50 moves into the future");
                    }else if enemy_attack == 2{
                        Gotham_heath -=I_literally_dont_care  ;
                        println!("Hikaru literally doesnt care");
                    }else if enemy_attack== 3{
                        Gotham_heath-=I_will_beat_you_blindfolded;
                        println!("Hikaru uses his incredible senses and memory to spot your pieses");
                    }else{
                        Gotham_heath-=Domain_expantion_blitz_world_champion;
                        println!("Hikaru opens his domain")
                    }
                    
                        
                    

                    
                }if Hikaru_heath <= 0{
                println!("You win the battle.");
                break;
                }else if Gotham_heath<= 0{
                
                        println!("Hikaru wins");
                        break;
                }
            println!("Hikaru HP: {}", Hikaru_heath);
            println!("Gotham HP: {}", Gotham_heath);    
        } else if opponent == _magnus{ 
            
            
        
                if roll== 1{

                    
                    println!("Your turn to attack.");
                    crate::stats::gothamches::Da_rook::Gotham_stats();
                    let attack = choose_attack();
                    if attack == 1{
                        Magnus_heath -= DA_ROOK;
                        println!("And then he sacrifices THE ROOOOOK!");
                    }else if attack == 2{
                        Magnus_heath -= Trompowski;
                        println!("Gotham uses Tromposki one of his favorite openigs");
                    }else if attack == 3{
                        Magnus_heath-=Caro_Can;
                        println!("Gotham uses Caro-Can one of his favorite openigs");
                    }else if attack ==4{
                        Magnus_heath-=Milking_Magnus;
                        println!("Gotham uses forbidden techique and uses Magnus for content");
                
                    }else{
                        println!("Wrong. Chose another one");
                    }
                }
                else{
                    let enemy_attack: i32 = rng.random_range(1..=4);
                    println!("Magnuses turn to attack.");
                    if enemy_attack == 1 {
                        Gotham_heath -= Perfect_calculation ;
                        println!("Magnus calculates every possible move ");
                    }else if enemy_attack == 2{
                        Gotham_heath -=twenty_world_cups   ;
                        println!("slows time so you cant beat him");
                    }else if enemy_attack== 3{
                        Gotham_heath-=Positional_sification ;
                        println!("puts you in position where every move is bad");
                    }else{
                        Gotham_heath-=Magnus_Domain;
                        println!("Magnus opens his domain where he dominates you with his endgame skills");
                    }
                    
                        
                    

                    
                }if Magnus_heath <= 0{
                println!("You win the battle.");
                break;
                }else if Gotham_heath<= 0{
                
                        println!("Magnus wins");
                        break;
                }
            println!("Magnus HP: {}", Magnus_heath);
            println!("Gotham HP: {}", Gotham_heath);    
        } else if opponent == _martin{ 
            
            
        
                if roll== 1{

                    
                    println!("Your turn to attack.");
                    crate::stats::gothamches::Da_rook::Gotham_stats();
                    let attack = choose_attack();
                    if attack == 1{
                        Martin_heath -= DA_ROOK;
                        println!("And then he sacrifices THE ROOOOOK!");
                    }else if attack == 2{
                        Martin_heath -= Trompowski;
                        println!("Gotham uses Tromposki one of his favorite openigs");
                    }else if attack == 3{
                        Martin_heath-=Caro_Can;
                        println!("Gotham uses Caro-Can one of his favorite openigs");
                    }else if attack ==4{
                        Martin_heath-=Milking_Magnus;
                        println!("Gotham uses forbidden techique and uses Magnus for content");
                
                    }else{
                        println!("Wrong. Chose another one");
                    }
                }
                else{
                    let enemy_attack: i32 = rng.random_range(1..=4);
                    println!("Martins turn to attack.");
                    if enemy_attack == 1 {
                        Gotham_heath -= I_just_here_for_fun ;
                        println!("Martin is having a fun playing chess with you");
                    }else if enemy_attack == 2{
                        Gotham_heath -=oh_no_my_queen   ;
                        println!("Oh no my queen");
                    }else if enemy_attack== 3{
                        Gotham_heath-=You_are_pretty_good_do_you_wanna_coach_my_kids;
                        println!("You are pretty good maybe you wanna coach my kids?");
                    }else{
                        Gotham_heath-=I_love_chess;
                        println!("Martin just loves chess you cant blame him for it");
                    }
                    
                        
                    

                    
                }if Martin_heath <= 0{
                println!("You win the battle.");
                break;
                }else if Gotham_heath<= 0{
                
                        println!("Martin wins");
                        break;
                }
            println!("Martin HP: {}", Martin_heath);
            println!("Gotham HP: {}", Gotham_heath);    
        } else if opponent == _tal{ 
            
            
        
                if roll== 1{

                    
                    println!("Your turn to attack.");
                    crate::stats::gothamches::Da_rook::Gotham_stats();
                    let attack = choose_attack();
                    if attack == 1{
                        Tal_heath -= DA_ROOK;
                        
                        println!("And then he sacrifices THE ROOOOOK!");
                    }else if attack == 2{
                        Tal_heath -= Trompowski;
                        println!("Gotham uses Tromposki one of his favorite openigs");
                        
                    }else if attack == 3{
                        Tal_heath-=Caro_Can;
                        println!("Gotham uses Caro-Can one of his favorite openigs");
                    }else if attack ==4{
                        Tal_heath-=Milking_Magnus;
                        println!("Gotham uses forbidden techique and uses Magnus for content");
                
                    }else{
                        println!("Wrong. Chose another one");
                    }
                }
                else{
                    let enemy_attack: i32 = rng.random_range(1..=4);
                    println!("Tals turn to attack.");
                    if enemy_attack == 1 {
                        Gotham_heath -= Giant_cigarrete  ;
                        println!("The smoke from Tals ciggare makes you feel unwell");
                    }else if enemy_attack == 2{
                        Gotham_heath -=aggresive_playstle  ;
                        println!("Tal makes very aggesive moves one blunder and it is over");
                    }else if enemy_attack== 3{
                        Gotham_heath-=fear;
                        println!("You are becoming less confident with every move Tal makes");
                    }else{
                        Gotham_heath-=sacrifice;
                        println!("Tal sacrifices all of his pieses to get you king");
                    }
                    
                        
                    

                    
                }if Tal_heath <= 0{
                println!("You win the battle.");
                break;
                }else if Gotham_heath<= 0{
                
                        println!("Tal wins");
                        break;
                }
            println!("Tals HP: {}", Tal_heath);
            println!("Gotham HP: {}", Gotham_heath);    
        } else if opponent == _capablanca{ 
            
            
        
                if roll== 1{

                    
                    println!("Your turn to attack.");
                    crate::stats::gothamches::Da_rook::Gotham_stats();
                    let attack = choose_attack();
                    if attack == 1{
                        Capablanca_heath -= DA_ROOK;
                        println!("And then he sacrifices THE ROOOOOK!");
                    }else if attack == 2{
                        Capablanca_heath -= Trompowski;
                        println!("Gotham uses Tromposki one of his favorite openigs");
                    }else if attack == 3{
                        Capablanca_heath-=Caro_Can;
                        println!("Gotham uses Caro-Can one of his favorite openigs");
                    }else if attack ==4{
                        Capablanca_heath-=Milking_Magnus;
                        println!("Gotham uses forbidden techique and uses Magnus for content");
                
                    }else{
                        println!("Wrong. Chose another one");
                    }
                }
                else{
                    let enemy_attack: i32 = rng.random_range(1..=4);
                    println!("Capablancas turn to attack.");
                    if enemy_attack == 1 {
                        Gotham_heath -= chess_fundumentals ;
                        println!("Capablanca uses his to overwhelm you");
                    }else if enemy_attack == 2{
                        Gotham_heath -=Positional_sification   ;
                        println!("Capablanca slowly sificates you untill run out of moves");
                    }else if enemy_attack== 3{
                        Gotham_heath-=slow_but_steady ;
                        
                        println!("Capablanca has a lot of time so he will profilactic moves while you struggling");
                    }else{
                        Gotham_heath-=Perfect_calculation;
                        
                        println!("Capablanca is calm he has it all calculated");
                    }
                    
                        
                    

                    
                }if Capablanca_heath <= 0{
                println!("You win the battle.");

                break;
                }else if Gotham_heath<= 0{
                
                        println!("Capablanca wins");
                        break;
                }
            println!("Capablanca HP: {}", Capablanca_heath);
            println!("Gotham HP: {}", Gotham_heath);    
        } else if opponent == _lasker{ 
            
            
        
                if roll== 1{

                    
                    println!("Your turn to attack.");
                    crate::stats::gothamches::Da_rook::Gotham_stats();
                    let attack = choose_attack();
                    if attack == 1{
                        Lasker_heath -= DA_ROOK;
                        println!("And then he sacrifices THE ROOOOOK!");
                    }else if attack == 2{
                        Lasker_heath -= Trompowski;
                        println!("Gotham uses Tromposki one of his favorite openigs");
                    }else if attack == 3{
                        Lasker_heath-=Caro_Can;
                        println!("Gotham uses Caro-Can one of his favorite openigs");
                    }else if attack ==4{
                        Lasker_heath-=Milking_Magnus;
                        println!("Gotham uses forbidden techique and uses Magnus for content");
                
                    }else{
                        println!("Wrong. Chose another one");
                    }
                }
                else{
                    let enemy_attack: i32 = rng.random_range(1..=4);
                    println!("Laskers turn to attack.");
                    if enemy_attack == 1 {
                        Gotham_heath -= Greek_gift ;
                        println!("Lasker uses imaginery techique Greek gift");
                    }else if enemy_attack == 2{
                        Gotham_heath -=the_longest_world_champion   ;
                        println!("Lasker uses his knowlege from previos battes to beat you");
                    }else if enemy_attack== 3{
                        Gotham_heath-=chess_fundumentals  ;
                        println!("Lasker follos chess principles waiting for you to sleep up");
                    }else{
                        Gotham_heath-=aggresive_playstyle;
                        println!("Lasker attaks you untill there is nothig left to defend");
                    }
                    
                        
                    

                    
                }if Lasker_heath <= 0{
                println!("You win the battle.");
                break;
                }else if Gotham_heath<= 0{
                
                        println!("Lasker wins");
                        break;
                }
            println!("Lasker HP: {}", Lasker_heath);
            println!("Gotham HP: {}", Gotham_heath);    
        } else if opponent == _bobby_fisher{ 
            
            
        
                if roll== 1{

                    
                    println!("Your turn to attack.");
                    crate::stats::gothamches::Da_rook::Gotham_stats();
                    let attack = choose_attack();
                    if attack == 1{
                        Bobby_Fisher_heath -= DA_ROOK;
                        println!("And then he sacrifices THE ROOOOOK!");
                    }else if attack == 2{
                        Bobby_Fisher_heath -= Trompowski;
                        println!("Gotham uses Tromposki one of his favorite openigs");
                    }else if attack == 3{
                        Bobby_Fisher_heath-=Caro_Can;
                        println!("Gotham uses Caro-Can one of his favorite openigs");
                    }else if attack ==4{
                        Bobby_Fisher_heath-=Milking_Magnus;
                        println!("Gotham uses forbidden techique and uses Magnus for content");
                
                    }else{
                        println!("Wrong. Chose another one");
                    }
                }
                else{
                    let enemy_attack: i32 = rng.random_range(1..=4);
                    println!("Bobby_Fishers turn to attack.");
                    println!();
                    if enemy_attack == 1 {
                        Gotham_heath -= I_hate_chess ;
                        println!("He Genially hates chess");
                    }else if enemy_attack == 2{
                        Gotham_heath -=aggresive_playstle   ;
                        println!("Throws pieces left and right destroying you");
                    }else if enemy_attack== 3{
                        Gotham_heath-=sacrifice ;
                        println!("Brillant cursed techique leads to your unstoppble demise");
                    }else{
                        Gotham_heath-=aura ;
                        println!("Everything is stunned by his presence");
                    }
                    
                        
                    

                    
                }if Bobby_Fisher_heath <= 0{
                println!("You win the battle.");
                break;
                }else if Gotham_heath<= 0{
                
                        println!("Bobby_Fisher wins");
                        break;
                }
            println!("Bobby_Fisher HP: {}", Bobby_Fisher_heath);
            println!("Gotham HP: {}", Gotham_heath);    
        }
    }  
}
pub fn choose_attack() -> i32 {
    loop {
        let mut attack = String::new();

        println!("Choose your attack (1-4):");

        std::io::stdin()
            .read_line(&mut attack)
            .expect("Failed to read input");

        match attack.trim().parse::<i32>() {
            Ok(number) if (1..=4).contains(&number) => {
                return number;
            }
            _ => {
                println!("Wrong choice. Choose 1-4.");
            }
        }
    }
}

           
           