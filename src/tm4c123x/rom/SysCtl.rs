use chip::rom::getfun;

//*****************************************************************************
//
// The following are values that can be passed to the
// SysCtlPeripheralPresent(), SysCtlPeripheralEnable(),
// SysCtlPeripheralDisable(), and SysCtlPeripheralReset() APIs as the
// ui32Peripheral parameter.  The peripherals in the fourth group (upper nibble
// is 3) can only be used with the SysCtlPeripheralPresent() API.
//
//*****************************************************************************
pub const PERIPH_ADC0: u32 = 0xf0003800;  // ADC 0
pub const PERIPH_ADC1: u32 = 0xf0003801;  // ADC 1
pub const PERIPH_CAN0: u32 = 0xf0003400;  // CAN 0
pub const PERIPH_CAN1: u32 = 0xf0003401;  // CAN 1
pub const PERIPH_COMP0: u32 = 0xf0003c00;  // Analog Comparator Module 0
pub const PERIPH_EMAC0: u32 = 0xf0009c00;  // Ethernet MAC0
pub const PERIPH_EPHY0: u32 = 0xf0003000;  // Ethernet PHY0
pub const PERIPH_EPI0: u32 = 0xf0001000;  // EPI0
pub const PERIPH_GPIOA: u32 = 0xf0000800;  // GPIO A
pub const PERIPH_GPIOB: u32 = 0xf0000801;  // GPIO B
pub const PERIPH_GPIOC: u32 = 0xf0000802;  // GPIO C
pub const PERIPH_GPIOD: u32 = 0xf0000803;  // GPIO D
pub const PERIPH_GPIOE: u32 = 0xf0000804;  // GPIO E
pub const PERIPH_GPIOF: u32 = 0xf0000805;  // GPIO F
pub const PERIPH_GPIOG: u32 = 0xf0000806;  // GPIO G
pub const PERIPH_GPIOH: u32 = 0xf0000807;  // GPIO H
pub const PERIPH_GPIOJ: u32 = 0xf0000808;  // GPIO J
pub const PERIPH_HIBERNATE: u32 = 0xf0001400;  // Hibernation module
pub const PERIPH_CCM0: u32 = 0xf0007400;  // CCM 0
pub const PERIPH_EEPROM0: u32 = 0xf0005800;  // EEPROM 0
pub const PERIPH_FAN0: u32 = 0xf0005400;  // FAN 0
pub const PERIPH_FAN1: u32 = 0xf0005401;  // FAN 1
pub const PERIPH_GPIOK: u32 = 0xf0000809;  // GPIO K
pub const PERIPH_GPIOL: u32 = 0xf000080a;  // GPIO L
pub const PERIPH_GPIOM: u32 = 0xf000080b;  // GPIO M
pub const PERIPH_GPION: u32 = 0xf000080c;  // GPIO N
pub const PERIPH_GPIOP: u32 = 0xf000080d;  // GPIO P
pub const PERIPH_GPIOQ: u32 = 0xf000080e;  // GPIO Q
pub const PERIPH_GPIOR: u32 = 0xf000080f;  // GPIO R
pub const PERIPH_GPIOS: u32 = 0xf0000810;  // GPIO S
pub const PERIPH_GPIOT: u32 = 0xf0000811;  // GPIO T
pub const PERIPH_I2C0: u32 = 0xf0002000;  // I2C 0
pub const PERIPH_I2C1: u32 = 0xf0002001;  // I2C 1
pub const PERIPH_I2C2: u32 = 0xf0002002;  // I2C 2
pub const PERIPH_I2C3: u32 = 0xf0002003;  // I2C 3
pub const PERIPH_I2C4: u32 = 0xf0002004;  // I2C 4
pub const PERIPH_I2C5: u32 = 0xf0002005;  // I2C 5
pub const PERIPH_I2C6: u32 = 0xf0002006;  // I2C 6
pub const PERIPH_I2C7: u32 = 0xf0002007;  // I2C 7
pub const PERIPH_I2C8: u32 = 0xf0002008;  // I2C 8
pub const PERIPH_I2C9: u32 = 0xf0002009;  // I2C 9
pub const PERIPH_LCD0: u32 = 0xf0009000;  // LCD 0
pub const PERIPH_ONEWIRE0: u32 = 0xf0009800;  // One Wire 0
pub const PERIPH_PWM0: u32 = 0xf0004000;  // PWM 0
pub const PERIPH_PWM1: u32 = 0xf0004001;  // PWM 1
pub const PERIPH_QEI0: u32 = 0xf0004400;  // QEI 0
pub const PERIPH_QEI1: u32 = 0xf0004401;  // QEI 1
pub const PERIPH_SSI0: u32 = 0xf0001c00;  // SSI 0
pub const PERIPH_SSI1: u32 = 0xf0001c01;  // SSI 1
pub const PERIPH_SSI2: u32 = 0xf0001c02;  // SSI 2
pub const PERIPH_SSI3: u32 = 0xf0001c03;  // SSI 3
pub const PERIPH_TIMER0: u32 = 0xf0000400;  // Timer 0
pub const PERIPH_TIMER1: u32 = 0xf0000401;  // Timer 1
pub const PERIPH_TIMER2: u32 = 0xf0000402;  // Timer 2
pub const PERIPH_TIMER3: u32 = 0xf0000403;  // Timer 3
pub const PERIPH_TIMER4: u32 = 0xf0000404;  // Timer 4
pub const PERIPH_TIMER5: u32 = 0xf0000405;  // Timer 5
pub const PERIPH_TIMER6: u32 = 0xf0000406;  // Timer 6
pub const PERIPH_TIMER7: u32 = 0xf0000407;  // Timer 7
pub const PERIPH_UART0: u32 = 0xf0001800;  // UART 0
pub const PERIPH_UART1: u32 = 0xf0001801;  // UART 1
pub const PERIPH_UART2: u32 = 0xf0001802;  // UART 2
pub const PERIPH_UART3: u32 = 0xf0001803;  // UART 3
pub const PERIPH_UART4: u32 = 0xf0001804;  // UART 4
pub const PERIPH_UART5: u32 = 0xf0001805;  // UART 5
pub const PERIPH_UART6: u32 = 0xf0001806;  // UART 6
pub const PERIPH_UART7: u32 = 0xf0001807;  // UART 7
pub const PERIPH_UDMA: u32 = 0xf0000c00;  // uDMA
pub const PERIPH_USB0: u32 = 0xf0002800;  // USB 0
pub const PERIPH_WDOG0: u32 = 0xf0000000;  // Watchdog 0
pub const PERIPH_WDOG1: u32 = 0xf0000001;  // Watchdog 1
pub const PERIPH_WTIMER0: u32 = 0xf0005c00;  // Wide Timer 0
pub const PERIPH_WTIMER1: u32 = 0xf0005c01;  // Wide Timer 1
pub const PERIPH_WTIMER2: u32 = 0xf0005c02;  // Wide Timer 2
pub const PERIPH_WTIMER3: u32 = 0xf0005c03;  // Wide Timer 3
pub const PERIPH_WTIMER4: u32 = 0xf0005c04;  // Wide Timer 4
pub const PERIPH_WTIMER5: u32 = 0xf0005c05;  // Wide Timer 5

