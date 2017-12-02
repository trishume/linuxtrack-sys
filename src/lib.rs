
pub const STATE_LINUXTRACK_OK: State = 0;
pub const STATE_INITIALIZING: State = 1;
pub const STATE_RUNNING: State = 2;
pub const STATE_PAUSED: State = 3;
pub const STATE_STOPPED: State = 4;
pub const STATE_ERR_NOT_INITIALIZED: State = -1;
pub const STATE_ERR_SYMBOL_LOOKUP: State = -2;
pub const STATE_ERR_NO_CONFIG: State = -3;
pub const STATE_ERR_NOT_FOUND: State = -4;
pub const STATE_ERR_PROCESSING_FRAME: State = -5;
pub type State = ::std::os::raw::c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Pose {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub tx: f32,
    pub ty: f32,
    pub tz: f32,
    pub counter: u32,
    pub resolution_x: u32,
    pub resolution_y: u32,
    pub raw_pitch: f32,
    pub raw_yaw: f32,
    pub raw_roll: f32,
    pub raw_tx: f32,
    pub raw_ty: f32,
    pub raw_tz: f32,
    pub status: u8,
}

extern "C" {
    pub fn linuxtrack_init(cust_section: *const ::std::os::raw::c_char) -> State;
    pub fn linuxtrack_shutdown() -> State;
    pub fn linuxtrack_suspend() -> State;
    pub fn linuxtrack_wakeup() -> State;
    pub fn linuxtrack_recenter() -> State;
    pub fn linuxtrack_explain(err: State) -> *const ::std::os::raw::c_char;
    pub fn linuxtrack_get_tracking_state() -> State;
    pub fn linuxtrack_get_pose(heading: *mut f32,
                               pitch: *mut f32,
                               roll: *mut f32,
                               tx: *mut f32,
                               ty: *mut f32,
                               tz: *mut f32,
                               counter: *mut u32)
                               -> ::std::os::raw::c_int;
    pub fn linuxtrack_get_pose_full(pose: *mut Pose,
                                    blobs: *mut f32,
                                    num_blobs: ::std::os::raw::c_int,
                                    blobs_read: *mut ::std::os::raw::c_int)
                                    -> ::std::os::raw::c_int;
    pub fn linuxtrack_get_abs_pose(heading: *mut f32,
                                   pitch: *mut f32,
                                   roll: *mut f32,
                                   tx: *mut f32,
                                   ty: *mut f32,
                                   tz: *mut f32,
                                   counter: *mut u32)
                                   -> ::std::os::raw::c_int;
    pub fn linuxtrack_request_frames() -> State;
    pub fn linuxtrack_get_frame(req_width: *mut ::std::os::raw::c_int,
                                req_height: *mut ::std::os::raw::c_int,
                                buf_size: usize,
                                buffer: *mut u8)
                                -> ::std::os::raw::c_int;
    pub fn linuxtrack_notification_on() -> State;
    pub fn linuxtrack_get_notify_pipe() -> ::std::os::raw::c_int;
    pub fn linuxtrack_wait(timeout: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
