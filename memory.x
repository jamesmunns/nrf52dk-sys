/* Linker script for the nRF52832. */
MEMORY
{
  /* Flash and RAM is offset to leave room for the S132 soft device. */
  FLASH (rx) : ORIGIN = 0x1f000, LENGTH = 0x61000
  RAM (rwx) :  ORIGIN = 0x20001fc0, LENGTH = 0xe040
}