//*****************************************************************************
//
// The following are values that can be passed to the SysCtlLDOSleepSet() and
// SysCtlLDODeepSleepSet() APIs as the ui32Voltage value, or returned by the
// SysCtlLDOSleepGet() and SysCtlLDODeepSleepGet() APIs.
//
//*****************************************************************************
pub const LDO_0_90V: u32 = 0x80000012;  // LDO output of 0.90V
pub const LDO_0_95V: u32 = 0x80000013;  // LDO output of 0.95V
pub const LDO_1_00V: u32 = 0x80000014;  // LDO output of 1.00V
pub const LDO_1_05V: u32 = 0x80000015;  // LDO output of 1.05V
pub const LDO_1_10V: u32 = 0x80000016;  // LDO output of 1.10V
pub const LDO_1_15V: u32 = 0x80000017;  // LDO output of 1.15V
pub const LDO_1_20V: u32 = 0x80000018;  // LDO output of 1.20V

//*****************************************************************************
//
// The following are values that can be passed to the SysCtlIntEnable(),
// SysCtlIntDisable(), and SysCtlIntClear() APIs, or returned in the bit mask
// by the SysCtlIntStatus() API.
//
//*****************************************************************************
pub const INT_BOR0: u32 = 0x00000800;  // VDD under BOR0
pub const INT_VDDA_OK: u32 = 0x00000400;  // VDDA Power OK
pub const INT_MOSC_PUP: u32 = 0x00000100;  // MOSC power-up interrupt
pub const INT_USBPLL_LOCK: u32 = 0x00000080;  // USB PLL lock interrupt
pub const INT_PLL_LOCK: u32 = 0x00000040;  // PLL lock interrupt
pub const INT_MOSC_FAIL: u32 = 0x00000008;  // Main oscillator failure int
pub const INT_BOR1: u32 = 0x00000002;  // VDD under BOR1
pub const INT_BOR: u32 = 0x00000002;  // Brown out interrupt

//*****************************************************************************
//
// The following are values that can be passed to the SysCtlResetCauseClear()
// API or returned by the SysCtlResetCauseGet() API.
//
//*****************************************************************************
pub const CAUSE_HSRVREQ: u32 = 0x00001000;  // Hardware System Service Request
pub const CAUSE_HIB: u32 = 0x00000040;  // Hibernate reset
pub const CAUSE_WDOG1: u32 = 0x00000020;  // Watchdog 1 reset
pub const CAUSE_SW: u32 = 0x00000010;  // Software reset
pub const CAUSE_WDOG0: u32 = 0x00000008;  // Watchdog 0 reset
pub const CAUSE_BOR: u32 = 0x00000004;  // Brown-out reset
pub const CAUSE_POR: u32 = 0x00000002;  // Power on reset
pub const CAUSE_EXT: u32 = 0x00000001;  // External reset

//*****************************************************************************
//
// The following are values that can be passed to the SysCtlBrownOutConfigSet()
// API as the ui32Config parameter.
//
//*****************************************************************************
pub const BOR_RESET: u32 = 0x00000002;  // Reset instead of interrupting
pub const BOR_RESAMPLE: u32 = 0x00000001;  // Resample BOR before asserting

//*****************************************************************************
//
// The following are values that can be passed to the SysCtlPWMClockSet() API
// as the ui32Config parameter, and can be returned by the SysCtlPWMClockGet()
// API.
//
//*****************************************************************************
pub const PWMDIV_1: u32 = 0x00000000;  // PWM clock is processor clock /1
pub const PWMDIV_2: u32 = 0x00100000;  // PWM clock is processor clock /2
pub const PWMDIV_4: u32 = 0x00120000;  // PWM clock is processor clock /4
pub const PWMDIV_8: u32 = 0x00140000;  // PWM clock is processor clock /8
pub const PWMDIV_16: u32 = 0x00160000;  // PWM clock is processor clock /16
pub const PWMDIV_32: u32 = 0x00180000;  // PWM clock is processor clock /32
pub const PWMDIV_64: u32 = 0x001A0000;  // PWM clock is processor clock /64

