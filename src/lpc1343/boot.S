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

	interrupt pio0_0
	interrupt pio0_1
	interrupt pio0_2
	interrupt pio0_3
	interrupt pio0_4
	interrupt pio0_5
	interrupt pio0_6
	interrupt pio0_7
	interrupt pio0_8
	interrupt pio0_9
	interrupt pio0_10
	interrupt pio0_11

	interrupt pio1_0
	interrupt pio1_1
	interrupt pio1_2
	interrupt pio1_3
	interrupt pio1_4
	interrupt pio1_5
	interrupt pio1_6
	interrupt pio1_7
	interrupt pio1_8
	interrupt pio1_9
	interrupt pio1_10
	interrupt pio1_11

	interrupt pio2_0
	interrupt pio2_1
	interrupt pio2_2
	interrupt pio2_3
	interrupt pio2_4
	interrupt pio2_5
	interrupt pio2_6
	interrupt pio2_7
	interrupt pio2_8
	interrupt pio2_9
	interrupt pio2_10
	interrupt pio2_11

	interrupt pio3_0
	interrupt pio3_1
	interrupt pio3_2
	interrupt pio3_3

	interrupt i2c
	interrupt timer16b0
	interrupt timer16b1
	interrupt timer32b0
	interrupt timer32b1
	interrupt ssp
	interrupt uart
	interrupt usb_irq
	interrupt usb_fiq
	interrupt adc
	interrupt watchdog
	interrupt brownout
	interrupt flash
	interrupt port3
	interrupt port2
	interrupt port1
	interrupt port0
