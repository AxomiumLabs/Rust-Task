
mod nasa;
use nasa::appolo::fake_landing;
use nasa::artmis::future_mission;
use nasa::falcon::mars_mission::virgin_galatic;
fn main() {
    fake_landing::owner();
    future_mission::maywork();
    virgin_galatic::owner();
    
}
