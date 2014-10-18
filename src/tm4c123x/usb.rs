//*****************************************************************************
//
// The following are defines for the Univeral Serial Bus register offsets.
//
//*****************************************************************************
pub const O_FADDR: uint = 0x00000000;  // USB Device Functional Address
pub const O_POWER: uint = 0x00000001;  // USB Power
pub const O_TXIS: uint = 0x00000002;  // USB Transmit Interrupt Status
pub const O_RXIS: uint = 0x00000004;  // USB Receive Interrupt Status
pub const O_TXIE: uint = 0x00000006;  // USB Transmit Interrupt Enable
pub const O_RXIE: uint = 0x00000008;  // USB Receive Interrupt Enable
pub const O_IS: uint = 0x0000000A;  // USB General Interrupt Status
pub const O_IE: uint = 0x0000000B;  // USB Interrupt Enable
pub const O_FRAME: uint = 0x0000000C;  // USB Frame Value
pub const O_EPIDX: uint = 0x0000000E;  // USB Endpoint Index
pub const O_TEST: uint = 0x0000000F;  // USB Test Mode
pub const O_FIFO0: uint = 0x00000020;  // USB FIFO Endpoint 0
pub const O_FIFO1: uint = 0x00000024;  // USB FIFO Endpoint 1
pub const O_FIFO2: uint = 0x00000028;  // USB FIFO Endpoint 2
pub const O_FIFO3: uint = 0x0000002C;  // USB FIFO Endpoint 3
pub const O_FIFO4: uint = 0x00000030;  // USB FIFO Endpoint 4
pub const O_FIFO5: uint = 0x00000034;  // USB FIFO Endpoint 5
pub const O_FIFO6: uint = 0x00000038;  // USB FIFO Endpoint 6
pub const O_FIFO7: uint = 0x0000003C;  // USB FIFO Endpoint 7
pub const O_DEVCTL: uint = 0x00000060;  // USB Device Control
pub const O_CCONF: uint = 0x00000061;  // USB Common Configuration
pub const O_TXFIFOSZ: uint = 0x00000062;  // USB Transmit Dynamic FIFO Sizing
pub const O_RXFIFOSZ: uint = 0x00000063;  // USB Receive Dynamic FIFO Sizing
pub const O_TXFIFOADD: uint = 0x00000064;  // USB Transmit FIFO Start Address
pub const O_RXFIFOADD: uint = 0x00000066;  // USB Receive FIFO Start Address
pub const O_ULPIVBUSCTL: uint = 0x00000070;  // USB ULPI VBUS Control
pub const O_ULPIREGDATA: uint = 0x00000074;  // USB ULPI Register Data
pub const O_ULPIREGADDR: uint = 0x00000075;  // USB ULPI Register Address
pub const O_ULPIREGCTL: uint = 0x00000076;  // USB ULPI Register Control
pub const O_EPINFO: uint = 0x00000078;  // USB Endpoint Information
pub const O_RAMINFO: uint = 0x00000079;  // USB RAM Information
pub const O_CONTIM: uint = 0x0000007A;  // USB Connect Timing
pub const O_VPLEN: uint = 0x0000007B;  // USB OTG VBUS Pulse Timing
pub const O_HSEOF: uint = 0x0000007C;  // USB High-Speed Last Transaction
                                            // to End of Frame Timing
pub const O_FSEOF: uint = 0x0000007D;  // USB Full-Speed Last Transaction
                                            // to End of Frame Timing
pub const O_LSEOF: uint = 0x0000007E;  // USB Low-Speed Last Transaction
                                            // to End of Frame Timing
pub const O_TXFUNCADDR0: uint = 0x00000080;  // USB Transmit Functional Address
                                            // Endpoint 0
pub const O_TXHUBADDR0: uint = 0x00000082;  // USB Transmit Hub Address
                                            // Endpoint 0
pub const O_TXHUBPORT0: uint = 0x00000083;  // USB Transmit Hub Port Endpoint 0
pub const O_TXFUNCADDR1: uint = 0x00000088;  // USB Transmit Functional Address
                                            // Endpoint 1
pub const O_TXHUBADDR1: uint = 0x0000008A;  // USB Transmit Hub Address
                                            // Endpoint 1
pub const O_TXHUBPORT1: uint = 0x0000008B;  // USB Transmit Hub Port Endpoint 1
pub const O_RXFUNCADDR1: uint = 0x0000008C;  // USB Receive Functional Address
                                            // Endpoint 1
pub const O_RXHUBADDR1: uint = 0x0000008E;  // USB Receive Hub Address Endpoint
                                            // 1
pub const O_RXHUBPORT1: uint = 0x0000008F;  // USB Receive Hub Port Endpoint 1
pub const O_TXFUNCADDR2: uint = 0x00000090;  // USB Transmit Functional Address
                                            // Endpoint 2
pub const O_TXHUBADDR2: uint = 0x00000092;  // USB Transmit Hub Address
                                            // Endpoint 2
pub const O_TXHUBPORT2: uint = 0x00000093;  // USB Transmit Hub Port Endpoint 2
pub const O_RXFUNCADDR2: uint = 0x00000094;  // USB Receive Functional Address
                                            // Endpoint 2
pub const O_RXHUBADDR2: uint = 0x00000096;  // USB Receive Hub Address Endpoint
                                            // 2
pub const O_RXHUBPORT2: uint = 0x00000097;  // USB Receive Hub Port Endpoint 2
pub const O_TXFUNCADDR3: uint = 0x00000098;  // USB Transmit Functional Address
                                            // Endpoint 3
pub const O_TXHUBADDR3: uint = 0x0000009A;  // USB Transmit Hub Address
                                            // Endpoint 3
pub const O_TXHUBPORT3: uint = 0x0000009B;  // USB Transmit Hub Port Endpoint 3
pub const O_RXFUNCADDR3: uint = 0x0000009C;  // USB Receive Functional Address
                                            // Endpoint 3
pub const O_RXHUBADDR3: uint = 0x0000009E;  // USB Receive Hub Address Endpoint
                                            // 3
pub const O_RXHUBPORT3: uint = 0x0000009F;  // USB Receive Hub Port Endpoint 3
pub const O_TXFUNCADDR4: uint = 0x000000A0;  // USB Transmit Functional Address
                                            // Endpoint 4
pub const O_TXHUBADDR4: uint = 0x000000A2;  // USB Transmit Hub Address
                                            // Endpoint 4
pub const O_TXHUBPORT4: uint = 0x000000A3;  // USB Transmit Hub Port Endpoint 4
pub const O_RXFUNCADDR4: uint = 0x000000A4;  // USB Receive Functional Address
                                            // Endpoint 4
pub const O_RXHUBADDR4: uint = 0x000000A6;  // USB Receive Hub Address Endpoint
                                            // 4
pub const O_RXHUBPORT4: uint = 0x000000A7;  // USB Receive Hub Port Endpoint 4
pub const O_TXFUNCADDR5: uint = 0x000000A8;  // USB Transmit Functional Address
                                            // Endpoint 5
pub const O_TXHUBADDR5: uint = 0x000000AA;  // USB Transmit Hub Address
                                            // Endpoint 5
pub const O_TXHUBPORT5: uint = 0x000000AB;  // USB Transmit Hub Port Endpoint 5
pub const O_RXFUNCADDR5: uint = 0x000000AC;  // USB Receive Functional Address
                                            // Endpoint 5
pub const O_RXHUBADDR5: uint = 0x000000AE;  // USB Receive Hub Address Endpoint
                                            // 5
pub const O_RXHUBPORT5: uint = 0x000000AF;  // USB Receive Hub Port Endpoint 5
pub const O_TXFUNCADDR6: uint = 0x000000B0;  // USB Transmit Functional Address
                                            // Endpoint 6
pub const O_TXHUBADDR6: uint = 0x000000B2;  // USB Transmit Hub Address
                                            // Endpoint 6
pub const O_TXHUBPORT6: uint = 0x000000B3;  // USB Transmit Hub Port Endpoint 6
pub const O_RXFUNCADDR6: uint = 0x000000B4;  // USB Receive Functional Address
                                            // Endpoint 6
pub const O_RXHUBADDR6: uint = 0x000000B6;  // USB Receive Hub Address Endpoint
                                            // 6
pub const O_RXHUBPORT6: uint = 0x000000B7;  // USB Receive Hub Port Endpoint 6
pub const O_TXFUNCADDR7: uint = 0x000000B8;  // USB Transmit Functional Address
                                            // Endpoint 7
pub const O_TXHUBADDR7: uint = 0x000000BA;  // USB Transmit Hub Address
                                            // Endpoint 7
pub const O_TXHUBPORT7: uint = 0x000000BB;  // USB Transmit Hub Port Endpoint 7
pub const O_RXFUNCADDR7: uint = 0x000000BC;  // USB Receive Functional Address
                                            // Endpoint 7
pub const O_RXHUBADDR7: uint = 0x000000BE;  // USB Receive Hub Address Endpoint
                                            // 7
pub const O_RXHUBPORT7: uint = 0x000000BF;  // USB Receive Hub Port Endpoint 7
pub const O_CSRL0: uint = 0x00000102;  // USB Control and Status Endpoint
                                            // 0 Low
pub const O_CSRH0: uint = 0x00000103;  // USB Control and Status Endpoint
                                            // 0 High
pub const O_COUNT0: uint = 0x00000108;  // USB Receive Byte Count Endpoint
                                            // 0
pub const O_TYPE0: uint = 0x0000010A;  // USB Type Endpoint 0
pub const O_NAKLMT: uint = 0x0000010B;  // USB NAK Limit
pub const O_TXMAXP1: uint = 0x00000110;  // USB Maximum Transmit Data
                                            // Endpoint 1
pub const O_TXCSRL1: uint = 0x00000112;  // USB Transmit Control and Status
                                            // Endpoint 1 Low
pub const O_TXCSRH1: uint = 0x00000113;  // USB Transmit Control and Status
                                            // Endpoint 1 High
pub const O_RXMAXP1: uint = 0x00000114;  // USB Maximum Receive Data
                                            // Endpoint 1
pub const O_RXCSRL1: uint = 0x00000116;  // USB Receive Control and Status
                                            // Endpoint 1 Low
pub const O_RXCSRH1: uint = 0x00000117;  // USB Receive Control and Status
                                            // Endpoint 1 High
pub const O_RXCOUNT1: uint = 0x00000118;  // USB Receive Byte Count Endpoint
                                            // 1
pub const O_TXTYPE1: uint = 0x0000011A;  // USB Host Transmit Configure Type
                                            // Endpoint 1
pub const O_TXINTERVAL1: uint = 0x0000011B;  // USB Host Transmit Interval
                                            // Endpoint 1
pub const O_RXTYPE1: uint = 0x0000011C;  // USB Host Configure Receive Type
                                            // Endpoint 1
pub const O_RXINTERVAL1: uint = 0x0000011D;  // USB Host Receive Polling
                                            // Interval Endpoint 1
pub const O_TXMAXP2: uint = 0x00000120;  // USB Maximum Transmit Data
                                            // Endpoint 2
pub const O_TXCSRL2: uint = 0x00000122;  // USB Transmit Control and Status
                                            // Endpoint 2 Low
pub const O_TXCSRH2: uint = 0x00000123;  // USB Transmit Control and Status
                                            // Endpoint 2 High
pub const O_RXMAXP2: uint = 0x00000124;  // USB Maximum Receive Data
                                            // Endpoint 2
pub const O_RXCSRL2: uint = 0x00000126;  // USB Receive Control and Status
                                            // Endpoint 2 Low
pub const O_RXCSRH2: uint = 0x00000127;  // USB Receive Control and Status
                                            // Endpoint 2 High
pub const O_RXCOUNT2: uint = 0x00000128;  // USB Receive Byte Count Endpoint
                                            // 2
pub const O_TXTYPE2: uint = 0x0000012A;  // USB Host Transmit Configure Type
                                            // Endpoint 2
pub const O_TXINTERVAL2: uint = 0x0000012B;  // USB Host Transmit Interval
                                            // Endpoint 2
pub const O_RXTYPE2: uint = 0x0000012C;  // USB Host Configure Receive Type
                                            // Endpoint 2
pub const O_RXINTERVAL2: uint = 0x0000012D;  // USB Host Receive Polling
                                            // Interval Endpoint 2
pub const O_TXMAXP3: uint = 0x00000130;  // USB Maximum Transmit Data
                                            // Endpoint 3
pub const O_TXCSRL3: uint = 0x00000132;  // USB Transmit Control and Status
                                            // Endpoint 3 Low
pub const O_TXCSRH3: uint = 0x00000133;  // USB Transmit Control and Status
                                            // Endpoint 3 High
pub const O_RXMAXP3: uint = 0x00000134;  // USB Maximum Receive Data
                                            // Endpoint 3
pub const O_RXCSRL3: uint = 0x00000136;  // USB Receive Control and Status
                                            // Endpoint 3 Low
pub const O_RXCSRH3: uint = 0x00000137;  // USB Receive Control and Status
                                            // Endpoint 3 High
pub const O_RXCOUNT3: uint = 0x00000138;  // USB Receive Byte Count Endpoint
                                            // 3
pub const O_TXTYPE3: uint = 0x0000013A;  // USB Host Transmit Configure Type
                                            // Endpoint 3
pub const O_TXINTERVAL3: uint = 0x0000013B;  // USB Host Transmit Interval
                                            // Endpoint 3
pub const O_RXTYPE3: uint = 0x0000013C;  // USB Host Configure Receive Type
                                            // Endpoint 3
pub const O_RXINTERVAL3: uint = 0x0000013D;  // USB Host Receive Polling
                                            // Interval Endpoint 3
pub const O_TXMAXP4: uint = 0x00000140;  // USB Maximum Transmit Data
                                            // Endpoint 4
pub const O_TXCSRL4: uint = 0x00000142;  // USB Transmit Control and Status
                                            // Endpoint 4 Low
pub const O_TXCSRH4: uint = 0x00000143;  // USB Transmit Control and Status
                                            // Endpoint 4 High
pub const O_RXMAXP4: uint = 0x00000144;  // USB Maximum Receive Data
                                            // Endpoint 4
pub const O_RXCSRL4: uint = 0x00000146;  // USB Receive Control and Status
                                            // Endpoint 4 Low
pub const O_RXCSRH4: uint = 0x00000147;  // USB Receive Control and Status
                                            // Endpoint 4 High
pub const O_RXCOUNT4: uint = 0x00000148;  // USB Receive Byte Count Endpoint
                                            // 4
pub const O_TXTYPE4: uint = 0x0000014A;  // USB Host Transmit Configure Type
                                            // Endpoint 4
pub const O_TXINTERVAL4: uint = 0x0000014B;  // USB Host Transmit Interval
                                            // Endpoint 4
pub const O_RXTYPE4: uint = 0x0000014C;  // USB Host Configure Receive Type
                                            // Endpoint 4
pub const O_RXINTERVAL4: uint = 0x0000014D;  // USB Host Receive Polling
                                            // Interval Endpoint 4
pub const O_TXMAXP5: uint = 0x00000150;  // USB Maximum Transmit Data
                                            // Endpoint 5
pub const O_TXCSRL5: uint = 0x00000152;  // USB Transmit Control and Status
                                            // Endpoint 5 Low
pub const O_TXCSRH5: uint = 0x00000153;  // USB Transmit Control and Status
                                            // Endpoint 5 High
pub const O_RXMAXP5: uint = 0x00000154;  // USB Maximum Receive Data
                                            // Endpoint 5
pub const O_RXCSRL5: uint = 0x00000156;  // USB Receive Control and Status
                                            // Endpoint 5 Low
pub const O_RXCSRH5: uint = 0x00000157;  // USB Receive Control and Status
                                            // Endpoint 5 High
pub const O_RXCOUNT5: uint = 0x00000158;  // USB Receive Byte Count Endpoint
                                            // 5
pub const O_TXTYPE5: uint = 0x0000015A;  // USB Host Transmit Configure Type
                                            // Endpoint 5
pub const O_TXINTERVAL5: uint = 0x0000015B;  // USB Host Transmit Interval
                                            // Endpoint 5
pub const O_RXTYPE5: uint = 0x0000015C;  // USB Host Configure Receive Type
                                            // Endpoint 5
pub const O_RXINTERVAL5: uint = 0x0000015D;  // USB Host Receive Polling
                                            // Interval Endpoint 5
pub const O_TXMAXP6: uint = 0x00000160;  // USB Maximum Transmit Data
                                            // Endpoint 6
pub const O_TXCSRL6: uint = 0x00000162;  // USB Transmit Control and Status
                                            // Endpoint 6 Low