//*****************************************************************************
//
// The following are values that can be passed to the SysCtlClockSet() API as
// the ui32Config parameter.
//
//*****************************************************************************
pub const SYSDIV_1: u32 = 0x07800000;  // Processor clock is osc/pll /1
pub const SYSDIV_2: u32 = 0x00C00000;  // Processor clock is osc/pll /2
pub const SYSDIV_3: u32 = 0x01400000;  // Processor clock is osc/pll /3
pub const SYSDIV_4: u32 = 0x01C00000;  // Processor clock is osc/pll /4
pub const SYSDIV_5: u32 = 0x02400000;  // Processor clock is osc/pll /5
pub const SYSDIV_6: u32 = 0x02C00000;  // Processor clock is osc/pll /6
pub const SYSDIV_7: u32 = 0x03400000;  // Processor clock is osc/pll /7
pub const SYSDIV_8: u32 = 0x03C00000;  // Processor clock is osc/pll /8
pub const SYSDIV_9: u32 = 0x04400000;  // Processor clock is osc/pll /9
pub const SYSDIV_10: u32 = 0x04C00000;  // Processor clock is osc/pll /10
pub const SYSDIV_11: u32 = 0x05400000;  // Processor clock is osc/pll /11
pub const SYSDIV_12: u32 = 0x05C00000;  // Processor clock is osc/pll /12
pub const SYSDIV_13: u32 = 0x06400000;  // Processor clock is osc/pll /13
pub const SYSDIV_14: u32 = 0x06C00000;  // Processor clock is osc/pll /14
pub const SYSDIV_15: u32 = 0x07400000;  // Processor clock is osc/pll /15
pub const SYSDIV_16: u32 = 0x07C00000;  // Processor clock is osc/pll /16
pub const SYSDIV_17: u32 = 0x88400000;  // Processor clock is osc/pll /17
pub const SYSDIV_18: u32 = 0x88C00000;  // Processor clock is osc/pll /18
pub const SYSDIV_19: u32 = 0x89400000;  // Processor clock is osc/pll /19
pub const SYSDIV_20: u32 = 0x89C00000;  // Processor clock is osc/pll /20
pub const SYSDIV_21: u32 = 0x8A400000;  // Processor clock is osc/pll /21
pub const SYSDIV_22: u32 = 0x8AC00000;  // Processor clock is osc/pll /22
pub const SYSDIV_23: u32 = 0x8B400000;  // Processor clock is osc/pll /23
pub const SYSDIV_24: u32 = 0x8BC00000;  // Processor clock is osc/pll /24
pub const SYSDIV_25: u32 = 0x8C400000;  // Processor clock is osc/pll /25
pub const SYSDIV_26: u32 = 0x8CC00000;  // Processor clock is osc/pll /26
pub const SYSDIV_27: u32 = 0x8D400000;  // Processor clock is osc/pll /27
pub const SYSDIV_28: u32 = 0x8DC00000;  // Processor clock is osc/pll /28
pub const SYSDIV_29: u32 = 0x8E400000;  // Processor clock is osc/pll /29
pub const SYSDIV_30: u32 = 0x8EC00000;  // Processor clock is osc/pll /30
pub const SYSDIV_31: u32 = 0x8F400000;  // Processor clock is osc/pll /31
pub const SYSDIV_32: u32 = 0x8FC00000;  // Processor clock is osc/pll /32
pub const SYSDIV_33: u32 = 0x90400000;  // Processor clock is osc/pll /33
pub const SYSDIV_34: u32 = 0x90C00000;  // Processor clock is osc/pll /34
pub const SYSDIV_35: u32 = 0x91400000;  // Processor clock is osc/pll /35
pub const SYSDIV_36: u32 = 0x91C00000;  // Processor clock is osc/pll /36
pub const SYSDIV_37: u32 = 0x92400000;  // Processor clock is osc/pll /37
pub const SYSDIV_38: u32 = 0x92C00000;  // Processor clock is osc/pll /38
pub const SYSDIV_39: u32 = 0x93400000;  // Processor clock is osc/pll /39
pub const SYSDIV_40: u32 = 0x93C00000;  // Processor clock is osc/pll /40
pub const SYSDIV_41: u32 = 0x94400000;  // Processor clock is osc/pll /41
pub const SYSDIV_42: u32 = 0x94C00000;  // Processor clock is osc/pll /42
pub const SYSDIV_43: u32 = 0x95400000;  // Processor clock is osc/pll /43
pub const SYSDIV_44: u32 = 0x95C00000;  // Processor clock is osc/pll /44
pub const SYSDIV_45: u32 = 0x96400000;  // Processor clock is osc/pll /45
pub const SYSDIV_46: u32 = 0x96C00000;  // Processor clock is osc/pll /46
pub const SYSDIV_47: u32 = 0x97400000;  // Processor clock is osc/pll /47
pub const SYSDIV_48: u32 = 0x97C00000;  // Processor clock is osc/pll /48
pub const SYSDIV_49: u32 = 0x98400000;  // Processor clock is osc/pll /49
pub const SYSDIV_50: u32 = 0x98C00000;  // Processor clock is osc/pll /50
pub const SYSDIV_51: u32 = 0x99400000;  // Processor clock is osc/pll /51
pub const SYSDIV_52: u32 = 0x99C00000;  // Processor clock is osc/pll /52
pub const SYSDIV_53: u32 = 0x9A400000;  // Processor clock is osc/pll /53
pub const SYSDIV_54: u32 = 0x9AC00000;  // Processor clock is osc/pll /54
pub const SYSDIV_55: u32 = 0x9B400000;  // Processor clock is osc/pll /55
pub const SYSDIV_56: u32 = 0x9BC00000;  // Processor clock is osc/pll /56
pub const SYSDIV_57: u32 = 0x9C400000;  // Processor clock is osc/pll /57
pub const SYSDIV_58: u32 = 0x9CC00000;  // Processor clock is osc/pll /58
pub const SYSDIV_59: u32 = 0x9D400000;  // Processor clock is osc/pll /59
pub const SYSDIV_60: u32 = 0x9DC00000;  // Processor clock is osc/pll /60
pub const SYSDIV_61: u32 = 0x9E400000;  // Processor clock is osc/pll /61
pub const SYSDIV_62: u32 = 0x9EC00000;  // Processor clock is osc/pll /62
pub const SYSDIV_63: u32 = 0x9F400000;  // Processor clock is osc/pll /63
pub const SYSDIV_64: u32 = 0x9FC00000;  // Processor clock is osc/pll /64
pub const SYSDIV_2_5: u32 = 0xC1000000;  // Processor clock is pll / 2.5
pub const SYSDIV_3_5: u32 = 0xC1800000;  // Processor clock is pll / 3.5
pub const SYSDIV_4_5: u32 = 0xC2000000;  // Processor clock is pll / 4.5
pub const SYSDIV_5_5: u32 = 0xC2800000;  // Processor clock is pll / 5.5
pub const SYSDIV_6_5: u32 = 0xC3000000;  // Processor clock is pll / 6.5
pub const SYSDIV_7_5: u32 = 0xC3800000;  // Processor clock is pll / 7.5
pub const SYSDIV_8_5: u32 = 0xC4000000;  // Processor clock is pll / 8.5
pub const SYSDIV_9_5: u32 = 0xC4800000;  // Processor clock is pll / 9.5
pub const SYSDIV_10_5: u32 = 0xC5000000;  // Processor clock is pll / 10.5
pub const SYSDIV_11_5: u32 = 0xC5800000;  // Processor clock is pll / 11.5
pub const SYSDIV_12_5: u32 = 0xC6000000;  // Processor clock is pll / 12.5
pub const SYSDIV_13_5: u32 = 0xC6800000;  // Processor clock is pll / 13.5
pub const SYSDIV_14_5: u32 = 0xC7000000;  // Processor clock is pll / 14.5
pub const SYSDIV_15_5: u32 = 0xC7800000;  // Processor clock is pll / 15.5
pub const SYSDIV_16_5: u32 = 0xC8000000;  // Processor clock is pll / 16.5
pub const SYSDIV_17_5: u32 = 0xC8800000;  // Processor clock is pll / 17.5
pub const SYSDIV_18_5: u32 = 0xC9000000;  // Processor clock is pll / 18.5
pub const SYSDIV_19_5: u32 = 0xC9800000;  // Processor clock is pll / 19.5
pub const SYSDIV_20_5: u32 = 0xCA000000;  // Processor clock is pll / 20.5
pub const SYSDIV_21_5: u32 = 0xCA800000;  // Processor clock is pll / 21.5
pub const SYSDIV_22_5: u32 = 0xCB000000;  // Processor clock is pll / 22.5
pub const SYSDIV_23_5: u32 = 0xCB800000;  // Processor clock is pll / 23.5
pub const SYSDIV_24_5: u32 = 0xCC000000;  // Processor clock is pll / 24.5
pub const SYSDIV_25_5: u32 = 0xCC800000;  // Processor clock is pll / 25.5
pub const SYSDIV_26_5: u32 = 0xCD000000;  // Processor clock is pll / 26.5
pub const SYSDIV_27_5: u32 = 0xCD800000;  // Processor clock is pll / 27.5
pub const SYSDIV_28_5: u32 = 0xCE000000;  // Processor clock is pll / 28.5
pub const SYSDIV_29_5: u32 = 0xCE800000;  // Processor clock is pll / 29.5
pub const SYSDIV_30_5: u32 = 0xCF000000;  // Processor clock is pll / 30.5
pub const SYSDIV_31_5: u32 = 0xCF800000;  // Processor clock is pll / 31.5
pub const SYSDIV_32_5: u32 = 0xD0000000;  // Processor clock is pll / 32.5
pub const SYSDIV_33_5: u32 = 0xD0800000;  // Processor clock is pll / 33.5
pub const SYSDIV_34_5: u32 = 0xD1000000;  // Processor clock is pll / 34.5
pub const SYSDIV_35_5: u32 = 0xD1800000;  // Processor clock is pll / 35.5
pub const SYSDIV_36_5: u32 = 0xD2000000;  // Processor clock is pll / 36.5
pub const SYSDIV_37_5: u32 = 0xD2800000;  // Processor clock is pll / 37.5
pub const SYSDIV_38_5: u32 = 0xD3000000;  // Processor clock is pll / 38.5
pub const SYSDIV_39_5: u32 = 0xD3800000;  // Processor clock is pll / 39.5
pub const SYSDIV_40_5: u32 = 0xD4000000;  // Processor clock is pll / 40.5
pub const SYSDIV_41_5: u32 = 0xD4800000;  // Processor clock is pll / 41.5
pub const SYSDIV_42_5: u32 = 0xD5000000;  // Processor clock is pll / 42.5
pub const SYSDIV_43_5: u32 = 0xD5800000;  // Processor clock is pll / 43.5
pub const SYSDIV_44_5: u32 = 0xD6000000;  // Processor clock is pll / 44.5
pub const SYSDIV_45_5: u32 = 0xD6800000;  // Processor clock is pll / 45.5
pub const SYSDIV_46_5: u32 = 0xD7000000;  // Processor clock is pll / 46.5
pub const SYSDIV_47_5: u32 = 0xD7800000;  // Processor clock is pll / 47.5
pub const SYSDIV_48_5: u32 = 0xD8000000;  // Processor clock is pll / 48.5
pub const SYSDIV_49_5: u32 = 0xD8800000;  // Processor clock is pll / 49.5
pub const SYSDIV_50_5: u32 = 0xD9000000;  // Processor clock is pll / 50.5
pub const SYSDIV_51_5: u32 = 0xD9800000;  // Processor clock is pll / 51.5
pub const SYSDIV_52_5: u32 = 0xDA000000;  // Processor clock is pll / 52.5
pub const SYSDIV_53_5: u32 = 0xDA800000;  // Processor clock is pll / 53.5
pub const SYSDIV_54_5: u32 = 0xDB000000;  // Processor clock is pll / 54.5
pub const SYSDIV_55_5: u32 = 0xDB800000;  // Processor clock is pll / 55.5
pub const SYSDIV_56_5: u32 = 0xDC000000;  // Processor clock is pll / 56.5
pub const SYSDIV_57_5: u32 = 0xDC800000;  // Processor clock is pll / 57.5
pub const SYSDIV_58_5: u32 = 0xDD000000;  // Processor clock is pll / 58.5
pub const SYSDIV_59_5: u32 = 0xDD800000;  // Processor clock is pll / 59.5
pub const SYSDIV_60_5: u32 = 0xDE000000;  // Processor clock is pll / 60.5
pub const SYSDIV_61_5: u32 = 0xDE800000;  // Processor clock is pll / 61.5
pub const SYSDIV_62_5: u32 = 0xDF000000;  // Processor clock is pll / 62.5
pub const SYSDIV_63_5: u32 = 0xDF800000;  // Processor clock is pll / 63.5
pub const CFG_VCO_480: u32 = 0xF1000000;  // VCO is 480 MHz
pub const CFG_VCO_320: u32 = 0xF0000000;  // VCO is 320 MHz
pub const USE_PLL: u32 = 0x00000000;  // System clock is the PLL clock
pub const USE_OSC: u32 = 0x00003800;  // System clock is the osc clock
pub const XTAL_1MHZ: u32 = 0x00000000;  // External crystal is 1MHz
pub const XTAL_1_84MHZ: u32 = 0x00000040;  // External crystal is 1.8432MHz
pub const XTAL_2MHZ: u32 = 0x00000080;  // External crystal is 2MHz
pub const XTAL_2_45MHZ: u32 = 0x000000C0;  // External crystal is 2.4576MHz
pub const XTAL_3_57MHZ: u32 = 0x00000100;  // External crystal is 3.579545MHz
pub const XTAL_3_68MHZ: u32 = 0x00000140;  // External crystal is 3.6864MHz
pub const XTAL_4MHZ: u32 = 0x00000180;  // External crystal is 4MHz
pub const XTAL_4_09MHZ: u32 = 0x000001C0;  // External crystal is 4.096MHz
pub const XTAL_4_91MHZ: u32 = 0x00000200;  // External crystal is 4.9152MHz
pub const XTAL_5MHZ: u32 = 0x00000240;  // External crystal is 5MHz
pub const XTAL_5_12MHZ: u32 = 0x00000280;  // External crystal is 5.12MHz
pub const XTAL_6MHZ: u32 = 0x000002C0;  // External crystal is 6MHz
pub const XTAL_6_14MHZ: u32 = 0x00000300;  // External crystal is 6.144MHz
pub const XTAL_7_37MHZ: u32 = 0x00000340;  // External crystal is 7.3728MHz
pub const XTAL_8MHZ: u32 = 0x00000380;  // External crystal is 8MHz
pub const XTAL_8_19MHZ: u32 = 0x000003C0;  // External crystal is 8.192MHz
pub const XTAL_10MHZ: u32 = 0x00000400;  // External crystal is 10 MHz
pub const XTAL_12MHZ: u32 = 0x00000440;  // External crystal is 12 MHz
pub const XTAL_12_2MHZ: u32 = 0x00000480;  // External crystal is 12.288 MHz
pub const XTAL_13_5MHZ: u32 = 0x000004C0;  // External crystal is 13.56 MHz
pub const XTAL_14_3MHZ: u32 = 0x00000500;  // External crystal is 14.31818 MHz
pub const XTAL_16MHZ: u32 = 0x00000540;  // External crystal is 16 MHz
pub const XTAL_16_3MHZ: u32 = 0x00000580;  // External crystal is 16.384 MHz
pub const XTAL_18MHZ: u32 = 0x000005C0;  // External crystal is 18.0 MHz
pub const XTAL_20MHZ: u32 = 0x00000600;  // External crystal is 20.0 MHz
pub const XTAL_24MHZ: u32 = 0x00000640;  // External crystal is 24.0 MHz
pub const XTAL_25MHZ: u32 = 0x00000680;  // External crystal is 25.0 MHz
pub const OSC_MAIN: u32 = 0x00000000;  // Osc source is main osc
pub const OSC_INT: u32 = 0x00000010;  // Osc source is int. osc
pub const OSC_INT4: u32 = 0x00000020;  // Osc source is int. osc /4
pub const OSC_INT30: u32 = 0x00000030;  // Osc source is int. 30 KHz
pub const OSC_EXT32: u32 = 0x80000038;  // Osc source is ext. 32 KHz
pub const INT_OSC_DIS: u32 = 0x00000002;  // Disable internal oscillator
pub const MAIN_OSC_DIS: u32 = 0x00000001;  // Disable main oscillator

