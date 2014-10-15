pub const PA0_U0RX: u32 = 0x00000001;
pub const PA0_CAN1RX: u32 = 0x00000008;

pub const PA1_U0TX: u32 = 0x00000401;
pub const PA1_CAN1TX: u32 = 0x00000408;

pub const PA2_SSI0CLK: u32 = 0x00000802;

pub const PA3_SSI0FSS: u32 = 0x00000C02;

pub const PA4_SSI0RX: u32 = 0x00001002;

pub const PA5_SSI0TX: u32 = 0x00001402;

pub const PA6_I2C1SCL: u32 = 0x00001803;
pub const PA6_M1PWM2: u32 = 0x00001805;

pub const PA7_I2C1SDA: u32 = 0x00001C03;
pub const PA7_M1PWM3: u32 = 0x00001C05;

pub const PB0_U1RX: u32 = 0x00010001;
pub const PB0_T2CCP0: u32 = 0x00010007;

pub const PB1_U1TX: u32 = 0x00010401;
pub const PB1_T2CCP1: u32 = 0x00010407;

pub const PB2_I2C0SCL: u32 = 0x00010803;
pub const PB2_T3CCP0: u32 = 0x00010807;

pub const PB3_I2C0SDA: u32 = 0x00010C03;
pub const PB3_T3CCP1: u32 = 0x00010C07;

pub const PB4_SSI2CLK: u32 = 0x00011002;
pub const PB4_M0PWM2: u32 = 0x00011004;
pub const PB4_T1CCP0: u32 = 0x00011007;
pub const PB4_CAN0RX: u32 = 0x00011008;

pub const PB5_SSI2FSS: u32 = 0x00011402;
pub const PB5_M0PWM3: u32 = 0x00011404;
pub const PB5_T1CCP1: u32 = 0x00011407;
pub const PB5_CAN0TX: u32 = 0x00011408;

pub const PB6_SSI2RX: u32 = 0x00011802;
pub const PB6_M0PWM0: u32 = 0x00011804;
pub const PB6_T0CCP0: u32 = 0x00011807;

pub const PB7_SSI2TX: u32 = 0x00011C02;
pub const PB7_M0PWM1: u32 = 0x00011C04;
pub const PB7_T0CCP1: u32 = 0x00011C07;

pub const PC0_TCK: u32 = 0x00020001;
pub const PC0_SWCLK: u32 = 0x00020001;
pub const PC0_T4CCP0: u32 = 0x00020007;

pub const PC1_TMS: u32 = 0x00020401;
pub const PC1_SWDIO: u32 = 0x00020401;
pub const PC1_T4CCP1: u32 = 0x00020407;

pub const PC2_TDI: u32 = 0x00020801;
pub const PC2_T5CCP0: u32 = 0x00020807;

pub const PC3_SWO: u32 = 0x00020C01;
pub const PC3_TDO: u32 = 0x00020C01;
pub const PC3_T5CCP1: u32 = 0x00020C07;

pub const PC4_U4RX: u32 = 0x00021001;
pub const PC4_U1RX: u32 = 0x00021002;
pub const PC4_M0PWM6: u32 = 0x00021004;
pub const PC4_IDX1: u32 = 0x00021006;
pub const PC4_WT0CCP0: u32 = 0x00021007;
pub const PC4_U1RTS: u32 = 0x00021008;

pub const PC5_U4TX: u32 = 0x00021401;
pub const PC5_U1TX: u32 = 0x00021402;
pub const PC5_M0PWM7: u32 = 0x00021404;
pub const PC5_PHA1: u32 = 0x00021406;
pub const PC5_WT0CCP1: u32 = 0x00021407;
pub const PC5_U1CTS: u32 = 0x00021408;

pub const PC6_U3RX: u32 = 0x00021801;
pub const PC6_PHB1: u32 = 0x00021806;
pub const PC6_WT1CCP0: u32 = 0x00021807;
pub const PC6_USB0EPEN: u32 = 0x00021808;

pub const PC7_U3TX: u32 = 0x00021C01;
pub const PC7_WT1CCP1: u32 = 0x00021C07;
pub const PC7_USB0PFLT: u32 = 0x00021C08;

pub const PD0_SSI3CLK: u32 = 0x00030001;
pub const PD0_SSI1CLK: u32 = 0x00030002;
pub const PD0_I2C3SCL: u32 = 0x00030003;
pub const PD0_M0PWM6: u32 = 0x00030004;
pub const PD0_M1PWM0: u32 = 0x00030005;
pub const PD0_WT2CCP0: u32 = 0x00030007;

