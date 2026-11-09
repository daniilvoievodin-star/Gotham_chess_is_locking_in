mod a;
    
mod intro{
    pub mod gotham_is_locked_in;
}
mod stats;
mod Combat; 


fn main() {
    intro::gotham_is_locked_in::lock_in();
    let [_hikaru,_magnus,_martin,_tal,_capablanca,_lasker,_bobby_fisher]= ["Hikaru".to_string(),"Magnus".to_string(),"Martin".to_string(),"Tal".to_string(),"Capablanca".to_string(),"Lasker".to_string(),"Bobby_Fisher".to_string()];
    let mut opponent:String =String::new();
    crate::stats::gothamches::Da_rook::Gotham_stats();
    println!("Chose your opponent:");
    println!("{}",_hikaru);
    println!("{}",_magnus);
    println!("{}",_martin);
    println!("{}",_tal);
    println!("{}",_capablanca);
    println!("{}",_lasker);
    println!("{}",_bobby_fisher);
    a::players::chosing();
    

}

