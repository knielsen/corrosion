use chip::rom::getfun;

//*****************************************************************************
//
// The following are values that can be passed to USBIntEnableControl() and
// USBIntDisableControl() as the ui32Flags parameter, and are returned from
// USBIntStatusControl().
//
//*****************************************************************************
pub const INTCTRL_ALL: u32 = 0x000003FF;  // All control interrupt sources
pub const INTCTRL_STATUS: u32 = 0x000000FF;  // Status Interrupts
pub const INTCTRL_VBUS_ERR: u32 = 0x00000080;  // VBUS Error
pub const INTCTRL_SESSION: u32 = 0x00000040;  // Session Start Detected
pub const INTCTRL_SESSION_END: u32 = 0x00000040;  // Session End Detected
pub const INTCTRL_DISCONNECT: u32 = 0x00000020;  // Disconnect Detected
pub const INTCTRL_CONNECT: u32 = 0x00000010;  // Device Connect Detected
pub const INTCTRL_SOF: u32 = 0x00000008;  // Start of Frame Detected
pub const INTCTRL_BABBLE: u32 = 0x00000004;  // Babble signaled
pub const INTCTRL_RESET: u32 = 0x00000004;  // Reset signaled
pub const INTCTRL_RESUME: u32 = 0x00000002;  // Resume detected
pub const INTCTRL_SUSPEND: u32 = 0x00000001;  // Suspend detected
pub const INTCTRL_MODE_DETECT: u32 = 0x00000200;  // Mode value valid
pub const INTCTRL_POWER_FAULT: u32 = 0x00000100;  // Power Fault detected

//*****************************************************************************
//
// The following are values that can be passed to USBIntEnableEndpoint() and
// USBIntDisableEndpoint() as the ui32Flags parameter, and are returned from
// USBIntStatusEndpoint().
//
//*****************************************************************************
pub const INTEP_ALL:         u32 = 0xFFFFFFFF;  // Host IN Interrupts
pub const INTEP_HOST_IN:     u32 = 0xFFFE0000;  // Host IN Interrupts
pub const INTEP_HOST_IN_15:  u32 = 0x80000000;  // Endpoint 15 Host IN Interrupt
pub const INTEP_HOST_IN_14:  u32 = 0x40000000;  // Endpoint 14 Host IN Interrupt
pub const INTEP_HOST_IN_13:  u32 = 0x20000000;  // Endpoint 13 Host IN Interrupt
pub const INTEP_HOST_IN_12:  u32 = 0x10000000;  // Endpoint 12 Host IN Interrupt
pub const INTEP_HOST_IN_11:  u32 = 0x08000000;  // Endpoint 11 Host IN Interrupt
pub const INTEP_HOST_IN_10:  u32 = 0x04000000;  // Endpoint 10 Host IN Interrupt
pub const INTEP_HOST_IN_9:   u32 = 0x02000000;  // Endpoint 9 Host IN Interrupt
pub const INTEP_HOST_IN_8:   u32 = 0x01000000;  // Endpoint 8 Host IN Interrupt
pub const INTEP_HOST_IN_7:   u32 = 0x00800000;  // Endpoint 7 Host IN Interrupt
pub const INTEP_HOST_IN_6:   u32 = 0x00400000;  // Endpoint 6 Host IN Interrupt
pub const INTEP_HOST_IN_5:   u32 = 0x00200000;  // Endpoint 5 Host IN Interrupt
pub const INTEP_HOST_IN_4:   u32 = 0x00100000;  // Endpoint 4 Host IN Interrupt
pub const INTEP_HOST_IN_3:   u32 = 0x00080000;  // Endpoint 3 Host IN Interrupt
pub const INTEP_HOST_IN_2:   u32 = 0x00040000;  // Endpoint 2 Host IN Interrupt
pub const INTEP_HOST_IN_1:   u32 = 0x00020000;  // Endpoint 1 Host IN Interrupt

pub const INTEP_DEV_OUT:     u32 = 0xFFFE0000;  // Device OUT Interrupts
pub const INTEP_DEV_OUT_15:  u32 = 0x80000000;  // Endpoint 15 Device OUT Interrupt
pub const INTEP_DEV_OUT_14:  u32 = 0x40000000;  // Endpoint 14 Device OUT Interrupt
pub const INTEP_DEV_OUT_13:  u32 = 0x20000000;  // Endpoint 13 Device OUT Interrupt
pub const INTEP_DEV_OUT_12:  u32 = 0x10000000;  // Endpoint 12 Device OUT Interrupt
pub const INTEP_DEV_OUT_11:  u32 = 0x08000000;  // Endpoint 11 Device OUT Interrupt
pub const INTEP_DEV_OUT_10:  u32 = 0x04000000;  // Endpoint 10 Device OUT Interrupt
pub const INTEP_DEV_OUT_9:   u32 = 0x02000000;  // Endpoint 9 Device OUT Interrupt
pub const INTEP_DEV_OUT_8:   u32 = 0x01000000;  // Endpoint 8 Device OUT Interrupt
pub const INTEP_DEV_OUT_7:   u32 = 0x00800000;  // Endpoint 7 Device OUT Interrupt
pub const INTEP_DEV_OUT_6:   u32 = 0x00400000;  // Endpoint 6 Device OUT Interrupt
pub const INTEP_DEV_OUT_5:   u32 = 0x00200000;  // Endpoint 5 Device OUT Interrupt
pub const INTEP_DEV_OUT_4:   u32 = 0x00100000;  // Endpoint 4 Device OUT Interrupt
pub const INTEP_DEV_OUT_3:   u32 = 0x00080000;  // Endpoint 3 Device OUT Interrupt
pub const INTEP_DEV_OUT_2:   u32 = 0x00040000;  // Endpoint 2 Device OUT Interrupt
pub const INTEP_DEV_OUT_1:   u32 = 0x00020000;  // Endpoint 1 Device OUT Interrupt

