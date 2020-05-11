# OS

# Objectives
   - Describe the functions of the main hardware components of a computer
   - List and describe the primary functions of an operating system
   - Briefly outline the reasons that prompted Linus Torvalds to create Linux
   - Describe how the Linux core utilities support the kernel and together create an operating system
   
The operating system is the first level of software which allows your computer to
perform useful work. Understanding the role of the operating system is key to making
informed decisions about your computer.
   
Of course, most people do not realize that there even is a choice when it comes
to operating systems. Fortunately, Linux does give us a choice. Some vendors such as
EmperorLinux, System76, and others are now selling systems that already have Linux
installed. Others, like Dell, sometimes try out the idea of Linux by selling a single model
with few options.
   
We can always just purchase a new computer, install Linux on it, and wipe out
whatever other operating system might have previously been there. My preference is
to purchase the parts from a local computer store or the Internet and build my own
computers to my personal specifications. Most people don’t know that they have either
of these options and, if they did, would not want to try anyway.

# What is an Operating system?
> it is helpful to understand a little about the structure of the hardware which comprises a computer system. 

# Hardware

There are many different kinds of computers from single-board computers (SBC) like
the Arduino and the Raspberry Pi to desktop computers, servers, mainframes, and
supercomputers. Many of these use Intel or AMD processors, but others do not. For
the purposes of this series of books, I will work with Intel X86_64 hardware. Generally,
if I say Intel, you can also assume I mean the X86_64 processor series and supporting
hardware, and that AMD X86_64 hardware should produce the same results, and the
same hardware information will apply.

## Motherboard

Most Intel-based computers have a motherboard that contains many components of the
computer such as bus and I/O controllers. It also has connectors to install RAM memory
and a CPU, which are the primary components that need to be added to a motherboard
to make it functional. Single-board computers are self-contained on a single board
and do not require any additional hardware because components such as RAM, video,
network, USB, and other interfaces are all an integral part of the board.

Some motherboards contain a graphics processing unit (GPU) to connect the video
output to a monitor. If they do not, a video card can be added to the main computer I/O
bus, usually PCI2, or PCI Express (PCIe).3 Other I/O devices like a keyboard, mouse, and
external hard drives and USB memory sticks can be connected via the USB bus. Most
modern motherboards have one or two Gigabit Ethernet network interface cards (NIC)
and four or six SATA4 connectors for hard drives.

Random-access memory (RAM) is used to store data and programs while they
are being actively used by the computer. Programs and data cannot be used by the
computer unless they are stored in RAM from where they can be quickly moved into the
CPU cache. RAM and cache memory are both volatile memory; that is, the data stored
in them is lost if the computer is turned off. The computer can also erase or alter the
contents of RAM, and this is one of the things that gives computers their great flexibility
and power.

Hard drives are magnetic media used for long-term storage of data and programs.
Magnetic media is nonvolatile; the data stored on a disk remains even when power is
removed from the computer. DVDs and CD-ROM store data permanently and can be
read by the computer but not overwritten. The exception to this is that some DVD and
CD-ROM disks are re-writable. ROM means read-only memory because it can be read by
the computer but not erased or altered. Hard drives and DVD drives are connected to the
motherboard through SATA adapters.

Solid-state drives (SSDs) are the solid state equivalent of hard drives. They have the
same characteristics in terms of the long-term storage of data because it is persistent
through reboots and when the computer is powered off. Also like hard drives with
rotating magnetic disks, SSDs allow data to be erased, moved, and managed when
needed.

Printers are used to transfer data from the computer to paper. Sound cards convert
data to sound as well as the reverse. USB storage devices can be used to store data for
backup or transfer to other computers. The network interface cards (NICs) are used to
connect the computer to a network, hardwired or wireless, so that it can communicate
easily with other computers attached to the network.

# The processor
Five terms are important when we talk about processors: processor, CPU, socket, core, and thread. 

    The Linux command lscpu
    ➜  ~ lscpu 
    Architecture:        x86_64
    CPU op-mode(s):      32-bit, 64-bit
    Byte Order:          Little Endian
    CPU(s):              8
    On-line CPU(s) list: 0-7
    Thread(s) per core:  2
    Core(s) per socket:  4
    Socket(s):           1
    NUMA node(s):        1
    Vendor ID:           GenuineIntel
    CPU family:          6
    Model:               60
    Model name:          Intel(R) Core(TM) i7-4720HQ CPU @ 2.60GHz
    Stepping:            3
    CPU MHz:             2442.483
    CPU max MHz:         3600.0000
    CPU min MHz:         800.0000
    BogoMIPS:            5187.99
    Virtualization:      VT-x
    L1d cache:           32K
    L1i cache:           32K
    L2 cache:            256K
    L3 cache:            6144K
    NUMA node0 CPU(s):   0-7
    Flags:               fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm constant_tsc arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid aperfmperf pni pclmulqdq dtes64 monitor ds_cpl vmx est tm2 ssse3 sdbg fma cx16 xtpr pdcm pcid sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand lahf_lm abm cpuid_fault epb invpcid_single pti ssbd ibrs ibpb stibp tpr_shadow vnmi flexpriority ept vpid ept_ad fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid xsaveopt dtherm ida arat pln pts md_clear flush_l1d


    
    