pub const O_TXCSRH6: uint = 0x00000163;  // USB Transmit Control and Status
                                            // Endpoint 6 High
pub const O_RXMAXP6: uint = 0x00000164;  // USB Maximum Receive Data
                                            // Endpoint 6
pub const O_RXCSRL6: uint = 0x00000166;  // USB Receive Control and Status
                                            // Endpoint 6 Low
pub const O_RXCSRH6: uint = 0x00000167;  // USB Receive Control and Status
                                            // Endpoint 6 High
pub const O_RXCOUNT6: uint = 0x00000168;  // USB Receive Byte Count Endpoint
                                            // 6
pub const O_TXTYPE6: uint = 0x0000016A;  // USB Host Transmit Configure Type
                                            // Endpoint 6
pub const O_TXINTERVAL6: uint = 0x0000016B;  // USB Host Transmit Interval
                                            // Endpoint 6
pub const O_RXTYPE6: uint = 0x0000016C;  // USB Host Configure Receive Type
                                            // Endpoint 6
pub const O_RXINTERVAL6: uint = 0x0000016D;  // USB Host Receive Polling
                                            // Interval Endpoint 6
pub const O_TXMAXP7: uint = 0x00000170;  // USB Maximum Transmit Data
                                            // Endpoint 7
pub const O_TXCSRL7: uint = 0x00000172;  // USB Transmit Control and Status
                                            // Endpoint 7 Low
pub const O_TXCSRH7: uint = 0x00000173;  // USB Transmit Control and Status
                                            // Endpoint 7 High
pub const O_RXMAXP7: uint = 0x00000174;  // USB Maximum Receive Data
                                            // Endpoint 7
pub const O_RXCSRL7: uint = 0x00000176;  // USB Receive Control and Status
                                            // Endpoint 7 Low
pub const O_RXCSRH7: uint = 0x00000177;  // USB Receive Control and Status
                                            // Endpoint 7 High
pub const O_RXCOUNT7: uint = 0x00000178;  // USB Receive Byte Count Endpoint
                                            // 7
pub const O_TXTYPE7: uint = 0x0000017A;  // USB Host Transmit Configure Type
                                            // Endpoint 7
pub const O_TXINTERVAL7: uint = 0x0000017B;  // USB Host Transmit Interval
                                            // Endpoint 7
pub const O_RXTYPE7: uint = 0x0000017C;  // USB Host Configure Receive Type
                                            // Endpoint 7
pub const O_RXINTERVAL7: uint = 0x0000017D;  // USB Host Receive Polling
                                            // Interval Endpoint 7
pub const O_DMAINTR: uint = 0x00000200;  // USB DMA Interrupt
pub const O_DMACTL0: uint = 0x00000204;  // USB DMA Control 0
pub const O_DMAADDR0: uint = 0x00000208;  // USB DMA Address 0
pub const O_DMACOUNT0: uint = 0x0000020C;  // USB DMA Count 0
pub const O_DMACTL1: uint = 0x00000214;  // USB DMA Control 1
pub const O_DMAADDR1: uint = 0x00000218;  // USB DMA Address 1
pub const O_DMACOUNT1: uint = 0x0000021C;  // USB DMA Count 1
pub const O_DMACTL2: uint = 0x00000224;  // USB DMA Control 2
pub const O_DMAADDR2: uint = 0x00000228;  // USB DMA Address 2
pub const O_DMACOUNT2: uint = 0x0000022C;  // USB DMA Count 2
pub const O_DMACTL3: uint = 0x00000234;  // USB DMA Control 3
pub const O_DMAADDR3: uint = 0x00000238;  // USB DMA Address 3
pub const O_DMACOUNT3: uint = 0x0000023C;  // USB DMA Count 3
pub const O_DMACTL4: uint = 0x00000244;  // USB DMA Control 4
pub const O_DMAADDR4: uint = 0x00000248;  // USB DMA Address 4
pub const O_DMACOUNT4: uint = 0x0000024C;  // USB DMA Count 4
pub const O_DMACTL5: uint = 0x00000254;  // USB DMA Control 5
pub const O_DMAADDR5: uint = 0x00000258;  // USB DMA Address 5
pub const O_DMACOUNT5: uint = 0x0000025C;  // USB DMA Count 5
pub const O_DMACTL6: uint = 0x00000264;  // USB DMA Control 6
pub const O_DMAADDR6: uint = 0x00000268;  // USB DMA Address 6
pub const O_DMACOUNT6: uint = 0x0000026C;  // USB DMA Count 6
pub const O_DMACTL7: uint = 0x00000274;  // USB DMA Control 7
pub const O_DMAADDR7: uint = 0x00000278;  // USB DMA Address 7
pub const O_DMACOUNT7: uint = 0x0000027C;  // USB DMA Count 7
pub const O_RQPKTCOUNT1: uint = 0x00000304;  // USB Request Packet Count in
                                            // Block Transfer Endpoint 1
pub const O_RQPKTCOUNT2: uint = 0x00000308;  // USB Request Packet Count in
                                            // Block Transfer Endpoint 2
pub const O_RQPKTCOUNT3: uint = 0x0000030C;  // USB Request Packet Count in
                                            // Block Transfer Endpoint 3
pub const O_RQPKTCOUNT4: uint = 0x00000310;  // USB Request Packet Count in
                                            // Block Transfer Endpoint 4
pub const O_RQPKTCOUNT5: uint = 0x00000314;  // USB Request Packet Count in
                                            // Block Transfer Endpoint 5
pub const O_RQPKTCOUNT6: uint = 0x00000318;  // USB Request Packet Count in
                                            // Block Transfer Endpoint 6
pub const O_RQPKTCOUNT7: uint = 0x0000031C;  // USB Request Packet Count in
                                            // Block Transfer Endpoint 7
pub const O_RXDPKTBUFDIS: uint = 0x00000340;  // USB Receive Double Packet Buffer
                                            // Disable
pub const O_TXDPKTBUFDIS: uint = 0x00000342;  // USB Transmit Double Packet
                                            // Buffer Disable
pub const O_CTO: uint = 0x00000344;  // USB Chirp Timeout
pub const O_HHSRTN: uint = 0x00000346;  // USB High Speed to UTM Operating
                                            // Delay
pub const O_HSBT: uint = 0x00000348;  // USB High Speed Time-out Adder
pub const O_LPMATTR: uint = 0x00000360;  // USB LPM Attributes
pub const O_LPMCNTRL: uint = 0x00000362;  // USB LPM Control
pub const O_LPMIM: uint = 0x00000363;  // USB LPM Interrupt Mask
pub const O_LPMRIS: uint = 0x00000364;  // USB LPM Raw Interrupt Status
pub const O_LPMFADDR: uint = 0x00000365;  // USB LPM Function Address
pub const O_EPC: uint = 0x00000400;  // USB External Power Control
pub const O_EPCRIS: uint = 0x00000404;  // USB External Power Control Raw
                                            // Interrupt Status
pub const O_EPCIM: uint = 0x00000408;  // USB External Power Control
                                            // Interrupt Mask
pub const O_EPCISC: uint = 0x0000040C;  // USB External Power Control
                                            // Interrupt Status and Clear
pub const O_DRRIS: uint = 0x00000410;  // USB Device RESUME Raw Interrupt
                                            // Status
pub const O_DRIM: uint = 0x00000414;  // USB Device RESUME Interrupt Mask
pub const O_DRISC: uint = 0x00000418;  // USB Device RESUME Interrupt
                                            // Status and Clear
pub const O_GPCS: uint = 0x0000041C;  // USB General-Purpose Control and
                                            // Status
pub const O_VDC: uint = 0x00000430;  // USB VBUS Droop Control
pub const O_VDCRIS: uint = 0x00000434;  // USB VBUS Droop Control Raw
                                            // Interrupt Status
pub const O_VDCIM: uint = 0x00000438;  // USB VBUS Droop Control Interrupt
                                            // Mask
pub const O_VDCISC: uint = 0x0000043C;  // USB VBUS Droop Control Interrupt
                                            // Status and Clear
pub const O_IDVRIS: uint = 0x00000444;  // USB ID Valid Detect Raw
                                            // Interrupt Status
pub const O_IDVIM: uint = 0x00000448;  // USB ID Valid Detect Interrupt
                                            // Mask
pub const O_IDVISC: uint = 0x0000044C;  // USB ID Valid Detect Interrupt
                                            // Status and Clear
pub const O_DMASEL: uint = 0x00000450;  // USB DMA Select
pub const O_PP: uint = 0x00000FC0;  // USB Peripheral Properties
pub const O_PC: uint = 0x00000FC4;  // USB Peripheral Configuration
pub const O_CC: uint = 0x00000FC8;  // USB Clock Configuration

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_FADDR register.
//
//*****************************************************************************
pub const FADDR_M: u32 = 0x0000007F;  // Function Address
pub const FADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_POWER register.
//
//*****************************************************************************
pub const POWER_ISOUP: u32 = 0x00000080;  // Isochronous Update
pub const POWER_SOFTCONN: u32 = 0x00000040;  // Soft Connect/Disconnect
pub const POWER_HSENAB: u32 = 0x00000020;  // High Speed Enable
pub const POWER_HSMODE: u32 = 0x00000010;  // High Speed Enable
pub const POWER_RESET: u32 = 0x00000008;  // RESET Signaling
pub const POWER_RESUME: u32 = 0x00000004;  // RESUME Signaling
pub const POWER_SUSPEND: u32 = 0x00000002;  // SUSPEND Mode
pub const POWER_PWRDNPHY: u32 = 0x00000001;  // Power Down PHY

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXIS register.
//
//*****************************************************************************
pub const TXIS_EP7: u32 = 0x00000080;  // TX Endpoint 7 Interrupt
pub const TXIS_EP6: u32 = 0x00000040;  // TX Endpoint 6 Interrupt
pub const TXIS_EP5: u32 = 0x00000020;  // TX Endpoint 5 Interrupt
pub const TXIS_EP4: u32 = 0x00000010;  // TX Endpoint 4 Interrupt
pub const TXIS_EP3: u32 = 0x00000008;  // TX Endpoint 3 Interrupt
pub const TXIS_EP2: u32 = 0x00000004;  // TX Endpoint 2 Interrupt
pub const TXIS_EP1: u32 = 0x00000002;  // TX Endpoint 1 Interrupt
pub const TXIS_EP0: u32 = 0x00000001;  // TX and RX Endpoint 0 Interrupt

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXIS register.
//
//*****************************************************************************
pub const RXIS_EP7: u32 = 0x00000080;  // RX Endpoint 7 Interrupt
pub const RXIS_EP6: u32 = 0x00000040;  // RX Endpoint 6 Interrupt
pub const RXIS_EP5: u32 = 0x00000020;  // RX Endpoint 5 Interrupt
pub const RXIS_EP4: u32 = 0x00000010;  // RX Endpoint 4 Interrupt
pub const RXIS_EP3: u32 = 0x00000008;  // RX Endpoint 3 Interrupt
pub const RXIS_EP2: u32 = 0x00000004;  // RX Endpoint 2 Interrupt
pub const RXIS_EP1: u32 = 0x00000002;  // RX Endpoint 1 Interrupt

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXIE register.
//
//*****************************************************************************
pub const TXIE_EP7: u32 = 0x00000080;  // TX Endpoint 7 Interrupt Enable
pub const TXIE_EP6: u32 = 0x00000040;  // TX Endpoint 6 Interrupt Enable
pub const TXIE_EP5: u32 = 0x00000020;  // TX Endpoint 5 Interrupt Enable
pub const TXIE_EP4: u32 = 0x00000010;  // TX Endpoint 4 Interrupt Enable
pub const TXIE_EP3: u32 = 0x00000008;  // TX Endpoint 3 Interrupt Enable
pub const TXIE_EP2: u32 = 0x00000004;  // TX Endpoint 2 Interrupt Enable
pub const TXIE_EP1: u32 = 0x00000002;  // TX Endpoint 1 Interrupt Enable
pub const TXIE_EP0: u32 = 0x00000001;  // TX and RX Endpoint 0 Interrupt
                                            // Enable

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXIE register.
//
//*****************************************************************************
pub const RXIE_EP7: u32 = 0x00000080;  // RX Endpoint 7 Interrupt Enable
pub const RXIE_EP6: u32 = 0x00000040;  // RX Endpoint 6 Interrupt Enable
pub const RXIE_EP5: u32 = 0x00000020;  // RX Endpoint 5 Interrupt Enable
pub const RXIE_EP4: u32 = 0x00000010;  // RX Endpoint 4 Interrupt Enable
pub const RXIE_EP3: u32 = 0x00000008;  // RX Endpoint 3 Interrupt Enable
pub const RXIE_EP2: u32 = 0x00000004;  // RX Endpoint 2 Interrupt Enable
pub const RXIE_EP1: u32 = 0x00000002;  // RX Endpoint 1 Interrupt Enable

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_IS register.
//
//*****************************************************************************
pub const IS_VBUSERR: u32 = 0x00000080;  // VBUS Error (OTG only)
pub const IS_SESREQ: u32 = 0x00000040;  // SESSION REQUEST (OTG only)
pub const IS_DISCON: u32 = 0x00000020;  // Session Disconnect (OTG only)
pub const IS_CONN: u32 = 0x00000010;  // Session Connect
pub const IS_SOF: u32 = 0x00000008;  // Start of Frame
pub const IS_BABBLE: u32 = 0x00000004;  // Babble Detected
pub const IS_RESET: u32 = 0x00000004;  // RESET Signaling Detected
pub const IS_RESUME: u32 = 0x00000002;  // RESUME Signaling Detected
pub const IS_SUSPEND: u32 = 0x00000001;  // SUSPEND Signaling Detected

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_IE register.
//
//*****************************************************************************
pub const IE_VBUSERR: u32 = 0x00000080;  // Enable VBUS Error Interrupt (OTG
                                            // only)
pub const IE_SESREQ: u32 = 0x00000040;  // Enable Session Request (OTG
                                            // only)