pub const INTEP_HOST_OUT:    u32 = 0x0000FFFE;  // Host OUT Interrupts
pub const INTEP_HOST_OUT_15: u32 = 0x00008000;  // Endpoint 15 Host OUT Interrupt
pub const INTEP_HOST_OUT_14: u32 = 0x00004000;  // Endpoint 14 Host OUT Interrupt
pub const INTEP_HOST_OUT_13: u32 = 0x00002000;  // Endpoint 13 Host OUT Interrupt
pub const INTEP_HOST_OUT_12: u32 = 0x00001000;  // Endpoint 12 Host OUT Interrupt
pub const INTEP_HOST_OUT_11: u32 = 0x00000800;  // Endpoint 11 Host OUT Interrupt
pub const INTEP_HOST_OUT_10: u32 = 0x00000400;  // Endpoint 10 Host OUT Interrupt
pub const INTEP_HOST_OUT_9:  u32 = 0x00000200;  // Endpoint 9 Host OUT Interrupt
pub const INTEP_HOST_OUT_8:  u32 = 0x00000100;  // Endpoint 8 Host OUT Interrupt
pub const INTEP_HOST_OUT_7:  u32 = 0x00000080;  // Endpoint 7 Host OUT Interrupt
pub const INTEP_HOST_OUT_6:  u32 = 0x00000040;  // Endpoint 6 Host OUT Interrupt
pub const INTEP_HOST_OUT_5:  u32 = 0x00000020;  // Endpoint 5 Host OUT Interrupt
pub const INTEP_HOST_OUT_4:  u32 = 0x00000010;  // Endpoint 4 Host OUT Interrupt
pub const INTEP_HOST_OUT_3:  u32 = 0x00000008;  // Endpoint 3 Host OUT Interrupt
pub const INTEP_HOST_OUT_2:  u32 = 0x00000004;  // Endpoint 2 Host OUT Interrupt
pub const INTEP_HOST_OUT_1:  u32 = 0x00000002;  // Endpoint 1 Host OUT Interrupt

pub const INTEP_DEV_IN:      u32 = 0x0000FFFE;  // Device IN Interrupts
pub const INTEP_DEV_IN_15:   u32 = 0x00008000;  // Endpoint 15 Device IN Interrupt
pub const INTEP_DEV_IN_14:   u32 = 0x00004000;  // Endpoint 14 Device IN Interrupt
pub const INTEP_DEV_IN_13:   u32 = 0x00002000;  // Endpoint 13 Device IN Interrupt
pub const INTEP_DEV_IN_12:   u32 = 0x00001000;  // Endpoint 12 Device IN Interrupt
pub const INTEP_DEV_IN_11:   u32 = 0x00000800;  // Endpoint 11 Device IN Interrupt
pub const INTEP_DEV_IN_10:   u32 = 0x00000400;  // Endpoint 10 Device IN Interrupt
pub const INTEP_DEV_IN_9:    u32 = 0x00000200;  // Endpoint 9 Device IN Interrupt
pub const INTEP_DEV_IN_8:    u32 = 0x00000100;  // Endpoint 8 Device IN Interrupt
pub const INTEP_DEV_IN_7:    u32 = 0x00000080;  // Endpoint 7 Device IN Interrupt
pub const INTEP_DEV_IN_6:    u32 = 0x00000040;  // Endpoint 6 Device IN Interrupt
pub const INTEP_DEV_IN_5:    u32 = 0x00000020;  // Endpoint 5 Device IN Interrupt
pub const INTEP_DEV_IN_4:    u32 = 0x00000010;  // Endpoint 4 Device IN Interrupt
pub const INTEP_DEV_IN_3:    u32 = 0x00000008;  // Endpoint 3 Device IN Interrupt
pub const INTEP_DEV_IN_2:    u32 = 0x00000004;  // Endpoint 2 Device IN Interrupt
pub const INTEP_DEV_IN_1:    u32 = 0x00000002;  // Endpoint 1 Device IN Interrupt

pub const INTEP_0:           u32 = 0x00000001;  // Endpoint 0 Interrupt

//*****************************************************************************
//
// The following are values that are returned from USBSpeedGet().
//
//*****************************************************************************
pub const UNDEF_SPEED: u32 = 0x80000000;  // Current speed is undefined
pub const HIGH_SPEED:  u32 = 0x00000002;  // Current speed is High Speed
pub const FULL_SPEED:  u32 = 0x00000001;  // Current speed is Full Speed
pub const LOW_SPEED:   u32 = 0x00000000;  // Current speed is Low Speed

