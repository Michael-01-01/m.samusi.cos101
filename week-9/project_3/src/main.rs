use std::fs::File;
use std::io::Write;
use std::io::Read;

fn main() {
    let names = ["Aigbogun Alamba Daudu","Murtala Afeez"," Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"]; 

    let ministry = ["Internal affairs","Justice","Defense","Power & Steel","Petroleum"]; 

    let geopolitical_zone = ["South West","North East","South South","South West","South East"];

    let mut file = File::create("Ministers.csv").expect("Unable to create file");
    writeln!(file,"Name,ministry,geopolitical_zone").expect("write failed");

    for i in 0..names.len() {
        writeln!(file,"{},{},{}",names[i],ministry[i],geopolitical_zone[i]).expect("Failed to write");
    }

   
}