pub const IE_DISCON: u32 = 0x00000020;  // Enable Disconnect Interrupt
pub const IE_CONN: u32 = 0x00000010;  // Enable Connect Interrupt
pub const IE_SOF: u32 = 0x00000008;  // Enable Start-of-Frame Interrupt
pub const IE_BABBLE: u32 = 0x00000004;  // Enable Babble Interrupt
pub const IE_RESET: u32 = 0x00000004;  // Enable RESET Interrupt
pub const IE_RESUME: u32 = 0x00000002;  // Enable RESUME Interrupt
pub const IE_SUSPND: u32 = 0x00000001;  // Enable SUSPEND Interrupt

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_FRAME register.
//
//*****************************************************************************
pub const FRAME_M: u32 = 0x000007FF;  // Frame Number
pub const FRAME_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_EPIDX register.
//
//*****************************************************************************
pub const EPIDX_EPIDX_M: u32 = 0x0000000F;  // Endpoint Index
pub const EPIDX_EPIDX_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TEST register.
//
//*****************************************************************************
pub const TEST_FORCEH: u32 = 0x00000080;  // Force Host Mode
pub const TEST_FIFOACC: u32 = 0x00000040;  // FIFO Access
pub const TEST_FORCEFS: u32 = 0x00000020;  // Force Full-Speed Mode
pub const TEST_FORCEHS: u32 = 0x00000010;  // Force High-Speed Mode
pub const TEST_TESTPKT: u32 = 0x00000008;  // Test Packet Mode Enable
pub const TEST_TESTK: u32 = 0x00000004;  // Test_K Mode Enable
pub const TEST_TESTJ: u32 = 0x00000002;  // Test_J Mode Enable
pub const TEST_TESTSE0NAK: u32 = 0x00000001;  // Test_SE0_NAK Test Mode Enable

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_FIFO0 register.
//
//*****************************************************************************
pub const FIFO0_EPDATA_M: u32 = 0xFFFFFFFF;  // Endpoint Data
pub const FIFO0_EPDATA_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_FIFO1 register.
//
//*****************************************************************************
pub const FIFO1_EPDATA_M: u32 = 0xFFFFFFFF;  // Endpoint Data
pub const FIFO1_EPDATA_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_FIFO2 register.
//
//*****************************************************************************
pub const FIFO2_EPDATA_M: u32 = 0xFFFFFFFF;  // Endpoint Data
pub const FIFO2_EPDATA_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_FIFO3 register.
//
//*****************************************************************************
pub const FIFO3_EPDATA_M: u32 = 0xFFFFFFFF;  // Endpoint Data
pub const FIFO3_EPDATA_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_FIFO4 register.
//
//*****************************************************************************
pub const FIFO4_EPDATA_M: u32 = 0xFFFFFFFF;  // Endpoint Data
pub const FIFO4_EPDATA_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_FIFO5 register.
//
//*****************************************************************************
pub const FIFO5_EPDATA_M: u32 = 0xFFFFFFFF;  // Endpoint Data
pub const FIFO5_EPDATA_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_FIFO6 register.
//
//*****************************************************************************
pub const FIFO6_EPDATA_M: u32 = 0xFFFFFFFF;  // Endpoint Data
pub const FIFO6_EPDATA_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_FIFO7 register.
//
//*****************************************************************************
pub const FIFO7_EPDATA_M: u32 = 0xFFFFFFFF;  // Endpoint Data
pub const FIFO7_EPDATA_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DEVCTL register.
//
//*****************************************************************************
pub const DEVCTL_DEV: u32 = 0x00000080;  // Device Mode (OTG only)
pub const DEVCTL_FSDEV: u32 = 0x00000040;  // Full-Speed Device Detected
pub const DEVCTL_LSDEV: u32 = 0x00000020;  // Low-Speed Device Detected
pub const DEVCTL_VBUS_M: u32 = 0x00000018;  // VBUS Level (OTG only)
pub const DEVCTL_VBUS_NONE: u32 = 0x00000000;  // Below SessionEnd
pub const DEVCTL_VBUS_SEND: u32 = 0x00000008;  // Above SessionEnd, below AValid
pub const DEVCTL_VBUS_AVALID: u32 = 0x00000010;  // Above AValid, below VBUSValid
pub const DEVCTL_VBUS_VALID: u32 = 0x00000018;  // Above VBUSValid
pub const DEVCTL_HOST: u32 = 0x00000004;  // Host Mode
pub const DEVCTL_HOSTREQ: u32 = 0x00000002;  // Host Request (OTG only)
pub const DEVCTL_SESSION: u32 = 0x00000001;  // Session Start/End (OTG only)

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_CCONF register.
//
//*****************************************************************************
pub const CCONF_TXEDMA: u32 = 0x00000002;  // TX Early DMA Enable
pub const CCONF_RXEDMA: u32 = 0x00000001;  // TX Early DMA Enable

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXFIFOSZ register.
//
//*****************************************************************************
pub const TXFIFOSZ_DPB: u32 = 0x00000010;  // Double Packet Buffer Support
pub const TXFIFOSZ_SIZE_M: u32 = 0x0000000F;  // Max Packet Size
pub const TXFIFOSZ_SIZE_8: u32 = 0x00000000;  // 8
pub const TXFIFOSZ_SIZE_16: u32 = 0x00000001;  // 16
pub const TXFIFOSZ_SIZE_32: u32 = 0x00000002;  // 32
pub const TXFIFOSZ_SIZE_64: u32 = 0x00000003;  // 64
pub const TXFIFOSZ_SIZE_128: u32 = 0x00000004;  // 128
pub const TXFIFOSZ_SIZE_256: u32 = 0x00000005;  // 256
pub const TXFIFOSZ_SIZE_512: u32 = 0x00000006;  // 512
pub const TXFIFOSZ_SIZE_1024: u32 = 0x00000007;  // 1024
pub const TXFIFOSZ_SIZE_2048: u32 = 0x00000008;  // 2048

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXFIFOSZ register.
//
//*****************************************************************************
pub const RXFIFOSZ_DPB: u32 = 0x00000010;  // Double Packet Buffer Support
pub const RXFIFOSZ_SIZE_M: u32 = 0x0000000F;  // Max Packet Size
pub const RXFIFOSZ_SIZE_8: u32 = 0x00000000;  // 8
pub const RXFIFOSZ_SIZE_16: u32 = 0x00000001;  // 16
pub const RXFIFOSZ_SIZE_32: u32 = 0x00000002;  // 32
pub const RXFIFOSZ_SIZE_64: u32 = 0x00000003;  // 64
pub const RXFIFOSZ_SIZE_128: u32 = 0x00000004;  // 128
pub const RXFIFOSZ_SIZE_256: u32 = 0x00000005;  // 256
pub const RXFIFOSZ_SIZE_512: u32 = 0x00000006;  // 512
pub const RXFIFOSZ_SIZE_1024: u32 = 0x00000007;  // 1024
pub const RXFIFOSZ_SIZE_2048: u32 = 0x00000008;  // 2048

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXFIFOADD
// register.
//
//*****************************************************************************
pub const TXFIFOADD_ADDR_M: u32 = 0x000001FF;  // Transmit/Receive Start Address
pub const TXFIFOADD_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXFIFOADD
// register.
//
//*****************************************************************************
pub const RXFIFOADD_ADDR_M: u32 = 0x000001FF;  // Transmit/Receive Start Address
pub const RXFIFOADD_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_ULPIVBUSCTL
// register.
//
//*****************************************************************************
pub const ULPIVBUSCTL_USEEXTVBUSIND: u32 = 0x00000002;  // Use External VBUS Indicator
pub const ULPIVBUSCTL_USEEXTVBUS: u32 = 0x00000001;  // Use External VBUS

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_ULPIREGDATA
// register.
//
//*****************************************************************************
pub const ULPIREGDATA_REGDATA_M: u32 = 0x000000FF;  // Register Data
pub const ULPIREGDATA_REGDATA_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_ULPIREGADDR
// register.
//
//*****************************************************************************
pub const ULPIREGADDR_ADDR_M: u32 = 0x000000FF;  // Register Address
pub const ULPIREGADDR_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_ULPIREGCTL
// register.
//
//*****************************************************************************
pub const ULPIREGCTL_RDWR: u32 = 0x00000004;  // Read/Write Control
pub const ULPIREGCTL_REGCMPLT: u32 = 0x00000002;  // Register Access Complete
pub const ULPIREGCTL_REGACC: u32 = 0x00000001;  // Initiate Register Access

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_EPINFO register.
//
//*****************************************************************************
pub const EPINFO_RXEP_M: u32 = 0x000000F0;  // RX Endpoints
pub const EPINFO_TXEP_M: u32 = 0x0000000F;  // TX Endpoints
pub const EPINFO_RXEP_S: uint = 4;
pub const EPINFO_TXEP_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RAMINFO register.
//
//*****************************************************************************
pub const RAMINFO_DMACHAN_M: u32 = 0x000000F0;  // DMA Channels
pub const RAMINFO_RAMBITS_M: u32 = 0x0000000F;  // RAM Address Bus Width
pub const RAMINFO_DMACHAN_S: uint = 4;
pub const RAMINFO_RAMBITS_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_CONTIM register.
//
//*****************************************************************************
pub const CONTIM_WTCON_M: u32 = 0x000000F0;  // Connect Wait
pub const CONTIM_WTID_M: u32 = 0x0000000F;  // Wait ID
pub const CONTIM_WTCON_S: uint = 4;
pub const CONTIM_WTID_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_VPLEN register.
//
//*****************************************************************************
pub const VPLEN_VPLEN_M: u32 = 0x000000FF;  // VBUS Pulse Length
pub const VPLEN_VPLEN_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_HSEOF register.
//
//*****************************************************************************
pub const HSEOF_HSEOFG_M: u32 = 0x000000FF;  // HIgh-Speed End-of-Frame Gap
pub const HSEOF_HSEOFG_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_FSEOF register.
//
//*****************************************************************************
pub const FSEOF_FSEOFG_M: u32 = 0x000000FF;  // Full-Speed End-of-Frame Gap
pub const FSEOF_FSEOFG_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_LSEOF register.
//
//*****************************************************************************
pub const LSEOF_LSEOFG_M: u32 = 0x000000FF;  // Low-Speed End-of-Frame Gap
pub const LSEOF_LSEOFG_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXFUNCADDR0
// register.
//
//*****************************************************************************
pub const TXFUNCADDR0_ADDR_M: u32 = 0x0000007F;  // Device Address
pub const TXFUNCADDR0_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXHUBADDR0
// register.
//
//*****************************************************************************
pub const TXHUBADDR0_ADDR_M: u32 = 0x0000007F;  // Hub Address
pub const TXHUBADDR0_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXHUBPORT0
// register.
//
//*****************************************************************************
pub const TXHUBPORT0_PORT_M: u32 = 0x0000007F;  // Hub Port
pub const TXHUBPORT0_PORT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXFUNCADDR1
// register.
//
//*****************************************************************************
pub const TXFUNCADDR1_ADDR_M: u32 = 0x0000007F;  // Device Address
pub const TXFUNCADDR1_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXHUBADDR1
// register.
//
//*****************************************************************************
pub const TXHUBADDR1_ADDR_M: u32 = 0x0000007F;  // Hub Address
pub const TXHUBADDR1_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXHUBPORT1
// register.
//
//*****************************************************************************
pub const TXHUBPORT1_PORT_M: u32 = 0x0000007F;  // Hub Port
pub const TXHUBPORT1_PORT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXFUNCADDR1
// register.
//
//*****************************************************************************
pub const RXFUNCADDR1_ADDR_M: u32 = 0x0000007F;  // Device Address
pub const RXFUNCADDR1_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXHUBADDR1
// register.
//
//*****************************************************************************
pub const RXHUBADDR1_ADDR_M: u32 = 0x0000007F;  // Hub Address
pub const RXHUBADDR1_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXHUBPORT1
// register.
//
//*****************************************************************************
pub const RXHUBPORT1_PORT_M: u32 = 0x0000007F;  // Hub Port
pub const RXHUBPORT1_PORT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXFUNCADDR2
// register.
//
//*****************************************************************************
pub const TXFUNCADDR2_ADDR_M: u32 = 0x0000007F;  // Device Address
pub const TXFUNCADDR2_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXHUBADDR2
// register.
//
//*****************************************************************************
pub const TXHUBADDR2_ADDR_M: u32 = 0x0000007F;  // Hub Address
pub const TXHUBADDR2_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXHUBPORT2
// register.
//
//*****************************************************************************
pub const TXHUBPORT2_PORT_M: u32 = 0x0000007F;  // Hub Port
pub const TXHUBPORT2_PORT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXFUNCADDR2
// register.
//
//*****************************************************************************
pub const RXFUNCADDR2_ADDR_M: u32 = 0x0000007F;  // Device Address
pub const RXFUNCADDR2_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXHUBADDR2
// register.
//
//*****************************************************************************
pub const RXHUBADDR2_ADDR_M: u32 = 0x0000007F;  // Hub Address
pub const RXHUBADDR2_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXHUBPORT2
// register.
//
//*****************************************************************************
pub const RXHUBPORT2_PORT_M: u32 = 0x0000007F;  // Hub Port
pub const RXHUBPORT2_PORT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXFUNCADDR3
// register.
//
//*****************************************************************************
pub const TXFUNCADDR3_ADDR_M: u32 = 0x0000007F;  // Device Address
pub const TXFUNCADDR3_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXHUBADDR3
// register.
//
//*****************************************************************************
pub const TXHUBADDR3_ADDR_M: u32 = 0x0000007F;  // Hub Address
pub const TXHUBADDR3_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXHUBPORT3
// register.
//
//*****************************************************************************
pub const TXHUBPORT3_PORT_M: u32 = 0x0000007F;  // Hub Port
pub const TXHUBPORT3_PORT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXFUNCADDR3
// register.
//
//*****************************************************************************
pub const RXFUNCADDR3_ADDR_M: u32 = 0x0000007F;  // Device Address
pub const RXFUNCADDR3_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXHUBADDR3
// register.
//
//*****************************************************************************
pub const RXHUBADDR3_ADDR_M: u32 = 0x0000007F;  // Hub Address
pub const RXHUBADDR3_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXHUBPORT3
// register.
//
//*****************************************************************************
pub const RXHUBPORT3_PORT_M: u32 = 0x0000007F;  // Hub Port
pub const RXHUBPORT3_PORT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXFUNCADDR4
// register.
//
//*****************************************************************************
pub const TXFUNCADDR4_ADDR_M: u32 = 0x0000007F;  // Device Address
pub const TXFUNCADDR4_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXHUBADDR4
// register.
//
//*****************************************************************************
pub const TXHUBADDR4_ADDR_M: u32 = 0x0000007F;  // Hub Address
pub const TXHUBADDR4_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXHUBPORT4
// register.
//
//*****************************************************************************
pub const TXHUBPORT4_PORT_M: u32 = 0x0000007F;  // Hub Port
pub const TXHUBPORT4_PORT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXFUNCADDR4
// register.
//
//*****************************************************************************
pub const RXFUNCADDR4_ADDR_M: u32 = 0x0000007F;  // Device Address
pub const RXFUNCADDR4_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXHUBADDR4
// register.
//
//*****************************************************************************
pub const RXHUBADDR4_ADDR_M: u32 = 0x0000007F;  // Hub Address
pub const RXHUBADDR4_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXHUBPORT4
// register.
//
//*****************************************************************************
pub const RXHUBPORT4_PORT_M: u32 = 0x0000007F;  // Hub Port
pub const RXHUBPORT4_PORT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXFUNCADDR5
// register.
//
//*****************************************************************************
pub const TXFUNCADDR5_ADDR_M: u32 = 0x0000007F;  // Device Address
pub const TXFUNCADDR5_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXHUBADDR5
// register.
//
//*****************************************************************************
pub const TXHUBADDR5_ADDR_M: u32 = 0x0000007F;  // Hub Address
pub const TXHUBADDR5_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXHUBPORT5
// register.
//
//*****************************************************************************
pub const TXHUBPORT5_PORT_M: u32 = 0x0000007F;  // Hub Port
pub const TXHUBPORT5_PORT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXFUNCADDR5
// register.
//
//*****************************************************************************
pub const RXFUNCADDR5_ADDR_M: u32 = 0x0000007F;  // Device Address
pub const RXFUNCADDR5_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXHUBADDR5
// register.
//
//*****************************************************************************
pub const RXHUBADDR5_ADDR_M: u32 = 0x0000007F;  // Hub Address
pub const RXHUBADDR5_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXHUBPORT5
// register.
//
//*****************************************************************************
pub const RXHUBPORT5_PORT_M: u32 = 0x0000007F;  // Hub Port
pub const RXHUBPORT5_PORT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXFUNCADDR6
// register.
//
//*****************************************************************************
pub const TXFUNCADDR6_ADDR_M: u32 = 0x0000007F;  // Device Address
pub const TXFUNCADDR6_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXHUBADDR6
// register.
//
//*****************************************************************************
pub const TXHUBADDR6_ADDR_M: u32 = 0x0000007F;  // Hub Address
pub const TXHUBADDR6_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXHUBPORT6
// register.
//
//*****************************************************************************
pub const TXHUBPORT6_PORT_M: u32 = 0x0000007F;  // Hub Port
pub const TXHUBPORT6_PORT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXFUNCADDR6
// register.
//
//*****************************************************************************
pub const RXFUNCADDR6_ADDR_M: u32 = 0x0000007F;  // Device Address
pub const RXFUNCADDR6_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXHUBADDR6
// register.
//
//*****************************************************************************
pub const RXHUBADDR6_ADDR_M: u32 = 0x0000007F;  // Hub Address
pub const RXHUBADDR6_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXHUBPORT6
// register.
//
//*****************************************************************************
pub const RXHUBPORT6_PORT_M: u32 = 0x0000007F;  // Hub Port
pub const RXHUBPORT6_PORT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXFUNCADDR7
// register.
//
//*****************************************************************************
pub const TXFUNCADDR7_ADDR_M: u32 = 0x0000007F;  // Device Address
pub const TXFUNCADDR7_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXHUBADDR7
// register.
//
//*****************************************************************************
pub const TXHUBADDR7_ADDR_M: u32 = 0x0000007F;  // Hub Address
pub const TXHUBADDR7_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXHUBPORT7
// register.
//
//*****************************************************************************
pub const TXHUBPORT7_PORT_M: u32 = 0x0000007F;  // Hub Port
pub const TXHUBPORT7_PORT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXFUNCADDR7
// register.
//
//*****************************************************************************
pub const RXFUNCADDR7_ADDR_M: u32 = 0x0000007F;  // Device Address
pub const RXFUNCADDR7_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXHUBADDR7
// register.
//
//*****************************************************************************
pub const RXHUBADDR7_ADDR_M: u32 = 0x0000007F;  // Hub Address
pub const RXHUBADDR7_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXHUBPORT7
// register.
//
//*****************************************************************************
pub const RXHUBPORT7_PORT_M: u32 = 0x0000007F;  // Hub Port
pub const RXHUBPORT7_PORT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_CSRL0 register.
//
//*****************************************************************************
pub const CSRL0_NAKTO: u32 = 0x00000080;  // NAK Timeout
pub const CSRL0_SETENDC: u32 = 0x00000080;  // Setup End Clear
pub const CSRL0_STATUS: u32 = 0x00000040;  // STATUS Packet
pub const CSRL0_RXRDYC: u32 = 0x00000040;  // RXRDY Clear
pub const CSRL0_REQPKT: u32 = 0x00000020;  // Request Packet
pub const CSRL0_STALL: u32 = 0x00000020;  // Send Stall
pub const CSRL0_SETEND: u32 = 0x00000010;  // Setup End
pub const CSRL0_ERROR: u32 = 0x00000010;  // Error
pub const CSRL0_DATAEND: u32 = 0x00000008;  // Data End
pub const CSRL0_SETUP: u32 = 0x00000008;  // Setup Packet
pub const CSRL0_STALLED: u32 = 0x00000004;  // Endpoint Stalled
pub const CSRL0_TXRDY: u32 = 0x00000002;  // Transmit Packet Ready
pub const CSRL0_RXRDY: u32 = 0x00000001;  // Receive Packet Ready

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_CSRH0 register.
//
//*****************************************************************************
pub const CSRH0_DISPING: u32 = 0x00000008;  // PING Disable
pub const CSRH0_DTWE: u32 = 0x00000004;  // Data Toggle Write Enable
pub const CSRH0_DT: u32 = 0x00000002;  // Data Toggle
pub const CSRH0_FLUSH: u32 = 0x00000001;  // Flush FIFO

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_COUNT0 register.
//
//*****************************************************************************
pub const COUNT0_COUNT_M: u32 = 0x0000007F;  // FIFO Count
pub const COUNT0_COUNT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TYPE0 register.
//
//*****************************************************************************
pub const TYPE0_SPEED_M: u32 = 0x000000C0;  // Operating Speed
pub const TYPE0_SPEED_HIGH: u32 = 0x00000040;  // High
pub const TYPE0_SPEED_FULL: u32 = 0x00000080;  // Full
pub const TYPE0_SPEED_LOW: u32 = 0x000000C0;  // Low

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_NAKLMT register.
//
//*****************************************************************************
pub const NAKLMT_NAKLMT_M: u32 = 0x0000001F;  // EP0 NAK Limit
pub const NAKLMT_NAKLMT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXMAXP1 register.
//
//*****************************************************************************
pub const TXMAXP1_MAXLOAD_M: u32 = 0x000007FF;  // Maximum Payload
pub const TXMAXP1_MAXLOAD_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRL1 register.
//
//*****************************************************************************
pub const TXCSRL1_NAKTO: u32 = 0x00000080;  // NAK Timeout
pub const TXCSRL1_CLRDT: u32 = 0x00000040;  // Clear Data Toggle
pub const TXCSRL1_STALLED: u32 = 0x00000020;  // Endpoint Stalled
pub const TXCSRL1_STALL: u32 = 0x00000010;  // Send STALL
pub const TXCSRL1_SETUP: u32 = 0x00000010;  // Setup Packet
pub const TXCSRL1_FLUSH: u32 = 0x00000008;  // Flush FIFO
pub const TXCSRL1_ERROR: u32 = 0x00000004;  // Error
pub const TXCSRL1_UNDRN: u32 = 0x00000004;  // Underrun
pub const TXCSRL1_FIFONE: u32 = 0x00000002;  // FIFO Not Empty
pub const TXCSRL1_TXRDY: u32 = 0x00000001;  // Transmit Packet Ready

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRH1 register.
//
//*****************************************************************************
pub const TXCSRH1_AUTOSET: u32 = 0x00000080;  // Auto Set
pub const TXCSRH1_ISO: u32 = 0x00000040;  // Isochronous Transfers
pub const TXCSRH1_MODE: u32 = 0x00000020;  // Mode
pub const TXCSRH1_DMAEN: u32 = 0x00000010;  // DMA Request Enable
pub const TXCSRH1_FDT: u32 = 0x00000008;  // Force Data Toggle
pub const TXCSRH1_DMAMOD: u32 = 0x00000004;  // DMA Request Mode
pub const TXCSRH1_DTWE: u32 = 0x00000002;  // Data Toggle Write Enable
pub const TXCSRH1_DT: u32 = 0x00000001;  // Data Toggle

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXMAXP1 register.
//
//*****************************************************************************
pub const RXMAXP1_MAXLOAD_M: u32 = 0x000007FF;  // Maximum Payload
pub const RXMAXP1_MAXLOAD_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRL1 register.
//
//*****************************************************************************
pub const RXCSRL1_CLRDT: u32 = 0x00000080;  // Clear Data Toggle
pub const RXCSRL1_STALLED: u32 = 0x00000040;  // Endpoint Stalled
pub const RXCSRL1_STALL: u32 = 0x00000020;  // Send STALL
pub const RXCSRL1_REQPKT: u32 = 0x00000020;  // Request Packet
pub const RXCSRL1_FLUSH: u32 = 0x00000010;  // Flush FIFO
pub const RXCSRL1_DATAERR: u32 = 0x00000008;  // Data Error
pub const RXCSRL1_NAKTO: u32 = 0x00000008;  // NAK Timeout
pub const RXCSRL1_OVER: u32 = 0x00000004;  // Overrun
pub const RXCSRL1_ERROR: u32 = 0x00000004;  // Error
pub const RXCSRL1_FULL: u32 = 0x00000002;  // FIFO Full
pub const RXCSRL1_RXRDY: u32 = 0x00000001;  // Receive Packet Ready

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRH1 register.
//
//*****************************************************************************
pub const RXCSRH1_AUTOCL: u32 = 0x00000080;  // Auto Clear
pub const RXCSRH1_AUTORQ: u32 = 0x00000040;  // Auto Request
pub const RXCSRH1_ISO: u32 = 0x00000040;  // Isochronous Transfers
pub const RXCSRH1_DMAEN: u32 = 0x00000020;  // DMA Request Enable
pub const RXCSRH1_DISNYET: u32 = 0x00000010;  // Disable NYET
pub const RXCSRH1_PIDERR: u32 = 0x00000010;  // PID Error
pub const RXCSRH1_DMAMOD: u32 = 0x00000008;  // DMA Request Mode
pub const RXCSRH1_DTWE: u32 = 0x00000004;  // Data Toggle Write Enable
pub const RXCSRH1_DT: u32 = 0x00000002;  // Data Toggle
pub const RXCSRH1_INCOMPRX: u32 = 0x00000001;  // Incomplete RX Transmission
                                            // Status

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCOUNT1 register.
//
//*****************************************************************************
pub const RXCOUNT1_COUNT_M: u32 = 0x00001FFF;  // Receive Packet Count
pub const RXCOUNT1_COUNT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXTYPE1 register.
//
//*****************************************************************************
pub const TXTYPE1_SPEED_M: u32 = 0x000000C0;  // Operating Speed
pub const TXTYPE1_SPEED_DFLT: u32 = 0x00000000;  // Default
pub const TXTYPE1_SPEED_HIGH: u32 = 0x00000040;  // High
pub const TXTYPE1_SPEED_FULL: u32 = 0x00000080;  // Full
pub const TXTYPE1_SPEED_LOW: u32 = 0x000000C0;  // Low
pub const TXTYPE1_PROTO_M: u32 = 0x00000030;  // Protocol
pub const TXTYPE1_PROTO_CTRL: u32 = 0x00000000;  // Control
pub const TXTYPE1_PROTO_ISOC: u32 = 0x00000010;  // Isochronous
pub const TXTYPE1_PROTO_BULK: u32 = 0x00000020;  // Bulk
pub const TXTYPE1_PROTO_INT: u32 = 0x00000030;  // Interrupt
pub const TXTYPE1_TEP_M: u32 = 0x0000000F;  // Target Endpoint Number
pub const TXTYPE1_TEP_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXINTERVAL1
// register.
//
//*****************************************************************************
pub const TXINTERVAL1_NAKLMT_M: u32 = 0x000000FF;  // NAK Limit
pub const TXINTERVAL1_TXPOLL_M: u32 = 0x000000FF;  // TX Polling
pub const TXINTERVAL1_TXPOLL_S: uint = 0;
pub const TXINTERVAL1_NAKLMT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXTYPE1 register.
//
//*****************************************************************************
pub const RXTYPE1_SPEED_M: u32 = 0x000000C0;  // Operating Speed
pub const RXTYPE1_SPEED_DFLT: u32 = 0x00000000;  // Default
pub const RXTYPE1_SPEED_HIGH: u32 = 0x00000040;  // High
pub const RXTYPE1_SPEED_FULL: u32 = 0x00000080;  // Full
pub const RXTYPE1_SPEED_LOW: u32 = 0x000000C0;  // Low
pub const RXTYPE1_PROTO_M: u32 = 0x00000030;  // Protocol
pub const RXTYPE1_PROTO_CTRL: u32 = 0x00000000;  // Control
pub const RXTYPE1_PROTO_ISOC: u32 = 0x00000010;  // Isochronous
pub const RXTYPE1_PROTO_BULK: u32 = 0x00000020;  // Bulk
pub const RXTYPE1_PROTO_INT: u32 = 0x00000030;  // Interrupt
pub const RXTYPE1_TEP_M: u32 = 0x0000000F;  // Target Endpoint Number
pub const RXTYPE1_TEP_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXINTERVAL1
// register.
//
//*****************************************************************************
pub const RXINTERVAL1_TXPOLL_M: u32 = 0x000000FF;  // RX Polling
pub const RXINTERVAL1_NAKLMT_M: u32 = 0x000000FF;  // NAK Limit
pub const RXINTERVAL1_TXPOLL_S: uint = 0;
pub const RXINTERVAL1_NAKLMT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXMAXP2 register.
//
//*****************************************************************************
pub const TXMAXP2_MAXLOAD_M: u32 = 0x000007FF;  // Maximum Payload
pub const TXMAXP2_MAXLOAD_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRL2 register.
//
//*****************************************************************************
pub const TXCSRL2_NAKTO: u32 = 0x00000080;  // NAK Timeout
pub const TXCSRL2_CLRDT: u32 = 0x00000040;  // Clear Data Toggle
pub const TXCSRL2_STALLED: u32 = 0x00000020;  // Endpoint Stalled
pub const TXCSRL2_SETUP: u32 = 0x00000010;  // Setup Packet
pub const TXCSRL2_STALL: u32 = 0x00000010;  // Send STALL
pub const TXCSRL2_FLUSH: u32 = 0x00000008;  // Flush FIFO
pub const TXCSRL2_ERROR: u32 = 0x00000004;  // Error
pub const TXCSRL2_UNDRN: u32 = 0x00000004;  // Underrun
pub const TXCSRL2_FIFONE: u32 = 0x00000002;  // FIFO Not Empty
pub const TXCSRL2_TXRDY: u32 = 0x00000001;  // Transmit Packet Ready

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRH2 register.
//
//*****************************************************************************
pub const TXCSRH2_AUTOSET: u32 = 0x00000080;  // Auto Set
pub const TXCSRH2_ISO: u32 = 0x00000040;  // Isochronous Transfers
pub const TXCSRH2_MODE: u32 = 0x00000020;  // Mode
pub const TXCSRH2_DMAEN: u32 = 0x00000010;  // DMA Request Enable
pub const TXCSRH2_FDT: u32 = 0x00000008;  // Force Data Toggle
pub const TXCSRH2_DMAMOD: u32 = 0x00000004;  // DMA Request Mode
pub const TXCSRH2_DTWE: u32 = 0x00000002;  // Data Toggle Write Enable
pub const TXCSRH2_DT: u32 = 0x00000001;  // Data Toggle

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXMAXP2 register.
//
//*****************************************************************************
pub const RXMAXP2_MAXLOAD_M: u32 = 0x000007FF;  // Maximum Payload
pub const RXMAXP2_MAXLOAD_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRL2 register.
//
//*****************************************************************************
pub const RXCSRL2_CLRDT: u32 = 0x00000080;  // Clear Data Toggle
pub const RXCSRL2_STALLED: u32 = 0x00000040;  // Endpoint Stalled
pub const RXCSRL2_REQPKT: u32 = 0x00000020;  // Request Packet
pub const RXCSRL2_STALL: u32 = 0x00000020;  // Send STALL
pub const RXCSRL2_FLUSH: u32 = 0x00000010;  // Flush FIFO
pub const RXCSRL2_DATAERR: u32 = 0x00000008;  // Data Error
pub const RXCSRL2_NAKTO: u32 = 0x00000008;  // NAK Timeout
pub const RXCSRL2_ERROR: u32 = 0x00000004;  // Error
pub const RXCSRL2_OVER: u32 = 0x00000004;  // Overrun
pub const RXCSRL2_FULL: u32 = 0x00000002;  // FIFO Full
pub const RXCSRL2_RXRDY: u32 = 0x00000001;  // Receive Packet Ready

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRH2 register.
//
//*****************************************************************************
pub const RXCSRH2_AUTOCL: u32 = 0x00000080;  // Auto Clear
pub const RXCSRH2_AUTORQ: u32 = 0x00000040;  // Auto Request
pub const RXCSRH2_ISO: u32 = 0x00000040;  // Isochronous Transfers
pub const RXCSRH2_DMAEN: u32 = 0x00000020;  // DMA Request Enable
pub const RXCSRH2_DISNYET: u32 = 0x00000010;  // Disable NYET
pub const RXCSRH2_PIDERR: u32 = 0x00000010;  // PID Error
pub const RXCSRH2_DMAMOD: u32 = 0x00000008;  // DMA Request Mode
pub const RXCSRH2_DTWE: u32 = 0x00000004;  // Data Toggle Write Enable
pub const RXCSRH2_DT: u32 = 0x00000002;  // Data Toggle
pub const RXCSRH2_INCOMPRX: u32 = 0x00000001;  // Incomplete RX Transmission
                                            // Status

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCOUNT2 register.
//
//*****************************************************************************
pub const RXCOUNT2_COUNT_M: u32 = 0x00001FFF;  // Receive Packet Count
pub const RXCOUNT2_COUNT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXTYPE2 register.
//
//*****************************************************************************
pub const TXTYPE2_SPEED_M: u32 = 0x000000C0;  // Operating Speed
pub const TXTYPE2_SPEED_DFLT: u32 = 0x00000000;  // Default
pub const TXTYPE2_SPEED_HIGH: u32 = 0x00000040;  // High
pub const TXTYPE2_SPEED_FULL: u32 = 0x00000080;  // Full
pub const TXTYPE2_SPEED_LOW: u32 = 0x000000C0;  // Low
pub const TXTYPE2_PROTO_M: u32 = 0x00000030;  // Protocol
pub const TXTYPE2_PROTO_CTRL: u32 = 0x00000000;  // Control
pub const TXTYPE2_PROTO_ISOC: u32 = 0x00000010;  // Isochronous
pub const TXTYPE2_PROTO_BULK: u32 = 0x00000020;  // Bulk
pub const TXTYPE2_PROTO_INT: u32 = 0x00000030;  // Interrupt
pub const TXTYPE2_TEP_M: u32 = 0x0000000F;  // Target Endpoint Number
pub const TXTYPE2_TEP_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXINTERVAL2
// register.
//
//*****************************************************************************
pub const TXINTERVAL2_TXPOLL_M: u32 = 0x000000FF;  // TX Polling
pub const TXINTERVAL2_NAKLMT_M: u32 = 0x000000FF;  // NAK Limit
pub const TXINTERVAL2_NAKLMT_S: uint = 0;
pub const TXINTERVAL2_TXPOLL_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXTYPE2 register.
//
//*****************************************************************************
pub const RXTYPE2_SPEED_M: u32 = 0x000000C0;  // Operating Speed
pub const RXTYPE2_SPEED_DFLT: u32 = 0x00000000;  // Default
pub const RXTYPE2_SPEED_HIGH: u32 = 0x00000040;  // High
pub const RXTYPE2_SPEED_FULL: u32 = 0x00000080;  // Full
pub const RXTYPE2_SPEED_LOW: u32 = 0x000000C0;  // Low
pub const RXTYPE2_PROTO_M: u32 = 0x00000030;  // Protocol
pub const RXTYPE2_PROTO_CTRL: u32 = 0x00000000;  // Control
pub const RXTYPE2_PROTO_ISOC: u32 = 0x00000010;  // Isochronous
pub const RXTYPE2_PROTO_BULK: u32 = 0x00000020;  // Bulk
pub const RXTYPE2_PROTO_INT: u32 = 0x00000030;  // Interrupt
pub const RXTYPE2_TEP_M: u32 = 0x0000000F;  // Target Endpoint Number
pub const RXTYPE2_TEP_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXINTERVAL2
// register.
//
//*****************************************************************************
pub const RXINTERVAL2_TXPOLL_M: u32 = 0x000000FF;  // RX Polling
pub const RXINTERVAL2_NAKLMT_M: u32 = 0x000000FF;  // NAK Limit
pub const RXINTERVAL2_TXPOLL_S: uint = 0;
pub const RXINTERVAL2_NAKLMT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXMAXP3 register.
//
//*****************************************************************************
pub const TXMAXP3_MAXLOAD_M: u32 = 0x000007FF;  // Maximum Payload
pub const TXMAXP3_MAXLOAD_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRL3 register.
//
//*****************************************************************************
pub const TXCSRL3_NAKTO: u32 = 0x00000080;  // NAK Timeout
pub const TXCSRL3_CLRDT: u32 = 0x00000040;  // Clear Data Toggle
pub const TXCSRL3_STALLED: u32 = 0x00000020;  // Endpoint Stalled
pub const TXCSRL3_SETUP: u32 = 0x00000010;  // Setup Packet
pub const TXCSRL3_STALL: u32 = 0x00000010;  // Send STALL
pub const TXCSRL3_FLUSH: u32 = 0x00000008;  // Flush FIFO
pub const TXCSRL3_ERROR: u32 = 0x00000004;  // Error
pub const TXCSRL3_UNDRN: u32 = 0x00000004;  // Underrun
pub const TXCSRL3_FIFONE: u32 = 0x00000002;  // FIFO Not Empty
pub const TXCSRL3_TXRDY: u32 = 0x00000001;  // Transmit Packet Ready

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRH3 register.
//
//*****************************************************************************
pub const TXCSRH3_AUTOSET: u32 = 0x00000080;  // Auto Set
pub const TXCSRH3_ISO: u32 = 0x00000040;  // Isochronous Transfers
pub const TXCSRH3_MODE: u32 = 0x00000020;  // Mode
pub const TXCSRH3_DMAEN: u32 = 0x00000010;  // DMA Request Enable
pub const TXCSRH3_FDT: u32 = 0x00000008;  // Force Data Toggle
pub const TXCSRH3_DMAMOD: u32 = 0x00000004;  // DMA Request Mode
pub const TXCSRH3_DTWE: u32 = 0x00000002;  // Data Toggle Write Enable
pub const TXCSRH3_DT: u32 = 0x00000001;  // Data Toggle

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXMAXP3 register.
//
//*****************************************************************************
pub const RXMAXP3_MAXLOAD_M: u32 = 0x000007FF;  // Maximum Payload
pub const RXMAXP3_MAXLOAD_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRL3 register.
//
//*****************************************************************************
pub const RXCSRL3_CLRDT: u32 = 0x00000080;  // Clear Data Toggle
pub const RXCSRL3_STALLED: u32 = 0x00000040;  // Endpoint Stalled
pub const RXCSRL3_STALL: u32 = 0x00000020;  // Send STALL
pub const RXCSRL3_REQPKT: u32 = 0x00000020;  // Request Packet
pub const RXCSRL3_FLUSH: u32 = 0x00000010;  // Flush FIFO
pub const RXCSRL3_DATAERR: u32 = 0x00000008;  // Data Error
pub const RXCSRL3_NAKTO: u32 = 0x00000008;  // NAK Timeout
pub const RXCSRL3_ERROR: u32 = 0x00000004;  // Error
pub const RXCSRL3_OVER: u32 = 0x00000004;  // Overrun
pub const RXCSRL3_FULL: u32 = 0x00000002;  // FIFO Full
pub const RXCSRL3_RXRDY: u32 = 0x00000001;  // Receive Packet Ready

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRH3 register.
//
//*****************************************************************************
pub const RXCSRH3_AUTOCL: u32 = 0x00000080;  // Auto Clear
pub const RXCSRH3_AUTORQ: u32 = 0x00000040;  // Auto Request
pub const RXCSRH3_ISO: u32 = 0x00000040;  // Isochronous Transfers
pub const RXCSRH3_DMAEN: u32 = 0x00000020;  // DMA Request Enable
pub const RXCSRH3_DISNYET: u32 = 0x00000010;  // Disable NYET
pub const RXCSRH3_PIDERR: u32 = 0x00000010;  // PID Error
pub const RXCSRH3_DMAMOD: u32 = 0x00000008;  // DMA Request Mode
pub const RXCSRH3_DTWE: u32 = 0x00000004;  // Data Toggle Write Enable
pub const RXCSRH3_DT: u32 = 0x00000002;  // Data Toggle
pub const RXCSRH3_INCOMPRX: u32 = 0x00000001;  // Incomplete RX Transmission
                                            // Status

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCOUNT3 register.
//
//*****************************************************************************
pub const RXCOUNT3_COUNT_M: u32 = 0x00001FFF;  // Receive Packet Count
pub const RXCOUNT3_COUNT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXTYPE3 register.
//
//*****************************************************************************
pub const TXTYPE3_SPEED_M: u32 = 0x000000C0;  // Operating Speed
pub const TXTYPE3_SPEED_DFLT: u32 = 0x00000000;  // Default
pub const TXTYPE3_SPEED_HIGH: u32 = 0x00000040;  // High
pub const TXTYPE3_SPEED_FULL: u32 = 0x00000080;  // Full
pub const TXTYPE3_SPEED_LOW: u32 = 0x000000C0;  // Low
pub const TXTYPE3_PROTO_M: u32 = 0x00000030;  // Protocol
pub const TXTYPE3_PROTO_CTRL: u32 = 0x00000000;  // Control
pub const TXTYPE3_PROTO_ISOC: u32 = 0x00000010;  // Isochronous
pub const TXTYPE3_PROTO_BULK: u32 = 0x00000020;  // Bulk
pub const TXTYPE3_PROTO_INT: u32 = 0x00000030;  // Interrupt
pub const TXTYPE3_TEP_M: u32 = 0x0000000F;  // Target Endpoint Number
pub const TXTYPE3_TEP_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXINTERVAL3
// register.
//
//*****************************************************************************
pub const TXINTERVAL3_TXPOLL_M: u32 = 0x000000FF;  // TX Polling
pub const TXINTERVAL3_NAKLMT_M: u32 = 0x000000FF;  // NAK Limit
pub const TXINTERVAL3_TXPOLL_S: uint = 0;
pub const TXINTERVAL3_NAKLMT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXTYPE3 register.
//
//*****************************************************************************
pub const RXTYPE3_SPEED_M: u32 = 0x000000C0;  // Operating Speed
pub const RXTYPE3_SPEED_DFLT: u32 = 0x00000000;  // Default
pub const RXTYPE3_SPEED_HIGH: u32 = 0x00000040;  // High
pub const RXTYPE3_SPEED_FULL: u32 = 0x00000080;  // Full
pub const RXTYPE3_SPEED_LOW: u32 = 0x000000C0;  // Low
pub const RXTYPE3_PROTO_M: u32 = 0x00000030;  // Protocol
pub const RXTYPE3_PROTO_CTRL: u32 = 0x00000000;  // Control
pub const RXTYPE3_PROTO_ISOC: u32 = 0x00000010;  // Isochronous
pub const RXTYPE3_PROTO_BULK: u32 = 0x00000020;  // Bulk
pub const RXTYPE3_PROTO_INT: u32 = 0x00000030;  // Interrupt
pub const RXTYPE3_TEP_M: u32 = 0x0000000F;  // Target Endpoint Number
pub const RXTYPE3_TEP_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXINTERVAL3
// register.
//
//*****************************************************************************
pub const RXINTERVAL3_TXPOLL_M: u32 = 0x000000FF;  // RX Polling
pub const RXINTERVAL3_NAKLMT_M: u32 = 0x000000FF;  // NAK Limit
pub const RXINTERVAL3_TXPOLL_S: uint = 0;
pub const RXINTERVAL3_NAKLMT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXMAXP4 register.
//
//*****************************************************************************
pub const TXMAXP4_MAXLOAD_M: u32 = 0x000007FF;  // Maximum Payload
pub const TXMAXP4_MAXLOAD_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRL4 register.
//
//*****************************************************************************
pub const TXCSRL4_NAKTO: u32 = 0x00000080;  // NAK Timeout
pub const TXCSRL4_CLRDT: u32 = 0x00000040;  // Clear Data Toggle
pub const TXCSRL4_STALLED: u32 = 0x00000020;  // Endpoint Stalled
pub const TXCSRL4_SETUP: u32 = 0x00000010;  // Setup Packet
pub const TXCSRL4_STALL: u32 = 0x00000010;  // Send STALL
pub const TXCSRL4_FLUSH: u32 = 0x00000008;  // Flush FIFO
pub const TXCSRL4_ERROR: u32 = 0x00000004;  // Error
pub const TXCSRL4_UNDRN: u32 = 0x00000004;  // Underrun
pub const TXCSRL4_FIFONE: u32 = 0x00000002;  // FIFO Not Empty
pub const TXCSRL4_TXRDY: u32 = 0x00000001;  // Transmit Packet Ready

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRH4 register.
//
//*****************************************************************************
pub const TXCSRH4_AUTOSET: u32 = 0x00000080;  // Auto Set
pub const TXCSRH4_ISO: u32 = 0x00000040;  // Isochronous Transfers
pub const TXCSRH4_MODE: u32 = 0x00000020;  // Mode
pub const TXCSRH4_DMAEN: u32 = 0x00000010;  // DMA Request Enable
pub const TXCSRH4_FDT: u32 = 0x00000008;  // Force Data Toggle
pub const TXCSRH4_DMAMOD: u32 = 0x00000004;  // DMA Request Mode
pub const TXCSRH4_DTWE: u32 = 0x00000002;  // Data Toggle Write Enable
pub const TXCSRH4_DT: u32 = 0x00000001;  // Data Toggle

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXMAXP4 register.
//
//*****************************************************************************
pub const RXMAXP4_MAXLOAD_M: u32 = 0x000007FF;  // Maximum Payload
pub const RXMAXP4_MAXLOAD_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRL4 register.
//
//*****************************************************************************
pub const RXCSRL4_CLRDT: u32 = 0x00000080;  // Clear Data Toggle
pub const RXCSRL4_STALLED: u32 = 0x00000040;  // Endpoint Stalled
pub const RXCSRL4_STALL: u32 = 0x00000020;  // Send STALL
pub const RXCSRL4_REQPKT: u32 = 0x00000020;  // Request Packet
pub const RXCSRL4_FLUSH: u32 = 0x00000010;  // Flush FIFO
pub const RXCSRL4_NAKTO: u32 = 0x00000008;  // NAK Timeout
pub const RXCSRL4_DATAERR: u32 = 0x00000008;  // Data Error
pub const RXCSRL4_OVER: u32 = 0x00000004;  // Overrun
pub const RXCSRL4_ERROR: u32 = 0x00000004;  // Error
pub const RXCSRL4_FULL: u32 = 0x00000002;  // FIFO Full
pub const RXCSRL4_RXRDY: u32 = 0x00000001;  // Receive Packet Ready

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRH4 register.
//
//*****************************************************************************
pub const RXCSRH4_AUTOCL: u32 = 0x00000080;  // Auto Clear
pub const RXCSRH4_AUTORQ: u32 = 0x00000040;  // Auto Request
pub const RXCSRH4_ISO: u32 = 0x00000040;  // Isochronous Transfers
pub const RXCSRH4_DMAEN: u32 = 0x00000020;  // DMA Request Enable
pub const RXCSRH4_DISNYET: u32 = 0x00000010;  // Disable NYET
pub const RXCSRH4_PIDERR: u32 = 0x00000010;  // PID Error
pub const RXCSRH4_DMAMOD: u32 = 0x00000008;  // DMA Request Mode
pub const RXCSRH4_DTWE: u32 = 0x00000004;  // Data Toggle Write Enable
pub const RXCSRH4_DT: u32 = 0x00000002;  // Data Toggle
pub const RXCSRH4_INCOMPRX: u32 = 0x00000001;  // Incomplete RX Transmission
                                            // Status

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCOUNT4 register.
//
//*****************************************************************************
pub const RXCOUNT4_COUNT_M: u32 = 0x00001FFF;  // Receive Packet Count
pub const RXCOUNT4_COUNT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXTYPE4 register.
//
//*****************************************************************************
pub const TXTYPE4_SPEED_M: u32 = 0x000000C0;  // Operating Speed
pub const TXTYPE4_SPEED_DFLT: u32 = 0x00000000;  // Default
pub const TXTYPE4_SPEED_HIGH: u32 = 0x00000040;  // High
pub const TXTYPE4_SPEED_FULL: u32 = 0x00000080;  // Full
pub const TXTYPE4_SPEED_LOW: u32 = 0x000000C0;  // Low
pub const TXTYPE4_PROTO_M: u32 = 0x00000030;  // Protocol
pub const TXTYPE4_PROTO_CTRL: u32 = 0x00000000;  // Control
pub const TXTYPE4_PROTO_ISOC: u32 = 0x00000010;  // Isochronous
pub const TXTYPE4_PROTO_BULK: u32 = 0x00000020;  // Bulk
pub const TXTYPE4_PROTO_INT: u32 = 0x00000030;  // Interrupt
pub const TXTYPE4_TEP_M: u32 = 0x0000000F;  // Target Endpoint Number
pub const TXTYPE4_TEP_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXINTERVAL4
// register.
//
//*****************************************************************************
pub const TXINTERVAL4_TXPOLL_M: u32 = 0x000000FF;  // TX Polling
pub const TXINTERVAL4_NAKLMT_M: u32 = 0x000000FF;  // NAK Limit
pub const TXINTERVAL4_NAKLMT_S: uint = 0;
pub const TXINTERVAL4_TXPOLL_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXTYPE4 register.
//
//*****************************************************************************
pub const RXTYPE4_SPEED_M: u32 = 0x000000C0;  // Operating Speed
pub const RXTYPE4_SPEED_DFLT: u32 = 0x00000000;  // Default
pub const RXTYPE4_SPEED_HIGH: u32 = 0x00000040;  // High
pub const RXTYPE4_SPEED_FULL: u32 = 0x00000080;  // Full
pub const RXTYPE4_SPEED_LOW: u32 = 0x000000C0;  // Low
pub const RXTYPE4_PROTO_M: u32 = 0x00000030;  // Protocol
pub const RXTYPE4_PROTO_CTRL: u32 = 0x00000000;  // Control
pub const RXTYPE4_PROTO_ISOC: u32 = 0x00000010;  // Isochronous
pub const RXTYPE4_PROTO_BULK: u32 = 0x00000020;  // Bulk
pub const RXTYPE4_PROTO_INT: u32 = 0x00000030;  // Interrupt
pub const RXTYPE4_TEP_M: u32 = 0x0000000F;  // Target Endpoint Number
pub const RXTYPE4_TEP_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXINTERVAL4
// register.
//
//*****************************************************************************
pub const RXINTERVAL4_TXPOLL_M: u32 = 0x000000FF;  // RX Polling
pub const RXINTERVAL4_NAKLMT_M: u32 = 0x000000FF;  // NAK Limit
pub const RXINTERVAL4_NAKLMT_S: uint = 0;
pub const RXINTERVAL4_TXPOLL_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXMAXP5 register.
//
//*****************************************************************************
pub const TXMAXP5_MAXLOAD_M: u32 = 0x000007FF;  // Maximum Payload
pub const TXMAXP5_MAXLOAD_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRL5 register.
//
//*****************************************************************************
pub const TXCSRL5_NAKTO: u32 = 0x00000080;  // NAK Timeout
pub const TXCSRL5_CLRDT: u32 = 0x00000040;  // Clear Data Toggle
pub const TXCSRL5_STALLED: u32 = 0x00000020;  // Endpoint Stalled
pub const TXCSRL5_SETUP: u32 = 0x00000010;  // Setup Packet
pub const TXCSRL5_STALL: u32 = 0x00000010;  // Send STALL
pub const TXCSRL5_FLUSH: u32 = 0x00000008;  // Flush FIFO
pub const TXCSRL5_ERROR: u32 = 0x00000004;  // Error
pub const TXCSRL5_UNDRN: u32 = 0x00000004;  // Underrun
pub const TXCSRL5_FIFONE: u32 = 0x00000002;  // FIFO Not Empty
pub const TXCSRL5_TXRDY: u32 = 0x00000001;  // Transmit Packet Ready

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRH5 register.
//
//*****************************************************************************
pub const TXCSRH5_AUTOSET: u32 = 0x00000080;  // Auto Set
pub const TXCSRH5_ISO: u32 = 0x00000040;  // Isochronous Transfers
pub const TXCSRH5_MODE: u32 = 0x00000020;  // Mode
pub const TXCSRH5_DMAEN: u32 = 0x00000010;  // DMA Request Enable
pub const TXCSRH5_FDT: u32 = 0x00000008;  // Force Data Toggle
pub const TXCSRH5_DMAMOD: u32 = 0x00000004;  // DMA Request Mode
pub const TXCSRH5_DTWE: u32 = 0x00000002;  // Data Toggle Write Enable
pub const TXCSRH5_DT: u32 = 0x00000001;  // Data Toggle

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXMAXP5 register.
//
//*****************************************************************************
pub const RXMAXP5_MAXLOAD_M: u32 = 0x000007FF;  // Maximum Payload
pub const RXMAXP5_MAXLOAD_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRL5 register.
//
//*****************************************************************************
pub const RXCSRL5_CLRDT: u32 = 0x00000080;  // Clear Data Toggle
pub const RXCSRL5_STALLED: u32 = 0x00000040;  // Endpoint Stalled
pub const RXCSRL5_STALL: u32 = 0x00000020;  // Send STALL
pub const RXCSRL5_REQPKT: u32 = 0x00000020;  // Request Packet
pub const RXCSRL5_FLUSH: u32 = 0x00000010;  // Flush FIFO
pub const RXCSRL5_NAKTO: u32 = 0x00000008;  // NAK Timeout
pub const RXCSRL5_DATAERR: u32 = 0x00000008;  // Data Error
pub const RXCSRL5_ERROR: u32 = 0x00000004;  // Error
pub const RXCSRL5_OVER: u32 = 0x00000004;  // Overrun
pub const RXCSRL5_FULL: u32 = 0x00000002;  // FIFO Full
pub const RXCSRL5_RXRDY: u32 = 0x00000001;  // Receive Packet Ready

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRH5 register.
//
//*****************************************************************************
pub const RXCSRH5_AUTOCL: u32 = 0x00000080;  // Auto Clear
pub const RXCSRH5_AUTORQ: u32 = 0x00000040;  // Auto Request
pub const RXCSRH5_ISO: u32 = 0x00000040;  // Isochronous Transfers
pub const RXCSRH5_DMAEN: u32 = 0x00000020;  // DMA Request Enable
pub const RXCSRH5_DISNYET: u32 = 0x00000010;  // Disable NYET
pub const RXCSRH5_PIDERR: u32 = 0x00000010;  // PID Error
pub const RXCSRH5_DMAMOD: u32 = 0x00000008;  // DMA Request Mode
pub const RXCSRH5_DTWE: u32 = 0x00000004;  // Data Toggle Write Enable
pub const RXCSRH5_DT: u32 = 0x00000002;  // Data Toggle
pub const RXCSRH5_INCOMPRX: u32 = 0x00000001;  // Incomplete RX Transmission
                                            // Status

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCOUNT5 register.
//
//*****************************************************************************
pub const RXCOUNT5_COUNT_M: u32 = 0x00001FFF;  // Receive Packet Count
pub const RXCOUNT5_COUNT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXTYPE5 register.
//
//*****************************************************************************
pub const TXTYPE5_SPEED_M: u32 = 0x000000C0;  // Operating Speed
pub const TXTYPE5_SPEED_DFLT: u32 = 0x00000000;  // Default
pub const TXTYPE5_SPEED_HIGH: u32 = 0x00000040;  // High
pub const TXTYPE5_SPEED_FULL: u32 = 0x00000080;  // Full
pub const TXTYPE5_SPEED_LOW: u32 = 0x000000C0;  // Low
pub const TXTYPE5_PROTO_M: u32 = 0x00000030;  // Protocol
pub const TXTYPE5_PROTO_CTRL: u32 = 0x00000000;  // Control
pub const TXTYPE5_PROTO_ISOC: u32 = 0x00000010;  // Isochronous
pub const TXTYPE5_PROTO_BULK: u32 = 0x00000020;  // Bulk
pub const TXTYPE5_PROTO_INT: u32 = 0x00000030;  // Interrupt
pub const TXTYPE5_TEP_M: u32 = 0x0000000F;  // Target Endpoint Number
pub const TXTYPE5_TEP_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXINTERVAL5
// register.
//
//*****************************************************************************
pub const TXINTERVAL5_TXPOLL_M: u32 = 0x000000FF;  // TX Polling
pub const TXINTERVAL5_NAKLMT_M: u32 = 0x000000FF;  // NAK Limit
pub const TXINTERVAL5_NAKLMT_S: uint = 0;
pub const TXINTERVAL5_TXPOLL_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXTYPE5 register.
//
//*****************************************************************************
pub const RXTYPE5_SPEED_M: u32 = 0x000000C0;  // Operating Speed
pub const RXTYPE5_SPEED_DFLT: u32 = 0x00000000;  // Default
pub const RXTYPE5_SPEED_HIGH: u32 = 0x00000040;  // High
pub const RXTYPE5_SPEED_FULL: u32 = 0x00000080;  // Full
pub const RXTYPE5_SPEED_LOW: u32 = 0x000000C0;  // Low
pub const RXTYPE5_PROTO_M: u32 = 0x00000030;  // Protocol
pub const RXTYPE5_PROTO_CTRL: u32 = 0x00000000;  // Control
pub const RXTYPE5_PROTO_ISOC: u32 = 0x00000010;  // Isochronous
pub const RXTYPE5_PROTO_BULK: u32 = 0x00000020;  // Bulk
pub const RXTYPE5_PROTO_INT: u32 = 0x00000030;  // Interrupt
pub const RXTYPE5_TEP_M: u32 = 0x0000000F;  // Target Endpoint Number
pub const RXTYPE5_TEP_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXINTERVAL5
// register.
//
//*****************************************************************************
pub const RXINTERVAL5_TXPOLL_M: u32 = 0x000000FF;  // RX Polling
pub const RXINTERVAL5_NAKLMT_M: u32 = 0x000000FF;  // NAK Limit
pub const RXINTERVAL5_TXPOLL_S: uint = 0;
pub const RXINTERVAL5_NAKLMT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXMAXP6 register.
//
//*****************************************************************************
pub const TXMAXP6_MAXLOAD_M: u32 = 0x000007FF;  // Maximum Payload
pub const TXMAXP6_MAXLOAD_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRL6 register.
//
//*****************************************************************************
pub const TXCSRL6_NAKTO: u32 = 0x00000080;  // NAK Timeout
pub const TXCSRL6_CLRDT: u32 = 0x00000040;  // Clear Data Toggle
pub const TXCSRL6_STALLED: u32 = 0x00000020;  // Endpoint Stalled
pub const TXCSRL6_STALL: u32 = 0x00000010;  // Send STALL
pub const TXCSRL6_SETUP: u32 = 0x00000010;  // Setup Packet
pub const TXCSRL6_FLUSH: u32 = 0x00000008;  // Flush FIFO
pub const TXCSRL6_ERROR: u32 = 0x00000004;  // Error
pub const TXCSRL6_UNDRN: u32 = 0x00000004;  // Underrun
pub const TXCSRL6_FIFONE: u32 = 0x00000002;  // FIFO Not Empty
pub const TXCSRL6_TXRDY: u32 = 0x00000001;  // Transmit Packet Ready

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRH6 register.
//
//*****************************************************************************
pub const TXCSRH6_AUTOSET: u32 = 0x00000080;  // Auto Set
pub const TXCSRH6_ISO: u32 = 0x00000040;  // Isochronous Transfers
pub const TXCSRH6_MODE: u32 = 0x00000020;  // Mode
pub const TXCSRH6_DMAEN: u32 = 0x00000010;  // DMA Request Enable
pub const TXCSRH6_FDT: u32 = 0x00000008;  // Force Data Toggle
pub const TXCSRH6_DMAMOD: u32 = 0x00000004;  // DMA Request Mode
pub const TXCSRH6_DTWE: u32 = 0x00000002;  // Data Toggle Write Enable
pub const TXCSRH6_DT: u32 = 0x00000001;  // Data Toggle

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXMAXP6 register.
//
//*****************************************************************************
pub const RXMAXP6_MAXLOAD_M: u32 = 0x000007FF;  // Maximum Payload
pub const RXMAXP6_MAXLOAD_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRL6 register.
//
//*****************************************************************************
pub const RXCSRL6_CLRDT: u32 = 0x00000080;  // Clear Data Toggle
pub const RXCSRL6_STALLED: u32 = 0x00000040;  // Endpoint Stalled
pub const RXCSRL6_REQPKT: u32 = 0x00000020;  // Request Packet
pub const RXCSRL6_STALL: u32 = 0x00000020;  // Send STALL
pub const RXCSRL6_FLUSH: u32 = 0x00000010;  // Flush FIFO
pub const RXCSRL6_NAKTO: u32 = 0x00000008;  // NAK Timeout
pub const RXCSRL6_DATAERR: u32 = 0x00000008;  // Data Error
pub const RXCSRL6_ERROR: u32 = 0x00000004;  // Error
pub const RXCSRL6_OVER: u32 = 0x00000004;  // Overrun
pub const RXCSRL6_FULL: u32 = 0x00000002;  // FIFO Full
pub const RXCSRL6_RXRDY: u32 = 0x00000001;  // Receive Packet Ready

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRH6 register.
//
//*****************************************************************************
pub const RXCSRH6_AUTOCL: u32 = 0x00000080;  // Auto Clear
pub const RXCSRH6_AUTORQ: u32 = 0x00000040;  // Auto Request
pub const RXCSRH6_ISO: u32 = 0x00000040;  // Isochronous Transfers
pub const RXCSRH6_DMAEN: u32 = 0x00000020;  // DMA Request Enable
pub const RXCSRH6_DISNYET: u32 = 0x00000010;  // Disable NYET
pub const RXCSRH6_PIDERR: u32 = 0x00000010;  // PID Error
pub const RXCSRH6_DMAMOD: u32 = 0x00000008;  // DMA Request Mode
pub const RXCSRH6_DTWE: u32 = 0x00000004;  // Data Toggle Write Enable
pub const RXCSRH6_DT: u32 = 0x00000002;  // Data Toggle
pub const RXCSRH6_INCOMPRX: u32 = 0x00000001;  // Incomplete RX Transmission
                                            // Status

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCOUNT6 register.
//
//*****************************************************************************
pub const RXCOUNT6_COUNT_M: u32 = 0x00001FFF;  // Receive Packet Count
pub const RXCOUNT6_COUNT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXTYPE6 register.
//
//*****************************************************************************
pub const TXTYPE6_SPEED_M: u32 = 0x000000C0;  // Operating Speed
pub const TXTYPE6_SPEED_DFLT: u32 = 0x00000000;  // Default
pub const TXTYPE6_SPEED_HIGH: u32 = 0x00000040;  // High
pub const TXTYPE6_SPEED_FULL: u32 = 0x00000080;  // Full
pub const TXTYPE6_SPEED_LOW: u32 = 0x000000C0;  // Low
pub const TXTYPE6_PROTO_M: u32 = 0x00000030;  // Protocol
pub const TXTYPE6_PROTO_CTRL: u32 = 0x00000000;  // Control
pub const TXTYPE6_PROTO_ISOC: u32 = 0x00000010;  // Isochronous
pub const TXTYPE6_PROTO_BULK: u32 = 0x00000020;  // Bulk
pub const TXTYPE6_PROTO_INT: u32 = 0x00000030;  // Interrupt
pub const TXTYPE6_TEP_M: u32 = 0x0000000F;  // Target Endpoint Number
pub const TXTYPE6_TEP_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXINTERVAL6
// register.
//
//*****************************************************************************
pub const TXINTERVAL6_TXPOLL_M: u32 = 0x000000FF;  // TX Polling
pub const TXINTERVAL6_NAKLMT_M: u32 = 0x000000FF;  // NAK Limit
pub const TXINTERVAL6_TXPOLL_S: uint = 0;
pub const TXINTERVAL6_NAKLMT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXTYPE6 register.
//
//*****************************************************************************
pub const RXTYPE6_SPEED_M: u32 = 0x000000C0;  // Operating Speed
pub const RXTYPE6_SPEED_DFLT: u32 = 0x00000000;  // Default
pub const RXTYPE6_SPEED_HIGH: u32 = 0x00000040;  // High
pub const RXTYPE6_SPEED_FULL: u32 = 0x00000080;  // Full
pub const RXTYPE6_SPEED_LOW: u32 = 0x000000C0;  // Low
pub const RXTYPE6_PROTO_M: u32 = 0x00000030;  // Protocol
pub const RXTYPE6_PROTO_CTRL: u32 = 0x00000000;  // Control
pub const RXTYPE6_PROTO_ISOC: u32 = 0x00000010;  // Isochronous
pub const RXTYPE6_PROTO_BULK: u32 = 0x00000020;  // Bulk
pub const RXTYPE6_PROTO_INT: u32 = 0x00000030;  // Interrupt
pub const RXTYPE6_TEP_M: u32 = 0x0000000F;  // Target Endpoint Number
pub const RXTYPE6_TEP_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXINTERVAL6
// register.
//
//*****************************************************************************
pub const RXINTERVAL6_TXPOLL_M: u32 = 0x000000FF;  // RX Polling
pub const RXINTERVAL6_NAKLMT_M: u32 = 0x000000FF;  // NAK Limit
pub const RXINTERVAL6_NAKLMT_S: uint = 0;
pub const RXINTERVAL6_TXPOLL_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXMAXP7 register.
//
//*****************************************************************************
pub const TXMAXP7_MAXLOAD_M: u32 = 0x000007FF;  // Maximum Payload
pub const TXMAXP7_MAXLOAD_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRL7 register.
//
//*****************************************************************************
pub const TXCSRL7_NAKTO: u32 = 0x00000080;  // NAK Timeout
pub const TXCSRL7_CLRDT: u32 = 0x00000040;  // Clear Data Toggle
pub const TXCSRL7_STALLED: u32 = 0x00000020;  // Endpoint Stalled
pub const TXCSRL7_STALL: u32 = 0x00000010;  // Send STALL
pub const TXCSRL7_SETUP: u32 = 0x00000010;  // Setup Packet
pub const TXCSRL7_FLUSH: u32 = 0x00000008;  // Flush FIFO
pub const TXCSRL7_ERROR: u32 = 0x00000004;  // Error
pub const TXCSRL7_UNDRN: u32 = 0x00000004;  // Underrun
pub const TXCSRL7_FIFONE: u32 = 0x00000002;  // FIFO Not Empty
pub const TXCSRL7_TXRDY: u32 = 0x00000001;  // Transmit Packet Ready

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRH7 register.
//
//*****************************************************************************
pub const TXCSRH7_AUTOSET: u32 = 0x00000080;  // Auto Set
pub const TXCSRH7_ISO: u32 = 0x00000040;  // Isochronous Transfers
pub const TXCSRH7_MODE: u32 = 0x00000020;  // Mode
pub const TXCSRH7_DMAEN: u32 = 0x00000010;  // DMA Request Enable
pub const TXCSRH7_FDT: u32 = 0x00000008;  // Force Data Toggle
pub const TXCSRH7_DMAMOD: u32 = 0x00000004;  // DMA Request Mode
pub const TXCSRH7_DTWE: u32 = 0x00000002;  // Data Toggle Write Enable
pub const TXCSRH7_DT: u32 = 0x00000001;  // Data Toggle

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXMAXP7 register.
//
//*****************************************************************************
pub const RXMAXP7_MAXLOAD_M: u32 = 0x000007FF;  // Maximum Payload
pub const RXMAXP7_MAXLOAD_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRL7 register.
//
//*****************************************************************************
pub const RXCSRL7_CLRDT: u32 = 0x00000080;  // Clear Data Toggle
pub const RXCSRL7_STALLED: u32 = 0x00000040;  // Endpoint Stalled
pub const RXCSRL7_REQPKT: u32 = 0x00000020;  // Request Packet
pub const RXCSRL7_STALL: u32 = 0x00000020;  // Send STALL
pub const RXCSRL7_FLUSH: u32 = 0x00000010;  // Flush FIFO
pub const RXCSRL7_DATAERR: u32 = 0x00000008;  // Data Error
pub const RXCSRL7_NAKTO: u32 = 0x00000008;  // NAK Timeout
pub const RXCSRL7_ERROR: u32 = 0x00000004;  // Error
pub const RXCSRL7_OVER: u32 = 0x00000004;  // Overrun
pub const RXCSRL7_FULL: u32 = 0x00000002;  // FIFO Full
pub const RXCSRL7_RXRDY: u32 = 0x00000001;  // Receive Packet Ready

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRH7 register.
//
//*****************************************************************************
pub const RXCSRH7_AUTOCL: u32 = 0x00000080;  // Auto Clear
pub const RXCSRH7_ISO: u32 = 0x00000040;  // Isochronous Transfers
pub const RXCSRH7_AUTORQ: u32 = 0x00000040;  // Auto Request
pub const RXCSRH7_DMAEN: u32 = 0x00000020;  // DMA Request Enable
pub const RXCSRH7_PIDERR: u32 = 0x00000010;  // PID Error
pub const RXCSRH7_DISNYET: u32 = 0x00000010;  // Disable NYET
pub const RXCSRH7_DMAMOD: u32 = 0x00000008;  // DMA Request Mode
pub const RXCSRH7_DTWE: u32 = 0x00000004;  // Data Toggle Write Enable
pub const RXCSRH7_DT: u32 = 0x00000002;  // Data Toggle
pub const RXCSRH7_INCOMPRX: u32 = 0x00000001;  // Incomplete RX Transmission
                                            // Status

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCOUNT7 register.
//
//*****************************************************************************
pub const RXCOUNT7_COUNT_M: u32 = 0x00001FFF;  // Receive Packet Count
pub const RXCOUNT7_COUNT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXTYPE7 register.
//
//*****************************************************************************
pub const TXTYPE7_SPEED_M: u32 = 0x000000C0;  // Operating Speed
pub const TXTYPE7_SPEED_DFLT: u32 = 0x00000000;  // Default
pub const TXTYPE7_SPEED_HIGH: u32 = 0x00000040;  // High
pub const TXTYPE7_SPEED_FULL: u32 = 0x00000080;  // Full
pub const TXTYPE7_SPEED_LOW: u32 = 0x000000C0;  // Low
pub const TXTYPE7_PROTO_M: u32 = 0x00000030;  // Protocol
pub const TXTYPE7_PROTO_CTRL: u32 = 0x00000000;  // Control
pub const TXTYPE7_PROTO_ISOC: u32 = 0x00000010;  // Isochronous
pub const TXTYPE7_PROTO_BULK: u32 = 0x00000020;  // Bulk
pub const TXTYPE7_PROTO_INT: u32 = 0x00000030;  // Interrupt
pub const TXTYPE7_TEP_M: u32 = 0x0000000F;  // Target Endpoint Number
pub const TXTYPE7_TEP_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXINTERVAL7
// register.
//
//*****************************************************************************
pub const TXINTERVAL7_TXPOLL_M: u32 = 0x000000FF;  // TX Polling
pub const TXINTERVAL7_NAKLMT_M: u32 = 0x000000FF;  // NAK Limit
pub const TXINTERVAL7_NAKLMT_S: uint = 0;
pub const TXINTERVAL7_TXPOLL_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXTYPE7 register.
//
//*****************************************************************************
pub const RXTYPE7_SPEED_M: u32 = 0x000000C0;  // Operating Speed
pub const RXTYPE7_SPEED_DFLT: u32 = 0x00000000;  // Default
pub const RXTYPE7_SPEED_HIGH: u32 = 0x00000040;  // High
pub const RXTYPE7_SPEED_FULL: u32 = 0x00000080;  // Full
pub const RXTYPE7_SPEED_LOW: u32 = 0x000000C0;  // Low
pub const RXTYPE7_PROTO_M: u32 = 0x00000030;  // Protocol
pub const RXTYPE7_PROTO_CTRL: u32 = 0x00000000;  // Control
pub const RXTYPE7_PROTO_ISOC: u32 = 0x00000010;  // Isochronous
pub const RXTYPE7_PROTO_BULK: u32 = 0x00000020;  // Bulk
pub const RXTYPE7_PROTO_INT: u32 = 0x00000030;  // Interrupt
pub const RXTYPE7_TEP_M: u32 = 0x0000000F;  // Target Endpoint Number
pub const RXTYPE7_TEP_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXINTERVAL7
// register.
//
//*****************************************************************************
pub const RXINTERVAL7_TXPOLL_M: u32 = 0x000000FF;  // RX Polling
pub const RXINTERVAL7_NAKLMT_M: u32 = 0x000000FF;  // NAK Limit
pub const RXINTERVAL7_NAKLMT_S: uint = 0;
pub const RXINTERVAL7_TXPOLL_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMAINTR register.
//
//*****************************************************************************
pub const DMAINTR_CH7: u32 = 0x00000080;  // Channel 7 DMA Interrupt
pub const DMAINTR_CH6: u32 = 0x00000040;  // Channel 6 DMA Interrupt
pub const DMAINTR_CH5: u32 = 0x00000020;  // Channel 5 DMA Interrupt
pub const DMAINTR_CH4: u32 = 0x00000010;  // Channel 4 DMA Interrupt
pub const DMAINTR_CH3: u32 = 0x00000008;  // Channel 3 DMA Interrupt
pub const DMAINTR_CH2: u32 = 0x00000004;  // Channel 2 DMA Interrupt
pub const DMAINTR_CH1: u32 = 0x00000002;  // Channel 1 DMA Interrupt
pub const DMAINTR_CH0: u32 = 0x00000001;  // Channel 0 DMA Interrupt

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMACTL0 register.
//
//*****************************************************************************
pub const DMACTL0_BRSTM_M: u32 = 0x00000600;  // Burst Mode
pub const DMACTL0_BRSTM_ANY: u32 = 0x00000000;  // Bursts of unspecified length
pub const DMACTL0_BRSTM_INC4: u32 = 0x00000200;  // INCR4 or unspecified length
pub const DMACTL0_BRSTM_INC8: u32 = 0x00000400;  // INCR8, INCR4 or unspecified
                                            // length