//*****************************************************************************
//
// The following are values that can be passed to the SysCtlDeepSleepClockSet()
// API as the ui32Config parameter.
//
//*****************************************************************************
pub const DSLP_DIV_1: u32 = 0x00000000;  // Deep-sleep clock is osc /1
pub const DSLP_DIV_2: u32 = 0x00800000;  // Deep-sleep clock is osc /2
pub const DSLP_DIV_3: u32 = 0x01000000;  // Deep-sleep clock is osc /3
pub const DSLP_DIV_4: u32 = 0x01800000;  // Deep-sleep clock is osc /4
pub const DSLP_DIV_5: u32 = 0x02000000;  // Deep-sleep clock is osc /5
pub const DSLP_DIV_6: u32 = 0x02800000;  // Deep-sleep clock is osc /6
pub const DSLP_DIV_7: u32 = 0x03000000;  // Deep-sleep clock is osc /7
pub const DSLP_DIV_8: u32 = 0x03800000;  // Deep-sleep clock is osc /8
pub const DSLP_DIV_9: u32 = 0x04000000;  // Deep-sleep clock is osc /9
pub const DSLP_DIV_10: u32 = 0x04800000;  // Deep-sleep clock is osc /10
pub const DSLP_DIV_11: u32 = 0x05000000;  // Deep-sleep clock is osc /11
pub const DSLP_DIV_12: u32 = 0x05800000;  // Deep-sleep clock is osc /12
pub const DSLP_DIV_13: u32 = 0x06000000;  // Deep-sleep clock is osc /13
pub const DSLP_DIV_14: u32 = 0x06800000;  // Deep-sleep clock is osc /14
pub const DSLP_DIV_15: u32 = 0x07000000;  // Deep-sleep clock is osc /15
pub const DSLP_DIV_16: u32 = 0x07800000;  // Deep-sleep clock is osc /16
pub const DSLP_DIV_17: u32 = 0x08000000;  // Deep-sleep clock is osc /17
pub const DSLP_DIV_18: u32 = 0x08800000;  // Deep-sleep clock is osc /18
pub const DSLP_DIV_19: u32 = 0x09000000;  // Deep-sleep clock is osc /19
pub const DSLP_DIV_20: u32 = 0x09800000;  // Deep-sleep clock is osc /20
pub const DSLP_DIV_21: u32 = 0x0A000000;  // Deep-sleep clock is osc /21
pub const DSLP_DIV_22: u32 = 0x0A800000;  // Deep-sleep clock is osc /22
pub const DSLP_DIV_23: u32 = 0x0B000000;  // Deep-sleep clock is osc /23
pub const DSLP_DIV_24: u32 = 0x0B800000;  // Deep-sleep clock is osc /24
pub const DSLP_DIV_25: u32 = 0x0C000000;  // Deep-sleep clock is osc /25
pub const DSLP_DIV_26: u32 = 0x0C800000;  // Deep-sleep clock is osc /26
pub const DSLP_DIV_27: u32 = 0x0D000000;  // Deep-sleep clock is osc /27
pub const DSLP_DIV_28: u32 = 0x0D800000;  // Deep-sleep clock is osc /28
pub const DSLP_DIV_29: u32 = 0x0E000000;  // Deep-sleep clock is osc /29
pub const DSLP_DIV_30: u32 = 0x0E800000;  // Deep-sleep clock is osc /30
pub const DSLP_DIV_31: u32 = 0x0F000000;  // Deep-sleep clock is osc /31
pub const DSLP_DIV_32: u32 = 0x0F800000;  // Deep-sleep clock is osc /32
pub const DSLP_DIV_33: u32 = 0x10000000;  // Deep-sleep clock is osc /33
pub const DSLP_DIV_34: u32 = 0x10800000;  // Deep-sleep clock is osc /34
pub const DSLP_DIV_35: u32 = 0x11000000;  // Deep-sleep clock is osc /35
pub const DSLP_DIV_36: u32 = 0x11800000;  // Deep-sleep clock is osc /36
pub const DSLP_DIV_37: u32 = 0x12000000;  // Deep-sleep clock is osc /37
pub const DSLP_DIV_38: u32 = 0x12800000;  // Deep-sleep clock is osc /38
pub const DSLP_DIV_39: u32 = 0x13000000;  // Deep-sleep clock is osc /39
pub const DSLP_DIV_40: u32 = 0x13800000;  // Deep-sleep clock is osc /40
pub const DSLP_DIV_41: u32 = 0x14000000;  // Deep-sleep clock is osc /41
pub const DSLP_DIV_42: u32 = 0x14800000;  // Deep-sleep clock is osc /42
pub const DSLP_DIV_43: u32 = 0x15000000;  // Deep-sleep clock is osc /43
pub const DSLP_DIV_44: u32 = 0x15800000;  // Deep-sleep clock is osc /44
pub const DSLP_DIV_45: u32 = 0x16000000;  // Deep-sleep clock is osc /45
pub const DSLP_DIV_46: u32 = 0x16800000;  // Deep-sleep clock is osc /46
pub const DSLP_DIV_47: u32 = 0x17000000;  // Deep-sleep clock is osc /47
pub const DSLP_DIV_48: u32 = 0x17800000;  // Deep-sleep clock is osc /48
pub const DSLP_DIV_49: u32 = 0x18000000;  // Deep-sleep clock is osc /49
pub const DSLP_DIV_50: u32 = 0x18800000;  // Deep-sleep clock is osc /50
pub const DSLP_DIV_51: u32 = 0x19000000;  // Deep-sleep clock is osc /51
pub const DSLP_DIV_52: u32 = 0x19800000;  // Deep-sleep clock is osc /52
pub const DSLP_DIV_53: u32 = 0x1A000000;  // Deep-sleep clock is osc /53
pub const DSLP_DIV_54: u32 = 0x1A800000;  // Deep-sleep clock is osc /54
pub const DSLP_DIV_55: u32 = 0x1B000000;  // Deep-sleep clock is osc /55
pub const DSLP_DIV_56: u32 = 0x1B800000;  // Deep-sleep clock is osc /56
pub const DSLP_DIV_57: u32 = 0x1C000000;  // Deep-sleep clock is osc /57
pub const DSLP_DIV_58: u32 = 0x1C800000;  // Deep-sleep clock is osc /58
pub const DSLP_DIV_59: u32 = 0x1D000000;  // Deep-sleep clock is osc /59
pub const DSLP_DIV_60: u32 = 0x1D800000;  // Deep-sleep clock is osc /60
pub const DSLP_DIV_61: u32 = 0x1E000000;  // Deep-sleep clock is osc /61
pub const DSLP_DIV_62: u32 = 0x1E800000;  // Deep-sleep clock is osc /62
pub const DSLP_DIV_63: u32 = 0x1F000000;  // Deep-sleep clock is osc /63
pub const DSLP_DIV_64: u32 = 0x1F800000;  // Deep-sleep clock is osc /64
pub const DSLP_OSC_MAIN: u32 = 0x00000000;  // Osc source is main osc
pub const DSLP_OSC_INT: u32 = 0x00000010;  // Osc source is int. osc
pub const DSLP_OSC_INT30: u32 = 0x00000030;  // Osc source is int. 30 KHz
pub const DSLP_OSC_EXT32: u32 = 0x00000070;  // Osc source is ext. 32 KHz
pub const DSLP_PIOSC_PD: u32 = 0x00000002;  // Power down PIOSC in deep-sleep
pub const DSLP_MOSC_PD: u32 = 0x40000000;  // Power down MOSC in deep-sleep

