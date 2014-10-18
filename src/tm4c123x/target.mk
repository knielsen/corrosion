CPU = cortex-m4

SPORT = /dev/ttyACM0

STTY = stty
CAT = cat
LM4FLASH = lm4flash

stty: $(SPORT)
	$(STTY) -F $(SPORT) raw cs8 -parenb -cstopb 115200

cat: $(SPORT)
	$(CAT) $(SPORT)

upload: $(NAME).bin
	$(LM4FLASH) $<