//*****************************************************************************
//
// The following are values that are returned from USBEndpointStatus().  The
// USB_HOST_* values are used when the USB controller is in host mode and the
// USB_DEV_* values are used when the USB controller is in device mode.
//
//*****************************************************************************
pub const HOST_IN_STATUS: u32 = 0x114F0000;  // Mask of all host IN interrupts
pub const HOST_IN_PID_ERROR: u32 = 0x10000000;  // Stall on this endpoint received
pub const HOST_IN_NOT_COMP: u32 = 0x01000000;  // Device failed to respond
pub const HOST_IN_STALL: u32 = 0x00400000;  // Stall on this endpoint received
pub const HOST_IN_DATA_ERROR: u32 = 0x00080000;  // CRC or bit-stuff error
                                            // (ISOC Mode)
pub const HOST_IN_NAK_TO: u32 = 0x00080000;  // NAK received for more than the
                                            // specified timeout period
pub const HOST_IN_ERROR: u32 = 0x00040000;  // Failed to communicate with a
                                            // device
pub const HOST_IN_FIFO_FULL: u32 = 0x00020000;  // RX FIFO full
pub const HOST_IN_PKTRDY: u32 = 0x00010000;  // Data packet ready
pub const HOST_OUT_STATUS: u32 = 0x000000A7;  // Mask of all host OUT interrupts
pub const HOST_OUT_NAK_TO: u32 = 0x00000080;  // NAK received for more than the
                                            // specified timeout period
pub const HOST_OUT_NOT_COMP: u32 = 0x00000080;  // No response from device
                                            // (ISOC mode)
pub const HOST_OUT_STALL: u32 = 0x00000020;  // Stall on this endpoint received
pub const HOST_OUT_ERROR: u32 = 0x00000004;  // Failed to communicate with a
                                            // device
pub const HOST_OUT_FIFO_NE: u32 = 0x00000002;  // TX FIFO is not empty
pub const HOST_OUT_PKTPEND: u32 = 0x00000001;  // Transmit still being transmitted
pub const HOST_EP0_NAK_TO: u32 = 0x00000080;  // NAK received for more than the
                                            // specified timeout period
pub const HOST_EP0_STATUS: u32 = 0x00000040;  // This was a status packet
pub const HOST_EP0_ERROR: u32 = 0x00000010;  // Failed to communicate with a
                                            // device
pub const HOST_EP0_RX_STALL: u32 = 0x00000004;  // Stall on this endpoint received
pub const HOST_EP0_RXPKTRDY: u32 = 0x00000001;  // Receive data packet ready
pub const DEV_RX_PID_ERROR: u32 = 0x01000000;  // PID error in isochronous
                                            // transfer
pub const DEV_RX_SENT_STALL: u32 = 0x00400000;  // Stall was sent on this endpoint
pub const DEV_RX_DATA_ERROR: u32 = 0x00080000;  // CRC error on the data
pub const DEV_RX_OVERRUN: u32 = 0x00040000;  // OUT packet was not loaded due to
                                            // a full FIFO
pub const DEV_RX_FIFO_FULL: u32 = 0x00020000;  // RX FIFO full
pub const DEV_RX_PKT_RDY: u32 = 0x00010000;  // Data packet ready
pub const DEV_TX_NOT_COMP: u32 = 0x00000080;  // Large packet split up, more data
                                            // to come
pub const DEV_TX_SENT_STALL: u32 = 0x00000020;  // Stall was sent on this endpoint
pub const DEV_TX_UNDERRUN: u32 = 0x00000004;  // IN received with no data ready
pub const DEV_TX_FIFO_NE: u32 = 0x00000002;  // The TX FIFO is not empty
pub const DEV_TX_TXPKTRDY: u32 = 0x00000001;  // Transmit still being transmitted
pub const DEV_EP0_SETUP_END: u32 = 0x00000010;  // Control transaction ended before
                                            // Data End seen
pub const DEV_EP0_SENT_STALL: u32 = 0x00000004;  // Stall was sent on this endpoint
pub const DEV_EP0_IN_PKTPEND: u32 = 0x00000002;  // Transmit data packet pending
pub const DEV_EP0_OUT_PKTRDY: u32 = 0x00000001;  // Receive data packet ready

//*****************************************************************************
//
// The following are values that can be passed to USBHostEndpointConfig() and
// USBDevEndpointConfigSet() as the ui32Flags parameter.
//
//*****************************************************************************
pub const EP_AUTO_SET: u32 = 0x00000001;  // Auto set feature enabled
pub const EP_AUTO_REQUEST: u32 = 0x00000002;  // Auto request feature enabled
pub const EP_AUTO_CLEAR: u32 = 0x00000004;  // Auto clear feature enabled
pub const EP_DMA_MODE_0: u32 = 0x00000008;  // Enable DMA access using mode 0
pub const EP_DMA_MODE_1: u32 = 0x00000010;  // Enable DMA access using mode 1
pub const EP_DIS_NYET: u32 = 0x00000020;  // Disable NYET response for
                                            // high-speed Bulk and Interrupt
                                            // endpoints in device mode.
pub const EP_MODE_ISOC: u32 = 0x00000000;  // Isochronous endpoint
pub const EP_MODE_BULK: u32 = 0x00000100;  // Bulk endpoint
pub const EP_MODE_INT: u32 = 0x00000200;  // Interrupt endpoint
pub const EP_MODE_CTRL: u32 = 0x00000300;  // Control endpoint
pub const EP_MODE_MASK: u32 = 0x00000300;  // Mode Mask
pub const EP_SPEED_LOW: u32 = 0x00000000;  // Low Speed
pub const EP_SPEED_FULL: u32 = 0x00001000;  // Full Speed
pub const EP_SPEED_HIGH: u32 = 0x00004000;  // High Speed
pub const EP_HOST_IN: u32 = 0x00000000;  // Host IN endpoint
pub const EP_HOST_OUT: u32 = 0x00002000;  // Host OUT endpoint
pub const EP_DEV_IN: u32 = 0x00002000;  // Device IN endpoint
pub const EP_DEV_OUT: u32 = 0x00000000;  // Device OUT endpoint

