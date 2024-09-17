//! Automated UI Testing
//!
//! Run with:
//! ```sh
//! cargo test -- --test-threads 1
//! ```

use std::{panic, process::Command, thread::sleep, time::Duration};

use test_by_a11y::prelude::*;

#[cfg(target_os = "linux")]
fn start_test<F>(test_script: F)
where
    F: FnOnce(panic::AssertUnwindSafe<&mut TestByATSPI>) -> () + panic::UnwindSafe,
{
    // Start logging
    let _ = pretty_env_logger::try_init();

    // Build first
    log::debug!("Build program...");
    let _ = Command::new("cargo")
        .arg("build")
        .output()
        .expect("cannot start child");

    // Start the program
    log::debug!("Starting program...");
    let mut program = Command::new("cargo")
        .arg("run")
        .spawn()
        .expect("cannot start child");

    // To allow program to start...
    sleep(Duration::from_millis(500));

    // Connect to the accessibility interface
    log::debug!("Connecting to the a11y interface...");
    let result = if let Ok(mut test) = TestByATSPI::connect("relm-test".to_string()) {
        // Run the test, catching any panics
        log::info!("Running test...");
        let wrapper = panic::AssertUnwindSafe(&mut test);
        Some(panic::catch_unwind(move || test_script(wrapper)))
    } else {
        None
    };

    // Kill the program now testing is complete
    log::debug!("Killing child.");
    program.kill().expect("failed to kill child");

    // Resume any panics
    if let Some(result) = result {
        if let Err(e) = result {
            log::debug!("Forwarding panic.");
            panic::resume_unwind(e);
        }
    } else {
        panic!("failed to connect to accessibility interface")
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
