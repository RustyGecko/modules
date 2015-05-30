KIT = stk3700

RSFLAGS  = --target thumbv7m-none-eabi --verbose --features $(KIT)
LINKARGS = "-mthumb -mcpu=cortex-m3 -Tefm32gg.ld --specs=nosys.specs -lgcc -lc -lnosys -lm"

EX       = gpio
DIR      = examples
EXAMPLES = $(wildcard $(DIR)/*.rs)
NAMES    = $(EXAMPLES:$(DIR)/%.rs=%)

.PHONY: lib example examples clean

lib:
	cargo rustc --lib $(RSFLAGS) -- -C link-args=$(LINKARGS)

example: $(EX)-build

examples: $(NAMES:=-build)

%-build: $(DIR)/$(@:-build=.rs)
	cargo rustc --example $(@:-build=) $(RSFLAGS) -- -C link-args=$(LINKARGS)

clean:
	cargo clean