//*****************************************************************************
//
// The following are values that can be passed to USBHostPwrConfig() as the
// ui32Flags parameter.
//
//*****************************************************************************
pub const HOST_PWRFLT_LOW: u32 = 0x00000010;
pub const HOST_PWRFLT_HIGH: u32 = 0x00000030;
pub const HOST_PWRFLT_EP_NONE: u32 = 0x00000000;
pub const HOST_PWRFLT_EP_TRI: u32 = 0x00000140;
pub const HOST_PWRFLT_EP_LOW: u32 = 0x00000240;
pub const HOST_PWRFLT_EP_HIGH: u32 = 0x00000340;
pub const HOST_PWREN_MAN_LOW: u32 = 0x00000000;
pub const HOST_PWREN_MAN_HIGH: u32 = 0x00000001;
pub const HOST_PWREN_AUTOLOW: u32 = 0x00000002;
pub const HOST_PWREN_AUTOHIGH: u32 = 0x00000003;
pub const HOST_PWREN_FILTER: u32 = 0x00010000;

//*****************************************************************************
//
// The following are the valid values that can be passed to the
// USBHostLPMConfig() function in the ui32Config parameter.
//
//*****************************************************************************
pub const HOST_LPM_RMTWAKE: u32 = 0x00000100;
pub const HOST_LPM_L1: u32 = 0x00000001;

//*****************************************************************************
//
// The following are the valid values that can be passed to the
// USBDevLPMConfig() function in the ui32Config parameter.
//
//*****************************************************************************
pub const DEV_LPM_NAK: u32 = 0x00000010;
pub const DEV_LPM_NONE: u32 = 0x00000000;
pub const DEV_LPM_EN: u32 = 0x0000000c;
pub const DEV_LPM_EXTONLY: u32 = 0x00000004;

//*****************************************************************************
//
// The following are the valid values that are returned from the
// USBLPMLinkStateGet() function.
//
//*****************************************************************************
pub const DEV_LPM_LS_RMTWAKE: u32 = 0x00000100;
pub const DEV_LPM_LS_L1: u32 = 0x00000001;

//*****************************************************************************
//
// The following are the valid values that are passed to the USBLPMIntEnable()
// or USBLPMIntDisable() functions or are returned from the USBLPMIntStatus()
// function.
//
//*****************************************************************************
pub const INTLPM_ERROR: u32 = 0x00000020;
pub const INTLPM_RESUME: u32 = 0x00000010;
pub const INTLPM_INCOMPLETE: u32 = 0x00000008;
pub const INTLPM_ACK: u32 = 0x00000004;
pub const INTLPM_NYET: u32 = 0x00000002;
pub const INTLPM_STALL: u32 = 0x00000001;

//*****************************************************************************
//
// The following are the valid values that are passed to the USBClockEnable()
// functions.
//
//*****************************************************************************
pub const CLOCK_INTERNAL: u32 = 0x00000200;
pub const CLOCK_EXTERNAL: u32 = 0x00000300;

//*****************************************************************************
//
// The configuration options used with the USBULPIConfig() API.
//
//*****************************************************************************
pub const ULPI_EXTVBUS: u32 = 0x00000001;
pub const ULPI_EXTVBUS_IND: u32 = 0x00000002;

//*****************************************************************************
//
// The following are special values that can be passed to
// USBHostEndpointConfig() as the ui32NAKPollInterval parameter.
//
//*****************************************************************************
pub const MAX_NAK_LIMIT: u32 = 31;          // Maximum NAK interval
pub const DISABLE_NAK_LIMIT: u32 = 0;           // No NAK timeouts

//*****************************************************************************
//
// This value specifies the maximum size of transfers on endpoint 0 as 64
// bytes.  This value is fixed in hardware as the FIFO size for endpoint 0.
//
//*****************************************************************************
pub const MAX_PACKET_SIZE_EP0: u32 = 64;

//*****************************************************************************
//
// These values are used to indicate which endpoint to access.
//
//*****************************************************************************
pub const EP_0: u32 = 0x00000000;  // Endpoint 0
pub const EP_1: u32 = 0x00000010;  // Endpoint 1
pub const EP_2: u32 = 0x00000020;  // Endpoint 2
pub const EP_3: u32 = 0x00000030;  // Endpoint 3
pub const EP_4: u32 = 0x00000040;  // Endpoint 4
pub const EP_5: u32 = 0x00000050;  // Endpoint 5
pub const EP_6: u32 = 0x00000060;  // Endpoint 6
pub const EP_7: u32 = 0x00000070;  // Endpoint 7
pub const NUM_EP: u32 = 8;         // Number of supported endpoints

//*****************************************************************************
//
// These macros allow conversion between 0-based endpoint indices and the
// USB_EP_x values required when calling various USB APIs.
//
//*****************************************************************************
#[inline]
pub fn IndexToUSBEP(x: u32) -> u32 { x << 4 }
#[inline]
pub fn USBEPToIndex(x: u32) -> u32 { x >> 4 }