//*****************************************************************************
//
// The following are values that can be passed to the SysCtlPIOSCCalibrate()
// API as the ui32Type parameter.
//
//*****************************************************************************
pub const PIOSC_CAL_AUTO: u32 = 0x00000200;  // Automatic calibration
pub const PIOSC_CAL_FACT: u32 = 0x00000100;  // Factory calibration
pub const PIOSC_CAL_USER: u32 = 0x80000100;  // User-supplied calibration

//*****************************************************************************
//
// The following are values that can be passed to the SysCtlMOSCConfigSet() API
// as the ui32Config parameter.
//
//*****************************************************************************
pub const MOSC_VALIDATE: u32 = 0x00000001;  // Enable MOSC validation
pub const MOSC_INTERRUPT: u32 = 0x00000002;  // Generate interrupt on MOSC fail
pub const MOSC_NO_XTAL: u32 = 0x00000004;  // No crystal is attached to MOSC
pub const MOSC_PWR_DIS: u32 = 0x00000008;  // Power down the MOSC.
pub const MOSC_LOWFREQ: u32 = 0x00000000;  // MOSC is less than 10MHz
pub const MOSC_HIGHFREQ: u32 = 0x00000010;  // MOSC is greater than 10MHz
pub const MOSC_SESRC: u32 = 0x00000020;  // Singled ended oscillator source.

