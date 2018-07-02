SEARCH_DIR(.)

GROUP(-lgcc -lc -lnosys)

SECTIONS
{
    .heap (COPY):
    {
        PROVIDE(end = .);
        KEEP(*(.heap*))
    } > RAM
} INSERT AFTER .bss;

INCLUDE "link.x"
