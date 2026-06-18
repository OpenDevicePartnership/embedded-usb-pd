/// Type-C current
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Current {
    /// Default USB current
    #[default]
    UsbDefault,
    /// 1.5A
    Current1A5,
    /// 3A0
    Current3A0,
}

impl Current {
    /// Returns the current in mA
    pub fn to_ma(self, is_usb2: bool) -> u16 {
        match self {
            Current::UsbDefault => {
                if is_usb2 {
                    500
                } else {
                    900
                }
            }
            Current::Current1A5 => 1500,
            Current::Current3A0 => 3000,
        }
    }
}

/// The current state of a Type-C port.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConnectionState {
    /// The port is connected to an USB Type-C Digital Audio (TCDA) accessory.
    ///
    /// See [USB Type-C specification, release 2.4](https://www.usb.org/document-library/usb-type-cr-cable-and-connector-specification-release-24),
    /// section C "USB Type-C Digital Audio".
    AudioAccessory,

    /// The port is in Debug Accessory Mode (DAM).
    ///
    /// See [USB Type-C specification, release 2.4](https://www.usb.org/document-library/usb-type-cr-cable-and-connector-specification-release-24),
    /// section B "Debug Accessory Mode".
    DebugAccessory,

    /// A port that is attached to another device, either PD-capable or not.
    ///
    /// An *attached* port is one that is mechanically joined with USB cable to another port.
    ///
    /// A *connected* port is one that has exchanged a Message and a GoodCRC Message response using
    /// the USB Power Delivery protocol so that both Port Partners know that each is PD Capable.
    ///
    /// See [USB PD specification, revision 3.2, version 1.1](https://www.usb.org/document-library/usb-power-delivery),
    /// section 1.6 "Terms and Abbreviations".
    Attached,
}
