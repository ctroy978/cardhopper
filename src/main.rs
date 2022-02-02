mod games;

use crate::games::stud::*;
use cardlib::*;

fn main() {
    let mut players = vec!["tim", "lin", "corn"];
    play_stud(players);
}
