use std::io;

pub fn chosing(){
   
    let [_hikaru,_magnus,_martin,_tal,_capablanca,_lasker,_bobby_fisher]= ["Hikaru".to_string(),"Magnus".to_string(),"Martin".to_string(),"Tal".to_string(),"Capablanca".to_string(),"Lasker".to_string(),"Bobby_Fisher".to_string()];
   
    loop{
        
        
        
        let mut opponent = String::new();
        io::stdin().read_line(&mut opponent).expect("chose again");

        let opponent = opponent.trim().to_string();
        if opponent == _hikaru{
            println!("You are fithing against {}!",_hikaru);
            crate::stats::oppnets::chess::Hikaru_stats();
           
             
            
        
        }else if opponent == _magnus{
            println!("You are fithing against {}!",_magnus);
            crate::stats::oppnets::chess::Magnus_stats();
            
              
            
        }else if opponent == _martin{
            println!("You are fithing against {}!",_martin);
            crate::stats::oppnets::chess::Martin_stats();
            
             
           
        }else if opponent == _tal{
            println!("You are fithing against {}!",_tal);
            crate::stats::oppnets::chess::Tal_stats();
            
            
            
        
        }else if opponent == _capablanca{
            println!("You are fithing against {}!",_capablanca);
            crate::stats::oppnets::chess::Capablanca_stats();
            
           
            
        }else if opponent == _lasker{
            println!("You are fithing against {}!",_lasker);
            crate::stats::oppnets::chess::Lasker_stats();
            
            
        }else if opponent== _bobby_fisher{
            println!("You are fithing against {}!",_bobby_fisher);
            crate::stats::oppnets::chess::Bobby_Fisher_stats();
            
            
        }else{
            println!("Wrong opponent. Chose again");
            continue;
        }
        crate::Combat::combat::Combat(opponent.clone());
        continue;
    }    
}    