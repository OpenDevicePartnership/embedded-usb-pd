pub const PD_VDM_DATA_OBJ_SIZE: usize = 4;
pub const PD_VDM_MAX_VDOS: usize = 6;
pub const PD_VDM_MAX_NUM_DATA_OBJECTS: usize = 7;
pub const PD_VDM_MAX_SIZE: usize = PD_VDM_DATA_OBJ_SIZE * PD_VDM_MAX_NUM_DATA_OBJECTS;
pub const PD_VDM_OBJ_POS_ALL_MODES: u8 = 7;
pub const PD_VDM_HEADER_VERSION: u8 = 1;

/// SVDM header commands
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VdmCmd {
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
    /// Custom vendor SVID Commands Start here
    SvidCmdStart = 16,
}