//*****************************************************************************
//
// The following are values that can be passed to USBFIFOConfigSet() as the
// ui32FIFOSize parameter.
//
//*****************************************************************************
pub const FIFO_SZ_8: u32 = 0x00000000;  // 8 byte FIFO
pub const FIFO_SZ_16: u32 = 0x00000001;  // 16 byte FIFO
pub const FIFO_SZ_32: u32 = 0x00000002;  // 32 byte FIFO
pub const FIFO_SZ_64: u32 = 0x00000003;  // 64 byte FIFO
pub const FIFO_SZ_128: u32 = 0x00000004;  // 128 byte FIFO
pub const FIFO_SZ_256: u32 = 0x00000005;  // 256 byte FIFO
pub const FIFO_SZ_512: u32 = 0x00000006;  // 512 byte FIFO
pub const FIFO_SZ_1024: u32 = 0x00000007;  // 1024 byte FIFO
pub const FIFO_SZ_2048: u32 = 0x00000008;  // 2048 byte FIFO

//*****************************************************************************
//
// This macro allow conversion from a FIFO size label as defined above to
// a number of bytes
//
//*****************************************************************************
#[inline]
pub fn FIFOSizeToBytes(x: u32) -> u32 { 8u32 << (x as uint) }

//*****************************************************************************
//
// The following are values that can be passed to USBEndpointDataSend() as the
// ui32TransType parameter.
//
//*****************************************************************************
pub const TRANS_OUT: u32 = 0x00000102;  // Normal OUT transaction
pub const TRANS_IN: u32 = 0x00000102;  // Normal IN transaction
pub const TRANS_IN_LAST: u32 = 0x0000010a;  // Final IN transaction (for
                                            // endpoint 0 in device mode)
pub const TRANS_SETUP: u32 = 0x0000110a;  // Setup transaction (for endpoint
                                            // 0)
pub const TRANS_STATUS: u32 = 0x00000142;  // Status transaction (for endpoint
                                            // 0)

//*****************************************************************************
//
// The following are values are returned by the USBModeGet function.
//
//*****************************************************************************
pub const DUAL_MODE_HOST: u32 = 0x00000001;  // Dual mode controller is in Host
                                            // mode.
pub const DUAL_MODE_DEVICE: u32 = 0x00000081;  // Dual mode controller is in
                                            // Device mode.
pub const DUAL_MODE_NONE: u32 = 0x00000080;  // Dual mode controller mode is not
                                            // set.
pub const OTG_MODE_ASIDE_HOST: u32 = 0x0000001d;  // OTG controller on the A side of
                                            // the cable.
pub const OTG_MODE_ASIDE_NPWR: u32 = 0x00000001;  // OTG controller on the A side of
                                            // the cable.
pub const OTG_MODE_ASIDE_SESS: u32 = 0x00000009;  // OTG controller on the A side of
                                            // the cable Session Valid.
pub const OTG_MODE_ASIDE_AVAL: u32 = 0x00000011;  // OTG controller on the A side of
                                            // the cable A valid.
pub const OTG_MODE_ASIDE_DEV: u32 = 0x00000019;  // OTG controller on the A side of
                                            // the cable.
pub const OTG_MODE_BSIDE_HOST: u32 = 0x0000009d;  // OTG controller on the B side of
                                            // the cable.
pub const OTG_MODE_BSIDE_DEV: u32 = 0x00000099;  // OTG controller on the B side of
                                            // the cable.
pub const OTG_MODE_BSIDE_NPWR: u32 = 0x00000081;  // OTG controller on the B side of
                                            // the cable.
pub const OTG_MODE_NONE: u32 = 0x00000080;  // OTG controller mode is not set.

//*****************************************************************************
//
// The values for the USBDMAChannelIntEnable() and USBDMAChannelIntStatus()
// APIs.
//
//*****************************************************************************
pub const DMA_INT_CH8: u32 = 0x00000080;
pub const DMA_INT_CH7: u32 = 0x00000040;
pub const DMA_INT_CH6: u32 = 0x00000020;
pub const DMA_INT_CH5: u32 = 0x00000010;
pub const DMA_INT_CH4: u32 = 0x00000008;
pub const DMA_INT_CH3: u32 = 0x00000004;
pub const DMA_INT_CH2: u32 = 0x00000002;
pub const DMA_INT_CH1: u32 = 0x00000001;

//*****************************************************************************
//
// The values for the USBDMAChannelStatus() API.
//
//*****************************************************************************
pub const DMA_STATUS_ERROR: u32 = 0x00000100;

//*****************************************************************************
//
// The valid return values for the USBControllerVersion() API.
//
//*****************************************************************************
pub const CONTROLLER_VER_0: u32 = 0x00000000;  // This is for Blizzard class
                                            // devices.
pub const CONTROLLER_VER_1: u32 = 0x00000001;  // This is for Snowflake class
                                            // devices.

//*****************************************************************************
//
// The valid return values for the USBDMAModeSet() and USBDMAModeGet() APIs or
// USBDMAChannelConfig().
//
//*****************************************************************************
pub const DMA_CFG_BURST_NONE: u32 = 0x00000000;
pub const DMA_CFG_BURST_4: u32 = 0x00000200;
pub const DMA_CFG_BURST_8: u32 = 0x00000400;
pub const DMA_CFG_BURST_16: u32 = 0x00000600;
pub const DMA_CFG_INT_EN: u32 = 0x00000008;
pub const DMA_CFG_MODE_0: u32 = 0x00000000;
pub const DMA_CFG_MODE_1: u32 = 0x00000004;
pub const DMA_CFG_DIR_RX: u32 = 0x00000000;
pub const DMA_CFG_DIR_TX: u32 = 0x00000002;
pub const DMA_CFG_EN: u32 = 0x00000001;

