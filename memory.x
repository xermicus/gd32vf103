MEMORY
{
    /* 87*4=348 (0x15C) */
    VECTORS     (RX): ORIGIN = 0x08000000, LENGTH = 0x015C 
    MAIN_FLASH  (RX): ORIGIN = 0x08000000, LENGTH = 128K
    SRAM        (RW): ORIGIN = 0x20000000, LENGTH = 20K
}

REGION_ALIAS("REGION_TEXT", MAIN_FLASH);
REGION_ALIAS("REGION_RODATA", MAIN_FLASH);
REGION_ALIAS("REGION_DATA", SRAM);
REGION_ALIAS("REGION_BSS", SRAM);
REGION_ALIAS("REGION_HEAP", SRAM);
REGION_ALIAS("REGION_STACK", SRAM);

ENTRY(_gd32vf103_vectors)
EXTERN(_gd32vf103_vectors)
EXTERN(_gd32vf103_trap_entry)
EXTERN(_gd32vf103_irq_entry)

SECTIONS
{
  .vectors :
  {
    KEEP(*(.vectors));
  } > VECTORS
}