pub const PD1_SSI3FSS: u32 = 0x00030401;
pub const PD1_SSI1FSS: u32 = 0x00030402;
pub const PD1_I2C3SDA: u32 = 0x00030403;
pub const PD1_M0PWM7: u32 = 0x00030404;
pub const PD1_M1PWM1: u32 = 0x00030405;
pub const PD1_WT2CCP1: u32 = 0x00030407;

pub const PD2_SSI3RX: u32 = 0x00030801;
pub const PD2_SSI1RX: u32 = 0x00030802;
pub const PD2_M0FAULT0: u32 = 0x00030804;
pub const PD2_WT3CCP0: u32 = 0x00030807;
pub const PD2_USB0EPEN: u32 = 0x00030808;

pub const PD3_SSI3TX: u32 = 0x00030C01;
pub const PD3_SSI1TX: u32 = 0x00030C02;
pub const PD3_IDX0: u32 = 0x00030C06;
pub const PD3_WT3CCP1: u32 = 0x00030C07;
pub const PD3_USB0PFLT: u32 = 0x00030C08;

pub const PD4_U6RX: u32 = 0x00031001;
pub const PD4_WT4CCP0: u32 = 0x00031007;

pub const PD5_U6TX: u32 = 0x00031401;
pub const PD5_WT4CCP1: u32 = 0x00031407;

pub const PD6_U2RX: u32 = 0x00031801;
pub const PD6_M0FAULT0: u32 = 0x00031804;
pub const PD6_PHA0: u32 = 0x00031806;
pub const PD6_WT5CCP0: u32 = 0x00031807;

pub const PD7_U2TX: u32 = 0x00031C01;
pub const PD7_PHB0: u32 = 0x00031C06;
pub const PD7_WT5CCP1: u32 = 0x00031C07;
pub const PD7_NMI: u32 = 0x00031C08;

pub const PE0_U7RX: u32 = 0x00040001;

pub const PE1_U7TX: u32 = 0x00040401;

pub const PE4_U5RX: u32 = 0x00041001;
pub const PE4_I2C2SCL: u32 = 0x00041003;
pub const PE4_M0PWM4: u32 = 0x00041004;
pub const PE4_M1PWM2: u32 = 0x00041005;
pub const PE4_CAN0RX: u32 = 0x00041008;

pub const PE5_U5TX: u32 = 0x00041401;
pub const PE5_I2C2SDA: u32 = 0x00041403;
pub const PE5_M0PWM5: u32 = 0x00041404;
pub const PE5_M1PWM3: u32 = 0x00041405;
pub const PE5_CAN0TX: u32 = 0x00041408;

pub const PF0_U1RTS: u32 = 0x00050001;
pub const PF0_SSI1RX: u32 = 0x00050002;
pub const PF0_CAN0RX: u32 = 0x00050003;
pub const PF0_M1PWM4: u32 = 0x00050005;
pub const PF0_PHA0: u32 = 0x00050006;
pub const PF0_T0CCP0: u32 = 0x00050007;
pub const PF0_NMI: u32 = 0x00050008;
pub const PF0_C0O: u32 = 0x00050009;

pub const PF1_U1CTS: u32 = 0x00050401;
pub const PF1_SSI1TX: u32 = 0x00050402;
pub const PF1_M1PWM5: u32 = 0x00050405;
pub const PF1_PHB0: u32 = 0x00050406;
pub const PF1_T0CCP1: u32 = 0x00050407;
pub const PF1_C1O: u32 = 0x00050409;
pub const PF1_TRD1: u32 = 0x0005040E;

pub const PF2_SSI1CLK: u32 = 0x00050802;
pub const PF2_M0FAULT0: u32 = 0x00050804;
pub const PF2_M1PWM6: u32 = 0x00050805;
pub const PF2_T1CCP0: u32 = 0x00050807;
pub const PF2_TRD0: u32 = 0x0005080E;

pub const PF3_SSI1FSS: u32 = 0x00050C02;
pub const PF3_CAN0TX: u32 = 0x00050C03;
pub const PF3_M1PWM7: u32 = 0x00050C05;
pub const PF3_T1CCP1: u32 = 0x00050C07;
pub const PF3_TRCLK: u32 = 0x00050C0E;

pub const PF4_M1FAULT0: u32 = 0x00051005;
pub const PF4_IDX0: u32 = 0x00051006;
pub const PF4_T2CCP0: u32 = 0x00051007;
pub const PF4_USB0EPEN: u32 = 0x00051008;