pub const DMACTL0_BRSTM_INC16: u32 = 0x00000600;  // INCR16, INCR8, INCR4 or
                                            // unspecified length
pub const DMACTL0_ERR: u32 = 0x00000100;  // Bus Error Bit
pub const DMACTL0_EP_M: u32 = 0x000000F0;  // Endpoint number
pub const DMACTL0_IE: u32 = 0x00000008;  // DMA Interrupt Enable
pub const DMACTL0_MODE: u32 = 0x00000004;  // DMA Transfer Mode
pub const DMACTL0_DIR: u32 = 0x00000002;  // DMA Direction
pub const DMACTL0_ENABLE: u32 = 0x00000001;  // DMA Transfer Enable
pub const DMACTL0_EP_S: uint = 4;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMAADDR0 register.
//
//*****************************************************************************
pub const DMAADDR0_ADDR_M: u32 = 0xFFFFFFFC;  // DMA Address
pub const DMAADDR0_ADDR_S: uint = 2;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMACOUNT0
// register.
//
//*****************************************************************************
pub const DMACOUNT0_COUNT_M: u32 = 0xFFFFFFFC;  // DMA Count
pub const DMACOUNT0_COUNT_S: uint = 2;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMACTL1 register.
//
//*****************************************************************************
pub const DMACTL1_BRSTM_M: u32 = 0x00000600;  // Burst Mode
pub const DMACTL1_BRSTM_ANY: u32 = 0x00000000;  // Bursts of unspecified length
pub const DMACTL1_BRSTM_INC4: u32 = 0x00000200;  // INCR4 or unspecified length
pub const DMACTL1_BRSTM_INC8: u32 = 0x00000400;  // INCR8, INCR4 or unspecified
                                            // length
