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
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	interrupt svcall
	interrupt debug_monitor
	.word 0 /* reserved */
	interrupt pendsv
	interrupt systick

	interrupt port_a
	interrupt port_b
	interrupt port_c
	interrupt port_d
	interrupt port_e
	interrupt uart0
	interrupt uart1
	interrupt ssi0
	interrupt i2c0
	interrupt pwm0_fault
	interrupt pwm0_gen0
	interrupt pwm0_gen1
	interrupt pwm0_gen2
	interrupt qei0
	interrupt adc0_seq0
	interrupt adc0_seq1
	interrupt adc0_seq2
	interrupt adc0_seq3
	interrupt watchdog
	interrupt timer16_0a
	interrupt timer16_0b
	interrupt timer16_1a
	interrupt timer16_1b
	interrupt timer16_2a
	interrupt timer16_2b
	interrupt acomp0
	interrupt acomp1
	.word 0 /* reserved */
	interrupt sysctl
	interrupt flash
	interrupt port_f
	.word 0 /* reserved */
	.word 0 /* reserved */
	interrupt uart2
	interrupt ssi1
	interrupt timer16_3a
	interrupt timer16_3b
	interrupt i2c1
	interrupt qei1
	interrupt can0
	interrupt can1
	.word 0 /* reserved */
	.word 0 /* reserved */
	interrupt hibernate
	interrupt usb
	interrupt pwm0_gen3
	interrupt udma_software
	interrupt udma_error
	interrupt adc1_seq0
	interrupt adc1_seq1
	interrupt adc1_seq2
	interrupt adc1_seq3
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	interrupt ssi2
	interrupt ssi3
	interrupt uart3
	interrupt uart4
	interrupt uart5
	interrupt uart6
	interrupt uart7

	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	interrupt i2c2
	interrupt i2c3
	interrupt timer16_4a
	interrupt timer16_4b
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	interrupt timer16_5a
	interrupt timer16_5b
	interrupt timer32_0a
	interrupt timer32_0b
	interrupt timer32_1a
	interrupt timer32_1b
	interrupt timer32_2a
	interrupt timer32_2b
	interrupt timer32_3a
	interrupt timer32_3b
	interrupt timer32_4a
	interrupt timer32_4b
	interrupt timer32_5a
	interrupt timer32_5b
	interrupt sysexception
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	.word 0 /* reserved */
	interrupt pwm1_gen0
	interrupt pwm1_gen1
	interrupt pwm1_gen2
	interrupt pwm1_gen3
	interrupt pwm1_fault
