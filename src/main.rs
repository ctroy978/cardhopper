mod games;

use crate::games::stud::*;
use cardhopper::*;

use combinations::Combinations;

fn main() {
    let mut players = vec!["tim", "lin", "corn"];
    play_stud(players);


}