pub const DMACTL1_BRSTM_INC16: u32 = 0x00000600;  // INCR16, INCR8, INCR4 or
                                            // unspecified length
pub const DMACTL1_ERR: u32 = 0x00000100;  // Bus Error Bit
pub const DMACTL1_EP_M: u32 = 0x000000F0;  // Endpoint number
pub const DMACTL1_IE: u32 = 0x00000008;  // DMA Interrupt Enable
pub const DMACTL1_MODE: u32 = 0x00000004;  // DMA Transfer Mode
pub const DMACTL1_DIR: u32 = 0x00000002;  // DMA Direction
pub const DMACTL1_ENABLE: u32 = 0x00000001;  // DMA Transfer Enable
pub const DMACTL1_EP_S: uint = 4;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMAADDR1 register.
//
//*****************************************************************************
pub const DMAADDR1_ADDR_M: u32 = 0xFFFFFFFC;  // DMA Address
pub const DMAADDR1_ADDR_S: uint = 2;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMACOUNT1
// register.
//
//*****************************************************************************
pub const DMACOUNT1_COUNT_M: u32 = 0xFFFFFFFC;  // DMA Count
pub const DMACOUNT1_COUNT_S: uint = 2;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMACTL2 register.
//
//*****************************************************************************
pub const DMACTL2_BRSTM_M: u32 = 0x00000600;  // Burst Mode
pub const DMACTL2_BRSTM_ANY: u32 = 0x00000000;  // Bursts of unspecified length
pub const DMACTL2_BRSTM_INC4: u32 = 0x00000200;  // INCR4 or unspecified length
pub const DMACTL2_BRSTM_INC8: u32 = 0x00000400;  // INCR8, INCR4 or unspecified
                                            // length
