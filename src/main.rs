mod games;

use crate::games::stud::*;
use cardlib::*;

fn main() {
    let mut players = vec!["tim", "lin", "corn"];
    play_stud(players);

    //full house let crds = vec![6, 19, 32, 7, 20];
    let crds = vec![6, 20, 34, 35, 36];
    let mut player = Hand::new("tim");
    for cards in crds.iter() {
        player.deal_card(Card::new(*cards));
    }
}
