SECTIONS {
    . = 0x000000;
    .text :
    {
        LONG(0x554e454D);
        LONG(0x31305445);
        LONG(1);
        LONG(_start);
        LONG(0x5000);
        LONG(0x2000);
        LONG(0x2000);
        LONG(0);
        LONG(0);
        *(.text)
    }
    .data : {
        *(.data)
    }
    .bss : {
        *(.bss)
    }
    .stack : {
        *(.stack)
    }
    .rodata : {
        *(.rodata)
    }
}
