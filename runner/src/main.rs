use engine::{ClockManager, FrameRate};

fn main() {
    let mut manager = ClockManager::new();

    let _clock1 = manager.create_clock(FrameRate::Fps24);
    let _clock2 = manager.create_clock(FrameRate::Fps30);

    loop {
        manager.tick_all();
        manager.print_all();

        std::thread::sleep(std::time::Duration::from_millis(40));
    }
}