pub const DMACTL2_BRSTM_INC16: u32 = 0x00000600;  // INCR16, INCR8, INCR4 or
                                            // unspecified length
pub const DMACTL2_ERR: u32 = 0x00000100;  // Bus Error Bit
pub const DMACTL2_EP_M: u32 = 0x000000F0;  // Endpoint number
pub const DMACTL2_IE: u32 = 0x00000008;  // DMA Interrupt Enable
pub const DMACTL2_MODE: u32 = 0x00000004;  // DMA Transfer Mode
pub const DMACTL2_DIR: u32 = 0x00000002;  // DMA Direction
pub const DMACTL2_ENABLE: u32 = 0x00000001;  // DMA Transfer Enable
pub const DMACTL2_EP_S: uint = 4;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMAADDR2 register.
//
//*****************************************************************************
pub const DMAADDR2_ADDR_M: u32 = 0xFFFFFFFC;  // DMA Address
pub const DMAADDR2_ADDR_S: uint = 2;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMACOUNT2
// register.
//
//*****************************************************************************
pub const DMACOUNT2_COUNT_M: u32 = 0xFFFFFFFC;  // DMA Count
pub const DMACOUNT2_COUNT_S: uint = 2;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMACTL3 register.
//
//*****************************************************************************
pub const DMACTL3_BRSTM_M: u32 = 0x00000600;  // Burst Mode
pub const DMACTL3_BRSTM_ANY: u32 = 0x00000000;  // Bursts of unspecified length
pub const DMACTL3_BRSTM_INC4: u32 = 0x00000200;  // INCR4 or unspecified length
pub const DMACTL3_BRSTM_INC8: u32 = 0x00000400;  // INCR8, INCR4 or unspecified
                                            // length
