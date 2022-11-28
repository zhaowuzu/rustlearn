// use xiv_art::kinds::PrimaryColor;
// use xiv_art::kinds::SecondaryColor;
// use xiv_art::utils::mix;

use xiv_art::PrimaryColor;
use xiv_art::SecondaryColor;
use xiv_art::mix;

fn main(){
    let red = PrimaryColor::Red;
    let yellow = SecondaryColor::Orange;
    mix(red,yellow);
}