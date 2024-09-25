# ws281x-smi-rs

Most of this code was based on the existing work in [rpi_pixleds.c](https://github.com/jbentham/rpi/blob/master/rpi_pixleds.c) but translated and cleaned up to (hopefully) have less magic numbers and be easier to understand. The less-magic-numbers step is still a large work in progress.

The end goal is to drive a large 8x8x8 rgb led cube from an rpi zero 2 w while having smooth 60 fps animations with this code.

todo: document this better...

## Structure

/rpi-mailbox: fork of [rpi-mailbox](https://github.com/Idein/rpi-mailbox) with fixes to ensure it works on both arm and arm64 kernels

/src/dma: basic DMA peripheral manager, assumes you are only using one control block for now

/src/smi: SMI peripheral management, not very generic at this stage and instead assumes you are doing led-ish things with it

/src/gpio.rs: basic GPIO mode management

/src/vc_mem.rs: allocation/deallocation of uncached memory to be used for DMA src/dest things