//*****************************************************************************
//
// The following are values that can be passed to USBModeConfig() as the
// ui3Mode parameter.
//
//*****************************************************************************
pub const MODE_HOST_VBUS: u32 = 0x00000004;
pub const MODE_HOST: u32 = 0x00000002;
pub const MODE_DEV_VBUS: u32 = 0x00000005;
pub const MODE_DEV: u32 = 0x00000003;
pub const MODE_OTG: u32 = 0x00000000;

//*****************************************************************************
//
// Prototypes for the APIs.
//
//*****************************************************************************

pub fn UpdateUSB(bootrominfo: *mut u8) { unsafe {
    let func = getfun(16, 58) as *const extern "C" fn(*mut u8);
    (*func)(bootrominfo)
}}

pub fn DevAddrGet(base: u32) -> u32 { unsafe {
    let func = getfun(16, 1) as *const extern "C" fn(u32) -> u32;
    (*func)(base)
}}

pub fn DevAddrSet(base: u32, address: u32) { unsafe {
    let func = getfun(16, 2) as *const extern "C" fn(u32, u32);
    (*func)(base, address)
}}

pub fn DevConnect(base: u32) { unsafe {
    let func = getfun(16, 3) as *const extern "C" fn(u32);
    (*func)(base)
}}

pub fn DevDisconnect(base: u32) { unsafe {
    let func = getfun(16, 4) as *const extern "C" fn(u32);
    (*func)(base)
}}

pub fn DevEndpointConfigGet(base: u32, endpoint: u32, maxpacketsize: *mut u32, flags: *mut u32) { unsafe {
    let func = getfun(16, 41) as *const extern "C" fn(u32, u32, *mut u32, *mut u32);
    (*func)(base, endpoint, maxpacketsize, flags)
}}

pub fn DevEndpointConfigSet(base: u32, endpoint: u32, maxpacketsize: u32, flags: u32) { unsafe {
    let func = getfun(16, 5) as *const extern "C" fn(u32, u32, u32, u32);
    (*func)(base, endpoint, maxpacketsize, flags)
}}

pub fn DevEndpointDataAck(base: u32, endpoint: u32, last: bool) { unsafe {
    let func = getfun(16, 6) as *const extern "C" fn(u32, u32, u8);
    (*func)(base, endpoint, last as u8)
}}

pub fn DevEndpointStall(base: u32, endpoint: u32, flags: u32) { unsafe {
    let func = getfun(16, 7) as *const extern "C" fn(u32, u32, u32);
    (*func)(base, endpoint, flags)
}}

pub fn DevEndpointStallClear(base: u32, endpoint: u32, flags: u32) { unsafe {
    let func = getfun(16, 8) as *const extern "C" fn(u32, u32, u32);
    (*func)(base, endpoint, flags)
}}

pub fn DevEndpointStatusClear(base: u32, endpoint: u32, flags: u32) { unsafe {
    let func = getfun(16, 9) as *const extern "C" fn(u32, u32, u32);
    (*func)(base, endpoint, flags)
}}

pub fn DevMode(base: u32) { unsafe {
    let func = getfun(16, 55) as *const extern "C" fn(u32);
    (*func)(base)
}}

pub fn EndpointDataAvail(base: u32, endpoint: u32) -> u32 { unsafe {
    let func = getfun(16, 44) as *const extern "C" fn(u32, u32) -> u32;
    (*func)(base, endpoint)
}}

pub fn EndpointDataGet(base: u32, endpoint: u32, data: *mut u8, size: *mut u32) -> i32 { unsafe {
    let func = getfun(16, 10) as *const extern "C" fn(u32, u32, *mut u8, *mut u32) -> i32;
    (*func)(base, endpoint, data, size)
}}

pub fn EndpointDataPut(base: u32, endpoint: u32, data: *mut u8, size: u32) -> i32 { unsafe {
    let func = getfun(16, 11) as *const extern "C" fn(u32, u32, *mut u8, u32) -> i32;
    (*func)(base, endpoint, data, size)
}}

pub fn EndpointDataSend(base: u32, endpoint: u32, transtype: u32) -> i32 { unsafe {
    let func = getfun(16, 12) as *const extern "C" fn(u32, u32, u32) -> i32;
    (*func)(base, endpoint, transtype)
}}

pub fn EndpointDataToggleClear(base: u32, endpoint: u32, flags: u32) { unsafe {
    let func = getfun(16, 13) as *const extern "C" fn(u32, u32, u32);
    (*func)(base, endpoint, flags)
}}

pub fn EndpointDMAChannel(base: u32, endpoint: u32, channel: u32) { unsafe {
    let func = getfun(16, 47) as *const extern "C" fn(u32, u32, u32);
    (*func)(base, endpoint, channel)
}}

pub fn EndpointDMADisable(base: u32, endpoint: u32, flags: u32) { unsafe {
    let func = getfun(16, 43) as *const extern "C" fn(u32, u32, u32);
    (*func)(base, endpoint, flags)
}}

