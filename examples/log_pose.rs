extern crate linuxtrack_sys;
use linuxtrack_sys::*;
use std::ptr;
use std::mem;
use std::os::raw;

fn main() {
    unsafe {
        let status = linuxtrack_init(ptr::null());
        println!("Init status: {:?}", status);
        let status = linuxtrack_notification_on();
        println!("Notification status: {:?}", status);

        loop {
            let res = linuxtrack_wait(1000); // 1 second timeout
            println!("waited  {:?}", res);
            if res != 1 {
                let status = linuxtrack_get_tracking_state();
                println!("Status: {:?}", status);
                continue;
            }

            let mut pose: Pose = mem::zeroed();
            let mut blobs: [f32; 9] = [0.0; 9];
            let mut blobs_read: raw::c_int = 0;
            let res = linuxtrack_get_pose_full(&mut pose as *mut _, blobs[..].as_mut_ptr(), 3, &mut blobs_read as *mut _);
            println!("got pose {:?}", res);
            println!("Pose: {:?}", pose);
            println!("Blobs: {:?}", blobs);
        }
    }
}
