# LPC elf boot checksum tool

From the user manual of all LPC microcontrollers (except LPC55):

"The reserved exception vector location 7 (offset 0x001C in the vector table)  
should contain the 2’s complement of the check-sum of table entries 0 through 6. This  
causes the checksum of the first 8 table entries to be 0. The boot loader code checksums  
the first 8 locations in sector 0 of the flash. If the result is 0, then execution control is  
transferred to the user code."

This tool generates the checksum and patches an elf file so it becomes bootable.

For LPC55, use `lpc55_sign` from https://github.com/oxidecomputer/lpc55_support instead.