pub fn EndpointDMAEnable(base: u32, endpoint: u32, flags: u32) { unsafe {
    let func = getfun(16, 42) as *const extern "C" fn(u32, u32, u32);
    (*func)(base, endpoint, flags)
}}

pub fn EndpointStatus(base: u32, endpoint: u32) -> u32 { unsafe {
    let func = getfun(16, 14) as *const extern "C" fn(u32, u32) -> u32;
    (*func)(base, endpoint)
}}

/*
extern void USBEndpointDMAConfigSet(uint32_t ui32Base, uint32_t ui32Endpoint,
                                    uint32_t ui32Config);
extern void USBEndpointPacketCountSet(uint32_t ui32Base, uint32_t ui32Endpoint,
                                      uint32_t ui32Count);
*/

pub fn FIFOAddrGet(base: u32, endpoint: u32) -> u32 { unsafe {
    let func = getfun(16, 15) as *const extern "C" fn(u32, u32) -> u32;
    (*func)(base, endpoint)
}}

pub fn FIFOConfigGet(base: u32, endpoint: u32, address: *mut u32, size: *mut u32, flags: u32) { unsafe {
    let func = getfun(16, 16) as *const extern "C" fn(u32, u32, *mut u32, *mut u32, u32);
    (*func)(base, endpoint, address, size, flags)
}}

pub fn FIFOConfigSet(base: u32, endpoint: u32, address: u32, size: u32, flags: u32) { unsafe {
    let func = getfun(16, 17) as *const extern "C" fn(u32, u32, u32, u32, u32);
    (*func)(base, endpoint, address, size, flags)
}}

pub fn FIFOFlush(base: u32, endpoint: u32, flags: u32) { unsafe {
    let func = getfun(16, 18) as *const extern "C" fn(u32, u32, u32);
    (*func)(base, endpoint, flags)
}}

pub fn FrameNumberGet(base: u32) -> u32 { unsafe {
    let func = getfun(16, 19) as *const extern "C" fn(u32) -> u32;
    (*func)(base)
}}

/*
extern uint32_t USBHostAddrGet(uint32_t ui32Base, uint32_t ui32Endpoint,
                               uint32_t ui32Flags);
extern void USBHostAddrSet(uint32_t ui32Base, uint32_t ui32Endpoint,
                           uint32_t ui32Addr, uint32_t ui32Flags);
extern void USBHostEndpointConfig(uint32_t ui32Base, uint32_t ui32Endpoint,
                                  uint32_t ui32MaxPacketSize,
                                  uint32_t ui32NAKPollInterval,
                                  uint32_t ui32TargetEndpoint,
                                  uint32_t ui32Flags);
extern void USBHostEndpointDataAck(uint32_t ui32Base,
                                   uint32_t ui32Endpoint);
extern void USBHostEndpointDataToggle(uint32_t ui32Base, uint32_t ui32Endpoint,
                                      bool bDataToggle, uint32_t ui32Flags);
extern void USBHostEndpointStatusClear(uint32_t ui32Base,
                                       uint32_t ui32Endpoint,
                                       uint32_t ui32Flags);
extern uint32_t USBHostHubAddrGet(uint32_t ui32Base, uint32_t ui32Endpoint,
                                  uint32_t ui32Flags);
extern void USBHostHubAddrSet(uint32_t ui32Base, uint32_t ui32Endpoint,
                              uint32_t ui32Addr, uint32_t ui32Flags);
extern void USBHostPwrDisable(uint32_t ui32Base);
extern void USBHostPwrEnable(uint32_t ui32Base);
extern void USBHostPwrConfig(uint32_t ui32Base, uint32_t ui32Flags);
extern void USBHostPwrFaultDisable(uint32_t ui32Base);
extern void USBHostPwrFaultEnable(uint32_t ui32Base);
extern void USBHostRequestIN(uint32_t ui32Base, uint32_t ui32Endpoint);
extern void USBHostRequestINClear(uint32_t ui32Base, uint32_t ui32Endpoint);
extern void USBHostRequestStatus(uint32_t ui32Base);
extern void USBHostReset(uint32_t ui32Base, bool bStart);
extern void USBHostResume(uint32_t ui32Base, bool bStart);
extern uint32_t USBHostSpeedGet(uint32_t ui32Base);
extern void USBHostSuspend(uint32_t ui32Base);
*/

pub fn IntDisableControl(base: u32, endpoint: u32, flags: u32) { unsafe {
    let func = getfun(16, 48) as *const extern "C" fn(u32, u32, u32);
    (*func)(base, endpoint, flags)
}}

pub fn IntDisableEndpoint(base: u32, flags: u32) { unsafe {
    let func = getfun(16, 51) as *const extern "C" fn(u32, u32);
    (*func)(base, flags)
}}

pub fn IntEnableControl(base: u32, flags: u32) { unsafe {
    let func = getfun(16, 49) as *const extern "C" fn(u32, u32);
    (*func)(base, flags)
}}

pub fn IntEnableEndpoint(base: u32, flags: u32) { unsafe {
    let func = getfun(16, 52) as *const extern "C" fn(u32, u32);
    (*func)(base, flags)
}}

pub fn IntStatusControl(base: u32) -> u32 { unsafe {
    let func = getfun(16, 50) as *const extern "C" fn(u32) -> u32;
    (*func)(base)
}}

