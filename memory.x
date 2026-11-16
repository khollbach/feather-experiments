MEMORY
{
  /* SoftDevice uses flash below this, and the UF2 bootloader uses flash above this.
   * See `dfu_types.h` in https://github.com/adafruit/Adafruit_nRF52_Bootloader */
  /* TODO: if we end up switching from SD v6 to v7 (or getting rid of it entirely),
   * then the start address needs to change (eg, to $27000) */
  FLASH : ORIGIN = 0x00026000, LENGTH = 0xEA000 - 0x26000 /* The entire range is 1M */
  RAM   : ORIGIN = 0x20000000, LENGTH = 256K
}
