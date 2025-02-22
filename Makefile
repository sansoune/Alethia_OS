NASM=nasm
MTOOL=mcopy
MKFSFAT=mkfs.fat
QEMU=qemu-system-x86_64

BOOTLOADER_BIN=./target/x86_64-unknown-uefi/debug/bootloader.efi
KERNEL_BIN=./target/x86_64-unknown-none/debug/kernel
BOOTDISK_IMG=./build/alethia_os.img
STARTUP_FILE=./constants/startup.nsh
FONT_FILE=./constants/zap-light16.psf

all: $(BOOTDISK_IMG)

	
$(BOOTDISK_IMG):
    # Create an empty disk image
	dd if=/dev/zero of=$(BOOTDISK_IMG) bs=512 count=93750
    # Format the disk image with FAT12 filesystem
	$(MKFSFAT) -F 32 -n "Alethia_OS" $(BOOTDISK_IMG)
	mmd -i $(BOOTDISK_IMG) ::/EFI
	mmd -i $(BOOTDISK_IMG) ::/EFI/BOOT
    # Copy kernel binary to the disk image dd if=./build/test_test.bin of=$(BOOTDISK_IMG) 
	$(MTOOL) -i $(BOOTDISK_IMG) $(BOOTLOADER_BIN) ::/EFI/BOOT/BOOTX64.efi
	$(MTOOL) -i $(BOOTDISK_IMG) $(KERNEL_BIN) ::kernel.elf
	$(MTOOL) -i $(BOOTDISK_IMG) $(STARTUP_FILE) ::
	$(MTOOL) -i $(BOOTDISK_IMG) $(FONT_FILE) ::font.psf

run: $(BOOTDISK_IMG)
    # Run QEMU with the bootdisk image
	$(QEMU) -bios /usr/share/ovmf/OVMF.fd -net none -drive file=$(BOOTDISK_IMG) -serial stdio

run_gdb: $(BOOTDISK_IMG)
    # Run QEMU with the bootdisk image
	$(QEMU) -bios /usr/share/ovmf/OVMF.fd -net none -drive file=$(BOOTDISK_IMG) -S -s

clean:
    # Remove generated files
	rm -f $(BOOTLOADER_OBJ) $(BOOTDISK_IMG)
.PHONY: all run clean



# qemu-system-x86_64 -bios /usr/share/ovmf/OVMF.fd -net none