pub const DMACTL3_BRSTM_INC16: u32 = 0x00000600;  // INCR16, INCR8, INCR4 or
                                            // unspecified length
pub const DMACTL3_ERR: u32 = 0x00000100;  // Bus Error Bit
pub const DMACTL3_EP_M: u32 = 0x000000F0;  // Endpoint number
pub const DMACTL3_IE: u32 = 0x00000008;  // DMA Interrupt Enable
pub const DMACTL3_MODE: u32 = 0x00000004;  // DMA Transfer Mode
pub const DMACTL3_DIR: u32 = 0x00000002;  // DMA Direction
pub const DMACTL3_ENABLE: u32 = 0x00000001;  // DMA Transfer Enable
pub const DMACTL3_EP_S: uint = 4;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMAADDR3 register.
//
//*****************************************************************************
pub const DMAADDR3_ADDR_M: u32 = 0xFFFFFFFC;  // DMA Address
pub const DMAADDR3_ADDR_S: uint = 2;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMACOUNT3
// register.
//
//*****************************************************************************
pub const DMACOUNT3_COUNT_M: u32 = 0xFFFFFFFC;  // DMA Count
pub const DMACOUNT3_COUNT_S: uint = 2;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMACTL4 register.
//
//*****************************************************************************
pub const DMACTL4_BRSTM_M: u32 = 0x00000600;  // Burst Mode
pub const DMACTL4_BRSTM_ANY: u32 = 0x00000000;  // Bursts of unspecified length
pub const DMACTL4_BRSTM_INC4: u32 = 0x00000200;  // INCR4 or unspecified length
pub const DMACTL4_BRSTM_INC8: u32 = 0x00000400;  // INCR8, INCR4 or unspecified
                                            // length
pub const DMACTL4_BRSTM_INC16: u32 = 0x00000600;  // INCR16, INCR8, INCR4 or
                                            // unspecified length
pub const DMACTL4_ERR: u32 = 0x00000100;  // Bus Error Bit
pub const DMACTL4_EP_M: u32 = 0x000000F0;  // Endpoint number
pub const DMACTL4_IE: u32 = 0x00000008;  // DMA Interrupt Enable
pub const DMACTL4_MODE: u32 = 0x00000004;  // DMA Transfer Mode
pub const DMACTL4_DIR: u32 = 0x00000002;  // DMA Direction
pub const DMACTL4_ENABLE: u32 = 0x00000001;  // DMA Transfer Enable
pub const DMACTL4_EP_S: uint = 4;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMAADDR4 register.
//
//*****************************************************************************
pub const DMAADDR4_ADDR_M: u32 = 0xFFFFFFFC;  // DMA Address
pub const DMAADDR4_ADDR_S: uint = 2;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMACOUNT4
// register.
//
//*****************************************************************************
pub const DMACOUNT4_COUNT_M: u32 = 0xFFFFFFFC;  // DMA Count
pub const DMACOUNT4_COUNT_S: uint = 2;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMACTL5 register.
//
//*****************************************************************************
pub const DMACTL5_BRSTM_M: u32 = 0x00000600;  // Burst Mode
pub const DMACTL5_BRSTM_ANY: u32 = 0x00000000;  // Bursts of unspecified length
pub const DMACTL5_BRSTM_INC4: u32 = 0x00000200;  // INCR4 or unspecified length
pub const DMACTL5_BRSTM_INC8: u32 = 0x00000400;  // INCR8, INCR4 or unspecified
                                            // length
pub const DMACTL5_BRSTM_INC16: u32 = 0x00000600;  // INCR16, INCR8, INCR4 or
                                            // unspecified length
pub const DMACTL5_ERR: u32 = 0x00000100;  // Bus Error Bit
pub const DMACTL5_EP_M: u32 = 0x000000F0;  // Endpoint number
pub const DMACTL5_IE: u32 = 0x00000008;  // DMA Interrupt Enable
pub const DMACTL5_MODE: u32 = 0x00000004;  // DMA Transfer Mode
pub const DMACTL5_DIR: u32 = 0x00000002;  // DMA Direction
pub const DMACTL5_ENABLE: u32 = 0x00000001;  // DMA Transfer Enable
pub const DMACTL5_EP_S: uint = 4;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMAADDR5 register.
//
//*****************************************************************************
pub const DMAADDR5_ADDR_M: u32 = 0xFFFFFFFC;  // DMA Address
pub const DMAADDR5_ADDR_S: uint = 2;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMACOUNT5
// register.
//
//*****************************************************************************
pub const DMACOUNT5_COUNT_M: u32 = 0xFFFFFFFC;  // DMA Count
pub const DMACOUNT5_COUNT_S: uint = 2;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMACTL6 register.
//
//*****************************************************************************
pub const DMACTL6_BRSTM_M: u32 = 0x00000600;  // Burst Mode
pub const DMACTL6_BRSTM_ANY: u32 = 0x00000000;  // Bursts of unspecified length
pub const DMACTL6_BRSTM_INC4: u32 = 0x00000200;  // INCR4 or unspecified length
pub const DMACTL6_BRSTM_INC8: u32 = 0x00000400;  // INCR8, INCR4 or unspecified
                                            // length
pub const DMACTL6_BRSTM_INC16: u32 = 0x00000600;  // INCR16, INCR8, INCR4 or
                                            // unspecified length
pub const DMACTL6_ERR: u32 = 0x00000100;  // Bus Error Bit
pub const DMACTL6_EP_M: u32 = 0x000000F0;  // Endpoint number
pub const DMACTL6_IE: u32 = 0x00000008;  // DMA Interrupt Enable
pub const DMACTL6_MODE: u32 = 0x00000004;  // DMA Transfer Mode
pub const DMACTL6_DIR: u32 = 0x00000002;  // DMA Direction
pub const DMACTL6_ENABLE: u32 = 0x00000001;  // DMA Transfer Enable
pub const DMACTL6_EP_S: uint = 4;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMAADDR6 register.
//
//*****************************************************************************
pub const DMAADDR6_ADDR_M: u32 = 0xFFFFFFFC;  // DMA Address
pub const DMAADDR6_ADDR_S: uint = 2;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMACOUNT6
// register.
//
//*****************************************************************************
pub const DMACOUNT6_COUNT_M: u32 = 0xFFFFFFFC;  // DMA Count
pub const DMACOUNT6_COUNT_S: uint = 2;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMACTL7 register.
//
//*****************************************************************************
pub const DMACTL7_BRSTM_M: u32 = 0x00000600;  // Burst Mode
pub const DMACTL7_BRSTM_ANY: u32 = 0x00000000;  // Bursts of unspecified length
pub const DMACTL7_BRSTM_INC4: u32 = 0x00000200;  // INCR4 or unspecified length
pub const DMACTL7_BRSTM_INC8: u32 = 0x00000400;  // INCR8, INCR4 or unspecified
                                            // length
