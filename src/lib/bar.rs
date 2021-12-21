extern crate pbr;
use pbr::ProgressBar;

pub fn bar(res: &String) -> () {
    let count = res.len();
    let mut pb = ProgressBar::new(count as u64);
    pb.format("╢▌▌-╟");
    pb.show_percent = true;
    pb.show_time_left = true;
    for _ in 0..count {
        pb.inc();
    }
    pb.finish_print("done!");
    println!("{}", res);
}