//*****************************************************************************
//
// The following are values that can be passed to the SysCtlSleepPowerSet() and
// SysCtlDeepSleepPowerSet() APIs as the ui32Config parameter.
//
//*****************************************************************************
pub const LDO_SLEEP: u32 = 0x00000200;  // LDO in sleep mode
// (Deep Sleep Only)
pub const TEMP_LOW_POWER: u32 = 0x00000100;  // Temp sensor in low power mode
// (Deep Sleep Only)
pub const FLASH_NORMAL: u32 = 0x00000000;  // Flash in normal mode
pub const FLASH_LOW_POWER: u32 = 0x00000020;  // Flash in low power mode
pub const SRAM_NORMAL: u32 = 0x00000000;  // SRAM in normal mode
pub const SRAM_STANDBY: u32 = 0x00000001;  // SRAM in standby mode
pub const SRAM_LOW_POWER: u32 = 0x00000003;  // SRAM in low power mode

//*****************************************************************************
//
// Defines for the SysCtlResetBehaviorSet() and SysCtlResetBehaviorGet() APIs.
//
//*****************************************************************************
pub const ONRST_WDOG0_POR: u32 = 0x00000030;
pub const ONRST_WDOG0_SYS: u32 = 0x00000020;
pub const ONRST_WDOG1_POR: u32 = 0x000000C0;
pub const ONRST_WDOG1_SYS: u32 = 0x00000080;
pub const ONRST_BOR_POR: u32 = 0x0000000C;
pub const ONRST_BOR_SYS: u32 = 0x00000008;
pub const ONRST_EXT_POR: u32 = 0x00000003;
pub const ONRST_EXT_SYS: u32 = 0x00000002;

