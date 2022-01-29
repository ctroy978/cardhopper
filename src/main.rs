mod games;

use crate::games::hicard::*;
use cardlib::*;

fn main() {
    let mut players = vec!["tim", "lin", "corn"];
    play_hicard(players);
}
