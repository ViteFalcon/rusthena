mod db;

fn main() {
    println!("Hello, world!");
    let table = db::open_table("../conf/login_athena.conf");
}
