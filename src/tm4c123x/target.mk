CPU = cortex-m4

LM4FLASH = lm4flash

upload: $(NAME).bin
	$(LM4FLASH) $<