pub const DMACTL7_BRSTM_INC16: u32 = 0x00000600;  // INCR16, INCR8, INCR4 or
                                            // unspecified length
pub const DMACTL7_ERR: u32 = 0x00000100;  // Bus Error Bit
pub const DMACTL7_EP_M: u32 = 0x000000F0;  // Endpoint number
pub const DMACTL7_IE: u32 = 0x00000008;  // DMA Interrupt Enable
pub const DMACTL7_MODE: u32 = 0x00000004;  // DMA Transfer Mode
pub const DMACTL7_DIR: u32 = 0x00000002;  // DMA Direction
pub const DMACTL7_ENABLE: u32 = 0x00000001;  // DMA Transfer Enable
pub const DMACTL7_EP_S: uint = 4;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMAADDR7 register.
//
//*****************************************************************************
pub const DMAADDR7_ADDR_M: u32 = 0xFFFFFFFC;  // DMA Address
pub const DMAADDR7_ADDR_S: uint = 2;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMACOUNT7
// register.
//
//*****************************************************************************
pub const DMACOUNT7_COUNT_M: u32 = 0xFFFFFFFC;  // DMA Count
pub const DMACOUNT7_COUNT_S: uint = 2;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RQPKTCOUNT1
// register.
//
//*****************************************************************************
pub const RQPKTCOUNT1_M: u32 = 0x0000FFFF;  // Block Transfer Packet Count
pub const RQPKTCOUNT1_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RQPKTCOUNT2
// register.
//
//*****************************************************************************
pub const RQPKTCOUNT2_M: u32 = 0x0000FFFF;  // Block Transfer Packet Count
pub const RQPKTCOUNT2_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RQPKTCOUNT3
// register.
//
//*****************************************************************************
pub const RQPKTCOUNT3_M: u32 = 0x0000FFFF;  // Block Transfer Packet Count
pub const RQPKTCOUNT3_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RQPKTCOUNT4
// register.
//
//*****************************************************************************
pub const RQPKTCOUNT4_COUNT_M: u32 = 0x0000FFFF;  // Block Transfer Packet Count
pub const RQPKTCOUNT4_COUNT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RQPKTCOUNT5
// register.
//
//*****************************************************************************
pub const RQPKTCOUNT5_COUNT_M: u32 = 0x0000FFFF;  // Block Transfer Packet Count
pub const RQPKTCOUNT5_COUNT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RQPKTCOUNT6
// register.
//
//*****************************************************************************
pub const RQPKTCOUNT6_COUNT_M: u32 = 0x0000FFFF;  // Block Transfer Packet Count
pub const RQPKTCOUNT6_COUNT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RQPKTCOUNT7
// register.
//
//*****************************************************************************
pub const RQPKTCOUNT7_COUNT_M: u32 = 0x0000FFFF;  // Block Transfer Packet Count
pub const RQPKTCOUNT7_COUNT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXDPKTBUFDIS
// register.
//
//*****************************************************************************
pub const RXDPKTBUFDIS_EP7: u32 = 0x00000080;  // EP7 RX Double-Packet Buffer
                                            // Disable
pub const RXDPKTBUFDIS_EP6: u32 = 0x00000040;  // EP6 RX Double-Packet Buffer
                                            // Disable
pub const RXDPKTBUFDIS_EP5: u32 = 0x00000020;  // EP5 RX Double-Packet Buffer
                                            // Disable
pub const RXDPKTBUFDIS_EP4: u32 = 0x00000010;  // EP4 RX Double-Packet Buffer
                                            // Disable
pub const RXDPKTBUFDIS_EP3: u32 = 0x00000008;  // EP3 RX Double-Packet Buffer
                                            // Disable
pub const RXDPKTBUFDIS_EP2: u32 = 0x00000004;  // EP2 RX Double-Packet Buffer
                                            // Disable
pub const RXDPKTBUFDIS_EP1: u32 = 0x00000002;  // EP1 RX Double-Packet Buffer
                                            // Disable

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXDPKTBUFDIS
// register.
//
//*****************************************************************************
pub const TXDPKTBUFDIS_EP7: u32 = 0x00000080;  // EP7 TX Double-Packet Buffer
                                            // Disable
pub const TXDPKTBUFDIS_EP6: u32 = 0x00000040;  // EP6 TX Double-Packet Buffer
                                            // Disable
pub const TXDPKTBUFDIS_EP5: u32 = 0x00000020;  // EP5 TX Double-Packet Buffer
                                            // Disable
pub const TXDPKTBUFDIS_EP4: u32 = 0x00000010;  // EP4 TX Double-Packet Buffer
                                            // Disable
pub const TXDPKTBUFDIS_EP3: u32 = 0x00000008;  // EP3 TX Double-Packet Buffer
                                            // Disable
pub const TXDPKTBUFDIS_EP2: u32 = 0x00000004;  // EP2 TX Double-Packet Buffer
                                            // Disable
pub const TXDPKTBUFDIS_EP1: u32 = 0x00000002;  // EP1 TX Double-Packet Buffer
                                            // Disable

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_CTO register.
//
//*****************************************************************************
pub const CTO_CCTV_M: u32 = 0x0000FFFF;  // Configurable Chirp Timeout Value
pub const CTO_CCTV_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_HHSRTN register.
//
//*****************************************************************************
pub const HHSRTN_HHSRTN_M: u32 = 0x0000FFFF;  // HIgh Speed to UTM Operating
                                            // Delay
pub const HHSRTN_HHSRTN_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_HSBT register.
//
//*****************************************************************************
pub const HSBT_HSBT_M: u32 = 0x0000000F;  // High Speed Timeout Adder
pub const HSBT_HSBT_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_LPMATTR register.
//
//*****************************************************************************
pub const LPMATTR_ENDPT_M: u32 = 0x0000F000;  // Endpoint
pub const LPMATTR_RMTWAK: u32 = 0x00000100;  // Remote Wake
pub const LPMATTR_HIRD_M: u32 = 0x000000F0;  // Host Initiated Resume Duration
pub const LPMATTR_LS_M: u32 = 0x0000000F;  // Link State
pub const LPMATTR_LS_L1: u32 = 0x00000001;  // Sleep State (L1)
pub const LPMATTR_ENDPT_S: uint = 12;
pub const LPMATTR_HIRD_S: uint = 4;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_LPMCNTRL register.
//
//*****************************************************************************
pub const LPMCNTRL_NAK: u32 = 0x00000010;  // LPM NAK
pub const LPMCNTRL_EN_M: u32 = 0x0000000C;  // LPM Enable
pub const LPMCNTRL_EN_NONE: u32 = 0x00000000;  // LPM and Extended transactions
                                            // are not supported. In this case,
                                            // the USB does not respond to LPM
                                            // transactions and LPM
                                            // transactions cause a timeout
pub const LPMCNTRL_EN_EXT: u32 = 0x00000004;  // LPM is not supported but
                                            // extended transactions are
                                            // supported. In this case, the USB
                                            // does respond to an LPM
                                            // transaction with a STALL
pub const LPMCNTRL_EN_LPMEXT: u32 = 0x0000000C;  // The USB supports LPM extended
                                            // transactions. In this case, the
                                            // USB responds with a NYET or an
                                            // ACK as determined by the value
                                            // of TXLPM and other conditions
pub const LPMCNTRL_RES: u32 = 0x00000002;  // LPM Resume
pub const LPMCNTRL_TXLPM: u32 = 0x00000001;  // Transmit LPM Transaction Enable

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_LPMIM register.
//
//*****************************************************************************
pub const LPMIM_ERR: u32 = 0x00000020;  // LPM Error Interrupt Mask
pub const LPMIM_RES: u32 = 0x00000010;  // LPM Resume Interrupt Mask
pub const LPMIM_NC: u32 = 0x00000008;  // LPM NC Interrupt Mask
pub const LPMIM_ACK: u32 = 0x00000004;  // LPM ACK Interrupt Mask
pub const LPMIM_NY: u32 = 0x00000002;  // LPM NY Interrupt Mask
pub const LPMIM_STALL: u32 = 0x00000001;  // LPM STALL Interrupt Mask

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_LPMRIS register.
//
//*****************************************************************************
pub const LPMRIS_ERR: u32 = 0x00000020;  // LPM Interrupt Status
pub const LPMRIS_RES: u32 = 0x00000010;  // LPM Resume Interrupt Status
pub const LPMRIS_NC: u32 = 0x00000008;  // LPM NC Interrupt Status
pub const LPMRIS_ACK: u32 = 0x00000004;  // LPM ACK Interrupt Status
pub const LPMRIS_NY: u32 = 0x00000002;  // LPM NY Interrupt Status
pub const LPMRIS_LPMST: u32 = 0x00000001;  // LPM STALL Interrupt Status

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_LPMFADDR register.
//
//*****************************************************************************
pub const LPMFADDR_ADDR_M: u32 = 0x0000007F;  // LPM Function Address
pub const LPMFADDR_ADDR_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_EPC register.
//
//*****************************************************************************
pub const EPC_PFLTACT_M: u32 = 0x00000300;  // Power Fault Action
pub const EPC_PFLTACT_UNCHG: u32 = 0x00000000;  // Unchanged
pub const EPC_PFLTACT_TRIS: u32 = 0x00000100;  // Tristate
pub const EPC_PFLTACT_LOW: u32 = 0x00000200;  // Low
pub const EPC_PFLTACT_HIGH: u32 = 0x00000300;  // High
pub const EPC_PFLTAEN: u32 = 0x00000040;  // Power Fault Action Enable
pub const EPC_PFLTSEN_HIGH: u32 = 0x00000020;  // Power Fault Sense
pub const EPC_PFLTEN: u32 = 0x00000010;  // Power Fault Input Enable
pub const EPC_EPENDE: u32 = 0x00000004;  // EPEN Drive Enable
pub const EPC_EPEN_M: u32 = 0x00000003;  // External Power Supply Enable
                                            // Configuration
pub const EPC_EPEN_LOW: u32 = 0x00000000;  // Power Enable Active Low
pub const EPC_EPEN_HIGH: u32 = 0x00000001;  // Power Enable Active High
pub const EPC_EPEN_VBLOW: u32 = 0x00000002;  // Power Enable High if VBUS Low
                                            // (OTG only)
pub const EPC_EPEN_VBHIGH: u32 = 0x00000003;  // Power Enable High if VBUS High
                                            // (OTG only)

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_EPCRIS register.
//
//*****************************************************************************
pub const EPCRIS_PF: u32 = 0x00000001;  // USB Power Fault Interrupt Status

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_EPCIM register.
//
//*****************************************************************************
pub const EPCIM_PF: u32 = 0x00000001;  // USB Power Fault Interrupt Mask

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_EPCISC register.
//
//*****************************************************************************
pub const EPCISC_PF: u32 = 0x00000001;  // USB Power Fault Interrupt Status
                                            // and Clear

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DRRIS register.
//
//*****************************************************************************
pub const DRRIS_RESUME: u32 = 0x00000001;  // RESUME Interrupt Status

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DRIM register.
//
//*****************************************************************************
pub const DRIM_RESUME: u32 = 0x00000001;  // RESUME Interrupt Mask

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DRISC register.
//
//*****************************************************************************
pub const DRISC_RESUME: u32 = 0x00000001;  // RESUME Interrupt Status and
                                            // Clear

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_GPCS register.
//
//*****************************************************************************
pub const GPCS_DEVMOD_M: u32 = 0x00000007;  // Device Mode
pub const GPCS_DEVMOD_OTG: u32 = 0x00000000;  // Use USB0VBUS and USB0ID pin
pub const GPCS_DEVMOD_HOST: u32 = 0x00000002;  // Force USB0VBUS and USB0ID low
pub const GPCS_DEVMOD_DEV: u32 = 0x00000003;  // Force USB0VBUS and USB0ID high
pub const GPCS_DEVMOD_HOSTVBUS: u32 = 0x00000004; // Use USB0VBUS and force USB0ID
                                            // low
pub const GPCS_DEVMOD_DEVVBUS: u32 = 0x00000005;  // Use USB0VBUS and force USB0ID
                                            // high
pub const GPCS_DEVMODOTG: u32 = 0x00000002;  // Enable Device Mode
pub const GPCS_DEVMOD: u32 = 0x00000001;  // Device Mode

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_VDC register.
//
//*****************************************************************************
pub const VDC_VBDEN: u32 = 0x00000001;  // VBUS Droop Enable

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_VDCRIS register.
//
//*****************************************************************************
pub const VDCRIS_VD: u32 = 0x00000001;  // VBUS Droop Raw Interrupt Status

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_VDCIM register.
//
//*****************************************************************************
pub const VDCIM_VD: u32 = 0x00000001;  // VBUS Droop Interrupt Mask

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_VDCISC register.
//
//*****************************************************************************
pub const VDCISC_VD: u32 = 0x00000001;  // VBUS Droop Interrupt Status and
                                            // Clear

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_IDVRIS register.
//
//*****************************************************************************
pub const IDVRIS_ID: u32 = 0x00000001;  // ID Valid Detect Raw Interrupt
                                            // Status

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_IDVIM register.
//
//*****************************************************************************
pub const IDVIM_ID: u32 = 0x00000001;  // ID Valid Detect Interrupt Mask

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_IDVISC register.
//
//*****************************************************************************
pub const IDVISC_ID: u32 = 0x00000001;  // ID Valid Detect Interrupt Status
                                            // and Clear

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMASEL register.
//
//*****************************************************************************
pub const DMASEL_DMACTX_M: u32 = 0x00F00000;  // DMA C TX Select
pub const DMASEL_DMACRX_M: u32 = 0x000F0000;  // DMA C RX Select
pub const DMASEL_DMABTX_M: u32 = 0x0000F000;  // DMA B TX Select
pub const DMASEL_DMABRX_M: u32 = 0x00000F00;  // DMA B RX Select
pub const DMASEL_DMAATX_M: u32 = 0x000000F0;  // DMA A TX Select
pub const DMASEL_DMAARX_M: u32 = 0x0000000F;  // DMA A RX Select
pub const DMASEL_DMACTX_S: uint = 20;
pub const DMASEL_DMACRX_S: uint = 16;
pub const DMASEL_DMABTX_S: uint = 12;
pub const DMASEL_DMABRX_S: uint = 8;
pub const DMASEL_DMAATX_S: uint = 4;
pub const DMASEL_DMAARX_S: uint = 0;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_PP register.
//
//*****************************************************************************
pub const PP_ECNT_M: u32 = 0x0000FF00;  // Endpoint Count
pub const PP_USB_M: u32 = 0x000000C0;  // USB Capability
pub const PP_USB_DEVICE: u32 = 0x00000040;  // DEVICE
pub const PP_USB_HOSTDEVICE: u32 = 0x00000080;  // HOST
pub const PP_USB_OTG: u32 = 0x000000C0;  // OTG
pub const PP_ULPI: u32 = 0x00000020;  // ULPI Present
pub const PP_PHY: u32 = 0x00000010;  // PHY Present
pub const PP_TYPE_M: u32 = 0x0000000F;  // Controller Type
pub const PP_TYPE_0: u32 = 0x00000000;  // The first-generation USB
                                            // controller
pub const PP_TYPE_1: u32 = 0x00000001;  // Second-generation USB
                                            // controller.The controller
                                            // implemented in post Icestorm
                                            // devices that use the 3.0 version
                                            // of the Mentor controller
pub const PP_ECNT_S: uint = 8;

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_PC register.
//
//*****************************************************************************
pub const PC_ULPIEN: u32 = 0x00010000;  // ULPI Enable

//*****************************************************************************
//
// The following are defines for the bit fields in the USB_O_CC register.
//
//*****************************************************************************
pub const CC_CLKEN: u32 = 0x00000200;  // USB Clock Enable
pub const CC_CSD: u32 = 0x00000100;  // Clock Source/Direction
pub const CC_CLKDIV_M: u32 = 0x0000000F;  // PLL Clock Divisor
pub const CC_CLKDIV_S: uint = 0;
