.global __halt
.global __aeabi_unwind_cpp_pr0
.weak __aeabi_unwind_cpp_pr1
.weak __aeabi_unwind_cpp_pr2
.extern main

.section .text.reset, "ax"
.weak reset_interrupt
reset_interrupt:
	/* FIXME: check flash speed */
	/* copy data from r0 to r1, until r2 */
	ldr	r0, =__etext
	ldr	r1, =__data
	ldr	r2, =__edata
1:
	cmp	r1, r2
	bge	2f
	ldrb	r4, [r0]
	add	r0, #1
	strb	r4, [r1]
	add	r1, #1
	b	1b
.ltorg

	/* clear bss with r0 from r1 to r2 */
2:
	mov	r0, #0
	ldr	r1, =__bss
	ldr	r2, =__ebss
3:
	cmp	r1, r2
	bge	4f
	strb	r0, [r1]
	add	r1, #1
	b	3b
.ltorg

4:
	bl main

.section .text.halt, "ax"
__halt:
__aeabi_unwind_cpp_pr0 = __halt
__aeabi_unwind_cpp_pr1 = __halt
__aeabi_unwind_cpp_pr2 = __halt
	cpsid i
0:	b 0b

.macro interrupt name
	.weak \name\()_interrupt
	\name\()_interrupt = __halt
	.word \name\()_interrupt+1
.endm

.section .vectors, "ax"
__vectors:
	/* The first vector in Cortex is the stack pointer */
	.word __stack_entry
	.word reset_interrupt+1
	interrupt nmi
	interrupt hard_fault
	interrupt mpu_fault
	interrupt bus_fault
	interrupt usage_fault
	.word 0 /* reserved */
	.word 0 /* checksum */
	.word 0 /* reserved */
	.word 0 /* reserved */
	interrupt svcall
	interrupt debug_monitor
	.word 0 /* reserved */
	interrupt pendsv
	interrupt systick
	interrupt gpio_a
	interrupt gpio_b
	interrupt gpio_c
	interrupt gpio_d
	interrupt gpio_e
	interrupt uart0
	interrupt uart1
	interrupt ssi0
	interrupt i2c0
	interrupt pwm_fault
	interrupt pwm_0
	interrupt pwm_1
	interrupt pwm_2
	interrupt quadra0
	interrupt adc_0
	interrupt adc_1
	interrupt adc_2
	interrupt adc_3
	interrupt watchdog
	interrupt timer0_a
	interrupt timer0_b
	interrupt timer1_a
	interrupt timer1_b
	interrupt timer2_a
	interrupt timer2_b
	interrupt acomp0
	interrupt acomp1
	interrupt acomp2
	interrupt sysctl
	interrupt flash
	interrupt gpio_f
	interrupt gpio_g
	interrupt gpio_h
	interrupt uart2
	interrupt ssi1
	interrupt timer3_a
	interrupt timer3_b
	interrupt i2c1
	interrupt quadra1
	interrupt can0
	interrupt can1
	interrupt can2
	interrupt ether
	interrupt hibernate
	interrupt usb0
	interrupt pwm_3
	interrupt udma_transfer
	interrupt udma_error
	interrupt adc1_0
	interrupt adc1_1
	interrupt adc1_2
	interrupt adc1_3
	interrupt i2s_0
	interrupt extbus_0
	interrupt gpio_j
	interrupt gpio_k
	interrupt gpio_l
	interrupt ssi2
	interrupt ssi3
	interrupt uart3
	interrupt uart4
	interrupt uart5
	interrupt uart6
	interrupt uart7
	.word 0 /* reserved */
	.word 0 /* checksum */
	.word 0 /* reserved */
	.word 0 /* reserved */
	interrupt i2c2
	interrupt i2c3
	interrupt timer4_a
	interrupt timer4_b
	.word 0 /* reserved */
	.word 0 /* checksum */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* checksum */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* checksum */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* checksum */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* checksum */
	.word 0 /* reserved */
	.word 0 /* reserved */
	interrupt timer5_a
	interrupt timer5_b
	interrupt wtimer0_a
	interrupt wtimer0_b
	interrupt wtimer1_a
	interrupt wtimer1_b
	interrupt wtimer2_a
	interrupt wtimer2_b
	interrupt wtimer3_a
	interrupt wtimer3_b
	interrupt wtimer4_a
	interrupt wtimer4_b
	interrupt wtimer5_a
	interrupt wtimer5_b
	interrupt fpu
	interrupt peci
	interrupt lpc
	interrupt i2c4
	interrupt i2c5
	interrupt gpio_m
	interrupt gpio_n
	interrupt quadra2
	interrupt fan0
	.word 0 /* reserved */
	interrupt gpio_p
	interrupt gpio_p1
	interrupt gpio_p2
	interrupt gpio_p3
	interrupt gpio_p4
	interrupt gpio_p5
	interrupt gpio_p6
	interrupt gpio_p7
	interrupt gpio_q
	interrupt gpio_q1
	interrupt gpio_q2
	interrupt gpio_q3
	interrupt gpio_q4
	interrupt gpio_q5
	interrupt gpio_q6
	interrupt gpio_q7
	interrupt gpio_r
	interrupt gpio_s
	interrupt pwm1_0
	interrupt pwm1_1
	interrupt pwm1_2
	interrupt pwm1_3
	interrupt pwm1_fault