//*****************************************************************************
//
// Values used with the SysCtlVoltageEventConfig() API.
//
//*****************************************************************************
pub const VEVENT_VDDABO_NONE: u32 = 0x00000000;
pub const VEVENT_VDDABO_INT: u32 = 0x00000100;
pub const VEVENT_VDDABO_NMI: u32 = 0x00000200;
pub const VEVENT_VDDABO_RST: u32 = 0x00000300;
pub const VEVENT_VDDBO_NONE: u32 = 0x00000000;
pub const VEVENT_VDDBO_INT: u32 = 0x00000001;
pub const VEVENT_VDDBO_NMI: u32 = 0x00000002;
pub const VEVENT_VDDBO_RST: u32 = 0x00000003;

//*****************************************************************************
//
// Values used with the SysCtlVoltageEventStatus() and
// SysCtlVoltageEventClear() APIs.
//
//*****************************************************************************
pub const VESTAT_VDDBOR: u32 = 0x00000040;
pub const VESTAT_VDDABOR: u32 = 0x00000010;

//*****************************************************************************
//
// Values used with the SysCtlNMIStatus() API.
//
//*****************************************************************************
pub const NMI_MOSCFAIL: u32 = 0x00010000;
pub const NMI_TAMPER: u32 = 0x00000200;
pub const NMI_WDT1: u32 = 0x00000020;
pub const NMI_WDT0: u32 = 0x00000008;
pub const NMI_POWER: u32 = 0x00000004;
pub const NMI_EXTERNAL: u32 = 0x00000001;

//*****************************************************************************
//
// The defines for the SysCtlClockOutConfig() API.
//
//*****************************************************************************
pub const CLKOUT_EN: u32 = 0x80000000;
pub const CLKOUT_DIS: u32 = 0x00000000;
pub const CLKOUT_SYSCLK: u32 = 0x00000000;
pub const CLKOUT_PIOSC: u32 = 0x00010000;
pub const CLKOUT_MOSC: u32 = 0x00020000;

