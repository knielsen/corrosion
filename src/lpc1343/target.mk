CPU = cortex-m3

MCOPY = mcopy
DEST = a:

fix-checksum: src/fix-checksum.c
	$(HOSTCC) -O2 -Wall -Wextra -pipe $< -o $@

upload: $(NAME).bin fix-checksum
	./fix-checksum $<
	$(MCOPY) $< $(DEST)firmware.bin
