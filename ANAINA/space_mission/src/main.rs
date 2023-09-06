mod nasa;
use nasa::appolo::fake_landing;
use nasa::artmis::future_mission;
use nasa::falcon::mars_mission::virgin_galactic;
use nasa::falcon::space_tour::spacex;
fn main() {
    fake_landing::owner();
    future_mission::may_work();
    spacex::owner();
    virgin_galactic::owner();
}
