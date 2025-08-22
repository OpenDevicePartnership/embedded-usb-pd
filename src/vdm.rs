pub const DATA_OBJ_SIZE: usize = 4;
pub const MAX_VDOS: usize = 6;
pub const MAX_NUM_DATA_OBJECTS: usize = 7;
pub const MAX_SIZE: usize = DATA_OBJ_SIZE * MAX_NUM_DATA_OBJECTS;
pub const OBJ_POS_ALL_MODES: u8 = 7;
pub const HEADER_VERSION: u8 = 1;

/// SVDM header commands
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmd {
    /// Discover Id
    DiscId = 1,
    /// Discover SVIDs
    DiscSvid = 2,
    /// Discover Mode
    DiscMode = 3,
    /// Enter mode
    EnterMode = 4,
    /// Exit mode
    ExitMode = 5,
    /// Attention
    Attention = 6,
    /// Custom vendor SVID Commands start here
    SvidCmdStart = 16,
}
