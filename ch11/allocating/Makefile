obj-m := allocating.o
ifeq ($(RELEASE),1)
	BUILD = release
else
	BUILD = debug
endif

ifeq ($(TARGET),armv7l-linux-kernel-module)
allocating-objs := target/$(TARGET)/$(BUILD)/liballocating.a
else
allocating-objs := target/x86_64-linux-kernel-module/$(BUILD)/liballocating.a
endif
EXTRA_LDFLAGS += --entry=init_module

ifeq ($(TARGET),armv7l-linux-kernel-module)
CMD_LINE = ARCH=arm CROSS_COMPILE=$(CROSS)
else
KDIR = /lib/modules/$(shell uname -r)/build
CMD_LINE =
endif

all:
	$(MAKE) $(CMD_LINE) -C $(KDIR) M=$(CURDIR)

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) clean