//*****************************************************************************
//
// The following defines are used with the SysCtlAltClkConfig() function.
//
//*****************************************************************************
pub const ALTCLK_PIOSC: u32 = 0x00000000;
pub const ALTCLK_RTCOSC: u32 = 0x00000003;
pub const ALTCLK_LFIOSC: u32 = 0x00000004;

pub fn ClockGet() -> u32 { unsafe {
    let func = getfun(13, 24) as *const extern "C" fn() -> u32;
    (*func)()
}}

pub fn ClockSet(config: u32) { unsafe {
    let func = getfun(13, 23) as *const extern "C" fn(u32);
    (*func)(config)
}}

pub fn DeepSleep() { unsafe {
    let func = getfun(13, 20) as *const extern "C" fn();
    (*func)()
}}

pub fn DeepSleepClockSet(config: u32) { unsafe {
    let func = getfun(13, 46) as *const extern "C" fn(u32);
    (*func)(config)
}}

pub fn Delay(count: u32) { unsafe {
    let func = getfun(13, 34) as *const extern "C" fn(u32);
    (*func)(count)
}}

pub fn FlashSizeGet() -> u32 { unsafe {
    let func = getfun(13, 2) as *const extern "C" fn() -> u32;
    (*func)()
}}

pub fn IntClear(ints: u32) { unsafe {
    let func = getfun(13, 15) as *const extern "C" fn(u32);
    (*func)(ints)
}}

pub fn IntDisable(ints: u32) { unsafe {
    let func = getfun(13, 14) as *const extern "C" fn(u32);
    (*func)(ints)
}}

pub fn IntEnable(ints: u32) { unsafe {
    let func = getfun(13, 13) as *const extern "C" fn(u32);
    (*func)(ints)
}}

/*
pub fn IntStatus(masked: bool) -> u32 { unsafe {
    let func = getfun(13, 16) as *const extern "C" fn(bool) -> u32;
    (*func)(masked)
}}
*/

pub fn MOSCConigSet(config: u32) { unsafe {
    let func = getfun(13, 44) as *const extern "C" fn(u32);
    (*func)(config)
}}

/*
pub fn PeripheralClockGating(enable: bool) { unsafe {
    let func = getfun(13, 12) as *const extern "C" fn(bool);
    (*func)(enable)
}}
*/

pub fn PeripheralDeepSleepDisable(peripheral: u32) { unsafe {
    let func = getfun(13, 11) as *const extern "C" fn(u32);
    (*func)(peripheral)
}}

pub fn PeripheralDeepSleepEnable(peripheral: u32) { unsafe {
    let func = getfun(13, 10) as *const extern "C" fn(u32);
    (*func)(peripheral)
}}

pub fn PeripheralDisable(peripheral: u32) { unsafe {
    let func = getfun(13, 7) as *const extern "C" fn(u32);
    (*func)(peripheral)
}}

pub fn PeripheralEnable(peripheral: u32) { unsafe {
    let func = getfun(13, 6) as *const extern "C" fn(u32);
    (*func)(peripheral)
}}

pub fn PeripheralPowerOff(peripheral: u32) { unsafe {
    let func = getfun(13, 37) as *const extern "C" fn(u32);
    (*func)(peripheral)
}}

pub fn PeripheralPowerOn(peripheral: u32) { unsafe {
    let func = getfun(13, 36) as *const extern "C" fn(u32);
    (*func)(peripheral)
}}

/*
pub fn PeripheralPresent(peripheral: u32) -> bool { unsafe {
    let func = getfun(13, 4) as *const extern "C" fn(u32);
    (*func)(peripheral)
}}

pub fn PeripheralReady(peripheral: u32) -> bool { unsafe {
    let func = getfun(13, 35) as *const extern "C" fn(u32);
    (*func)(peripheral)
}}
*/

pub fn PeripheralReset(peripheral: u32) { unsafe {
    let func = getfun(13, 5) as *const extern "C" fn(u32);
    (*func)(peripheral)
}}

pub fn PeripheralSleepDisable(peripheral: u32) { unsafe {
    let func = getfun(13, 9) as *const extern "C" fn(u32);
    (*func)(peripheral)
}}

pub fn PeripheralSleepEnable(peripheral: u32) { unsafe {
    let func = getfun(13, 8) as *const extern "C" fn(u32);
    (*func)(peripheral)
}}

pub fn PIOSCCalibrate(typ: u32) -> u32 { unsafe {
    let func = getfun(13, 45) as *const extern "C" fn(u32) -> u32;
    (*func)(typ)
}}

pub fn PWMClockGet() -> u32 { unsafe {
    let func = getfun(13, 26) as *const extern "C" fn() -> u32;
    (*func)()
}}

pub fn PWMClockSet(config: u32) { unsafe {
    let func = getfun(13, 25) as *const extern "C" fn(u32);
    (*func)(config)
}}

pub fn Reset() { unsafe {
    let func = getfun(13, 19) as *const extern "C" fn();
    (*func)()
}}

pub fn ResetCauseClear(causes: u32) { unsafe {
    let func = getfun(13, 22) as *const extern "C" fn(u32);
    (*func)(causes)
}}

pub fn ResetCauseGet() -> u32 { unsafe {
    let func = getfun(13, 21) as *const extern "C" fn() -> u32;
    (*func)()
}}

pub fn SRAMSizeGet() -> u32 { unsafe {
    let func = getfun(13, 1) as *const extern "C" fn() -> u32;
    (*func)()
}}

pub fn USBPLLDisable() { unsafe {
    let func = getfun(13, 32) as *const extern "C" fn();
    (*func)()
}}

pub fn USBPLLEnable() { unsafe {
    let func = getfun(13, 31) as *const extern "C" fn();
    (*func)()
}}
