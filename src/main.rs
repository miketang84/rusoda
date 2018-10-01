
#[macro_use]
mod helper_macros;
mod db;
mod model;
mod dataservice;
mod util;


fn main () {
    env_logger::init();

    dataservice::user::test();
}
