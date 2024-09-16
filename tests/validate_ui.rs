use std::{panic, process::Command, thread::sleep, time::Duration};

use test_by_a11y::prelude::*;

#[cfg(target_os = "linux")]
fn start_test<F>(test_script: F)
where
    F: FnOnce(panic::AssertUnwindSafe<&mut TestByATSPI>) -> () + panic::UnwindSafe,
{
    // Delay between tests to prevent dbus issues.
    sleep(Duration::from_millis(1000));

    // Start logging
    let _ = pretty_env_logger::try_init();

    // Start the program
    log::debug!("Starting program...");
    let mut program = Command::new("cargo")
        .arg("run")
        .spawn()
        .expect("cannot start child");

    // To allow program to start...
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Connect to the accessibility interface
    log::debug!("Connecting to the a11y interface...");
    let mut test = TestByATSPI::connect("relm-test".to_string())
        .expect("failed to connect to accessibility interface");

    // To things to settle...
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Run the test, catching any panics
    log::info!("Running test...");
    let wrapper = panic::AssertUnwindSafe(&mut test);
    let result = panic::catch_unwind(move || test_script(wrapper));

    // Kill the program now testing is complete
    log::debug!("Killing child.");
    program.kill().expect("failed to kill child");

    // Resume any panics
    if let Err(e) = result {
        log::debug!("Forwarding panic.");
        panic::resume_unwind(e);
    }
}

#[test]
#[cfg(target_os = "linux")]
fn test_counter() {
    start_test(|mut test| {
        // Find the increase button
        let btn_inc = test
            .find(By::Text("Increment".to_string()))
            .unwrap()
            .unwrap();
        // Find the decrease button
        let btn_dec = test
            .find(By::Text("Decrement".to_string()))
            .unwrap()
            .unwrap();

        // Increment to 1
        test.interact(&btn_inc, Interaction::Click).unwrap();
        sleep(Duration::from_millis(500));
        assert!(test
            .find(By::Text("Counter: 1".to_string()))
            .unwrap()
            .is_some());
        // Decrement wrapping to 255
        test.interact(&btn_dec, Interaction::Click).unwrap();
        sleep(Duration::from_millis(500));
        test.interact(&btn_dec, Interaction::Click).unwrap();
        sleep(Duration::from_millis(500));
        assert!(test
            .find(By::Text("Counter: 255".to_string()))
            .unwrap()
            .is_some());
    });
}

#[test]
#[cfg(target_os = "linux")]
fn test_icon_button() {
    start_test(|mut test| {
        // Find the icon button
        let btn_icon = test
            .find(By::Text("Papyrus Button to test icons".to_string()))
            .unwrap();
        assert!(btn_icon.is_some());
    });
}
