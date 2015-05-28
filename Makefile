EXAMPLE  = usart

LINKARGS = "-mthumb -mcpu=cortex-m3 -Tefm32gg.ld --specs=nosys.specs -lgcc -lc -lnosys -lm"
RSFLAGS  = --target thumbv7m-none-eabi --verbose --features stk3700

.PHONY: all example clean example-build

all: example-build

example-build:
	cargo rustc --example $(EXAMPLE) $(RSFLAGS) -- -C link-args=$(LINKARGS)

clean:
	cargo clean
