CPU = cortex-m3

MCOPY = mcopy
DEST = a:

upload: $(NAME).bin
	$(MCOPY) $< $(DEST)firmware.bin