pub fn IntStatusEndpoint(base: u32) -> u32 { unsafe {
    let func = getfun(16, 53) as *const extern "C" fn(u32) -> u32;
    (*func)(base)
}}

/*
extern void USBIntRegister(uint32_t ui32Base, void (*pfnHandler)(void));
extern void USBIntUnregister(uint32_t ui32Base);
extern void USBOTGSessionRequest(uint32_t ui32Base, bool bStart);
*/

pub fn ModeGet(base: u32) -> u32 { unsafe {
    let func = getfun(16, 46) as *const extern "C" fn(u32) -> u32;
    (*func)(base)
}}

pub fn NumEndpointsGet(base: u32) -> u32 { unsafe {
    let func = getfun(16, 61) as *const extern "C" fn(u32) -> u32;
    (*func)(base)
}}

pub fn OTGMode(base: u32) { unsafe {
    let func = getfun(16, 59) as *const extern "C" fn(u32);
    (*func)(base)
}}

pub fn PHYPowerOff(base: u32) { unsafe {
    let func = getfun(16, 56) as *const extern "C" fn(u32);
    (*func)(base)
}}

pub fn PHYPowerOn(base: u32) { unsafe {
    let func = getfun(16, 57) as *const extern "C" fn(u32);
    (*func)(base)
}}


/*
extern uint32_t USBControllerVersion(uint32_t ui32Base);
extern uint32_t USBDMAChannelIntStatus(uint32_t ui32Base);
extern void USBDMAChannelConfigSet(uint32_t ui32Base, uint32_t ui32Channel,
                                   uint32_t ui32Endpoint, uint32_t ui32Config);
extern void USBDMAChannelAddressSet(uint32_t ui32Base, uint32_t ui32Channel,
                                    void *pvAddress);
extern void *USBDMAChannelAddressGet(uint32_t ui32Base, uint32_t ui32Channel);
extern void USBDMAChannelCountSet(uint32_t ui32Base, uint32_t ui32Count,
                                  uint32_t ui32Channel);
extern uint32_t USBDMAChannelCountGet(uint32_t ui32Base, uint32_t ui32Channel);
extern uint32_t USBDMANumChannels(uint32_t ui32Base);
extern void USBDMAChannelAssign(uint32_t ui32Base, uint32_t ui32Endpoint,
                                uint32_t ui32Channel, uint32_t ui32Flags);
extern void USBDMAChannelIntEnable(uint32_t ui32Base, uint32_t ui32Channel);
extern void USBDMAChannelIntDisable(uint32_t ui32Base, uint32_t ui32Channel);
extern void USBDMAChannelEnable(uint32_t ui32Base, uint32_t ui32Channel);
extern void USBDMAChannelDisable(uint32_t ui32Base, uint32_t ui32Channel);
extern uint32_t USBDMAChannelStatus(uint32_t ui32Base, uint32_t ui32Channel);
extern void USBDMAChannelStatusClear(uint32_t ui32Base, uint32_t ui32Channel,
                                     uint32_t ui32Status);
extern void USBHostEndpointSpeed(uint32_t ui32Base, uint32_t ui32Endpoint,
                                 uint32_t ui32Flags);
extern void USBHostEndpointPing(uint32_t ui32Base, uint32_t ui32Endpoint,
                                bool bEnable);
extern void USBHostLPMSend(uint32_t ui32Base, uint32_t ui32Address,
                           uint32_t uiEndpoint);
extern void USBHostLPMConfig(uint32_t ui32Base, uint32_t ui32ResumeTime,
                             uint32_t ui32Config);
extern bool USBLPMRemoteWakeEnabled(uint32_t ui32Base);
extern void USBHostLPMResume(uint32_t ui32Base);
extern void USBDevLPMRemoteWake(uint32_t ui32Base);
extern void USBDevLPMConfig(uint32_t ui32Base, uint32_t ui32Config);
extern void USBDevLPMEnable(uint32_t ui32Base);
extern void USBDevLPMDisable(uint32_t ui32Base);
extern uint32_t USBLPMLinkStateGet(uint32_t ui32Base);
extern uint32_t USBLPMEndpointGet(uint32_t ui32Base);
extern uint32_t USBLPMIntStatus(uint32_t ui32Base);
extern void USBLPMIntDisable(uint32_t ui32Base, uint32_t ui32Ints);
extern void USBLPMIntEnable(uint32_t ui32Base, uint32_t ui32Ints);
extern void USBHighSpeed(uint32_t ui32Base, bool bEnable);
extern uint32_t USBDevSpeedGet(uint32_t ui32Base);
extern void USBClockEnable(uint32_t ui32Base, uint32_t ui32Div,
                           uint32_t ui32Flags);
extern void USBClockDisable(uint32_t ui32Base);
extern void USBULPIConfig(uint32_t ui32Base, uint32_t ui32Config);
extern void USBULPIEnable(uint32_t ui32Base);
extern void USBULPIDisable(uint32_t ui32Base);
extern uint8_t USBULPIRegRead(uint32_t ui32Base, uint8_t ui8Reg);
extern void USBULPIRegWrite(uint32_t ui32Base, uint8_t ui8Reg,
                            uint8_t ui8Data);
extern void USBHostMode(uint32_t ui32Base);
extern void USBDevMode(uint32_t ui32Base);
extern void USBModeConfig(uint32_t ui32Base, uint32_t ui32Mode);
*/
