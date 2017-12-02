
pub const STATUS_LINUXTRACK_OK: Status = 0;
pub const STATUS_INITIALIZING: Status = 1;
pub const STATUS_RUNNING: Status = 2;
pub const STATUS_PAUSED: Status = 3;
pub const STATUS_STOPPED: Status = 4;
pub const STATUS_ERR_NOT_INITIALIZED: Status = -1;
pub const STATUS_ERR_SYMBOL_LOOKUP: Status = -2;
pub const STATUS_ERR_NO_CONFIG: Status = -3;
pub const STATUS_ERR_NOT_FOUND: Status = -4;
pub const STATUS_ERR_PROCESSING_FRAME: Status = -5;
pub type Status = ::std::os::raw::c_int;

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
    pub fn linuxtrack_init(cust_section: *const ::std::os::raw::c_char) -> Status;
    pub fn linuxtrack_shutdown() -> Status;
    pub fn linuxtrack_suspend() -> Status;
    pub fn linuxtrack_wakeup() -> Status;
    pub fn linuxtrack_recenter() -> Status;
    pub fn linuxtrack_explain(err: Status) -> *const ::std::os::raw::c_char;
    pub fn linuxtrack_get_tracking_state() -> Status;
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
    pub fn linuxtrack_request_frames() -> Status;
    pub fn linuxtrack_get_frame(req_width: *mut ::std::os::raw::c_int,
                                req_height: *mut ::std::os::raw::c_int,
                                buf_size: usize,
                                buffer: *mut u8)
                                -> ::std::os::raw::c_int;
    pub fn linuxtrack_notification_on() -> Status;
    pub fn linuxtrack_get_notify_pipe() -> ::std::os::raw::c_int;

    /// timeout is in milliseconds. Result of 0 is timeout, 1 is success, -1 is problem
    pub fn linuxtrack_wait(timeout: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
