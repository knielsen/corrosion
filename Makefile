MAKEFLAGS += -rR

NAME = tiva

HOSTCC = gcc
RUSTC = rustc
CROSS_COMPILE = arm-none-eabi-
GCC = $(CROSS_COMPILE)gcc
AR = $(CROSS_COMPILE)ar
OBJCOPY = $(CROSS_COMPILE)objcopy
STAT = stat
INSTALL = install
RM = rm -f

#CHIP = lpc1343
CHIP = tm4c123gh6pm

LDFLAGS = -nostdlib -static -Wl,-O1,--gc-sections,--as-needed -T 'src/$(CHIP)/memory.ld'
RUSTFLAGS = --opt-level 2 -g
TARGETFLAGS  = --cfg=chip_$(CHIP) --target=$(LLVM_TARGET_TRIPLE) -C target-cpu=$(CPU)
TARGETFLAGS += -C relocation-model=static -C no-stack-check -Z no-landing-pads
TARGETFLAGS += -L lib/$(LLVM_TARGET_TRIPLE)/$(CPU) -C linker=$(GCC) -C ar=$(AR)

.PRECIOUS: %.o %.elf
.PHONY: upload clean distclean

all: $(NAME).bin

include src/$(CHIP)/target.mk
include src/$(CPU).mk
-include $(NAME).d

lib/$(LLVM_TARGET_TRIPLE)/$(CPU)/libcompiler-rt.a:
	$(MAKE) -C src/compiler-rt \
		CC='$(GCC)' \
		AR='$(AR)' \
		RM='$(RM)' \
		RANLIB='$(AR) s' \
		CFLAGS='$(CFLAGS) -D_YUGA_LITTLE_ENDIAN=1 -D_YUGA_BIG_ENDIAN=0' \
		TargetTriple=$(LLVM_TARGET_TRIPLE) \
		triple-builtins
	$(INSTALL) -Dm644 src/compiler-rt/triple/builtins/libcompiler_rt.a lib/$(LLVM_TARGET_TRIPLE)/$(CPU)/libcompiler-rt.a
	$(RM) -r src/compiler-rt/triple

lib/$(LLVM_TARGET_TRIPLE)/$(CPU)/libcore.rlib: src/libcore/lib.rs lib/$(LLVM_TARGET_TRIPLE)/$(CPU)/libcompiler-rt.a
	$(RUSTC) $(RUSTFLAGS) $(TARGETFLAGS) --out-dir lib/$(LLVM_TARGET_TRIPLE)/$(CPU) $<

lib/boot-$(CHIP).o: src/$(CHIP)/boot.S
	$(GCC) $(CFLAGS) -c $< -o $@

%.d %.ll %.s %.o: %.rs lib/$(LLVM_TARGET_TRIPLE)/$(CPU)/libcore.rlib
	$(RUSTC) --dep-info $*.d --emit=ir,asm,obj $(RUSTFLAGS) $(TARGETFLAGS) $<

%.elf: lib/boot-$(CHIP).o %.o
	$(GCC) $(LDFLAGS) -o $@ $^ lib/$(LLVM_TARGET_TRIPLE)/$(CPU)/libcore.rlib lib/$(LLVM_TARGET_TRIPLE)/$(CPU)/libcompiler-rt.a

%.bin: %.elf
	$(OBJCOPY) -O binary $< $@
	@$(STAT) -c 'Size: %s bytes' $@

clean:
	rm -f *.d *.ll *.s *.o *.elf *.bin

distclean: clean
	rm -rf lib fix-checksum
