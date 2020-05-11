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

It can refer to the CPU6 – central processing unit – of the computer, to a graphic processing
unit (GPU7) that performs calculations relating to graphical video displays, or any number of 
other types of processors. The terms processor and CPU tend to be used interchangeably when 
referring to the physical package that is installed in your computer 

The line in the lscpu results that specifies the number of cores contained in the
processor package is “Core(s) per socket.” For this socket on my primary workstation,
there are sixteen (4) cores. That means that there are 4 separate computing devices in
the processor plugged into this socket.

以下命令显示GPU信息
    
    ➜  ~ sudo lshw -C display
    [sudo] password for teng: 
      *-display                 
           description: 3D controller
           product: GM204M [GeForce GTX 970M]
           vendor: NVIDIA Corporation
           physical id: 0
           bus info: pci@0000:01:00.0
           version: a1
           width: 64 bits
           clock: 33MHz
           capabilities: pm msi pciexpress bus_master cap_list
           configuration: driver=nouveau latency=0
           resources: irq:37 memory:f0000000-f0ffffff memory:c0000000-cfffffff memory:d0000000-d1ffffff ioport:5000(size=128)
      *-display
           description: VGA compatible controller
           product: 4th Gen Core Processor Integrated Graphics Controller
           vendor: Intel Corporation
           physical id: 2
           bus info: pci@0000:00:02.0
           version: 06
           width: 64 bits
           clock: 33MHz
           capabilities: msi pm vga_controller bus_master cap_list rom
           configuration: driver=i915 latency=0
           resources: irq:38 memory:f1000000-f13fffff memory:e0000000-efffffff ioport:6000(size=64) memory:c0000-dffff

Rather than let precious compute cycles go to waste in high-performance computing
environments, Intel developed hyper-threading technology that allows a single core to
process two streams of instructions and data by switching between them. This enables
a single core to perform almost as well as two. So the term CPU is used to specify that a
single hyper-threading core is reasonably close to the functional equivalent of two CPUs.

But there are some caveats. Hyper-threading is not particularly helpful if all you are
doing is word processing and spreadsheets. Hyper-threading is intended to improve
performance in high-performance computing environments where every CPU compute
cycle is important in speeding the results.

# Peripherals

Peripherals are hardware devices that can be plugged into the computer via the various
types of interface ports. USB devices such as external hard drives and thumb drives are
typical of this type of hardware. Other types include keyboards, mice, and printers

# Linux OS

The Linux kernel also manages access to the CPUs as computing resources. It uses
a complex algorithm to determine which processes have are allocated some CPU time,
when, and for how long. If necessary, the kernel can interrupt a running program in
order to allow another program to have some CPU time.

An operating system kernel like Linux can do little on its own. It requires other
programs – utilities – that can be used to perform basic functions such as create a
directory on the hard drive and then other program utilities to access that directory,
create files in that directory, and then manage those files. These utility programs perform
functions like creating files, deleting files, copying files from one place to another, setting
display resolution, and complex processing of textual data. We will cover the use of many
of these utilities as we proceed through this book

# Typical operating system functions
> Any operating system has a set of core functions which are the primary reason for its existence. 

These are the functions that enable the operating system to manage itself, the
hardware on which it runs, and the application programs and utilities that depend upon
it to allocate system resources to them:

- Memory management
> Linux and other modern operating systems use advanced memory management
  strategies to virtualize real memory – random-access memory9 (RAM) and swap memory
  (disk) – into a single virtual memory space which can be used as if it were all physical
  RAM. Portions of this virtual memory10 can be allocated by the memory management
  functions of the kernel to programs that request memory.

The memory management components of the operating system are responsible for assigning virtual memory 
space to applications and utilities and for translation between virtual memory spaces and physical memory

Virtual memory eliminates the need for the application programmer to deal directly with memory management because it provides a single virtual memory address space for each program. It also isolates each application's memory space from that of every other, thus making the program's memory space safe from being overwritten or viewed by other programs.

- Managing multitasking

Linux itself always has many programs running in the background – called
daemons – programs that help Linux manage the hardware and other software running
on the host. These programs are usually not noticed by users unless we specifically look
for them. Some of the tools you will learn about in this book can reveal these otherwise
hidden programs.

Even with all of its own programs running in the background and users’ programs
running, a modern Linux computer uses a few compute cycles and wastes most of its
CPU cycles waiting for things to happen. Linux can download and install its own updates
while performing any or all of the preceding tasks simultaneously – without the need
for a reboot. Wait... what?! That’s right. Linux does not usually need to reboot before,
during, or after installing updates or when installing new software. After a new kernel or
glibc (General C Libraries) is installed, however, you may wish to reboot the computer to
activate it, but you can do that whenever you want and not be forced to reboot multiple
times during an update or even stop doing your work while the updates are installed.

- Managing multiple users
>The multitasking functionality of Linux extends to its ability to host multiple users – tens
 or hundreds of them – all running the same or different programs at the same time on
 one single computer.

Multiuser capabilities means a number of different things. 

- First, it can mean a single user who has logged in multiple times via a combination of the GUI desktop interface and via the command line using one or more terminal sessions. We will explore the extreme flexibility available when using terminal sessions a bit later in this course.

- Second, multiuser means just that – many different users logged in at the same time, each doing their own thing, and each isolated and protected from the activities of the others. Some users can be logged in locally and others from anywhere in the world with an Internet connection if the host computer is properly configured.

The role of the operating system is to allocate resources to each user and to ensure
that any tasks, that is, processes, they have running have sufficient resources without
impinging upon the resources allocated to other users.

- Process management
> The Linux kernel manages the execution of all tasks running on the system. The Linux operating system is multitasking from the moment it boots up. Many of those tasks are the background tasks required to manage a multitasking and – for Linux – a multiuser environment. These tools take only a small fraction of the available CPU resources available on even modest computers.

The scheduler portion of the kernel allocates CPU time to each running process
based on its priority and whether it is capable of running. A task which is blocked –
perhaps it is waiting for data to be delivered from the disk, or for input from the
keyboard – does not receive CPU time. The Linux kernel will also preempt a lower
priority task when a task with a higher priority becomes unblocked and capable of
running.

In order to manage processes, the kernel creates data abstractions that represent that
process. Part of the data required is that of memory maps that define the memory that is
allocated to the process and whether it is data or executable code. The kernel maintains
information about the execution status such as how recently the program had some CPU
time, how much time, and a number called the “nice” number. It uses that information
and the nice number to calculate the priority of the process. The kernel uses the priority
of all of the process to determine which process(es) will be allocated some CPU time.

Note that not all processes need CPU time simultaneously. In fact, for most desktop
workstations in normal circumstances, usually only two or three processes at the most
need to be on the CPU at any given time. This means that a simple quad-core processor
can easily handle this type of CPU load.

If there are more programs – processes – running than there are CPUs in the system,
the kernel is responsible for determining which process to interrupt in order to replace it
with a different one that needs some CPU time.

- Interprocess communication
>Interprocess communication (IPC) is vital to any multitasking operating system. Many programs must be synchronized with each other to ensure that their work is properly coordinated. Interprocess communication is the tool that enables this type of interprogram cooperation.

The kernel manages a number of IPC methods. Shared memory is used when two
tasks need to pass data between them. The Linux clipboard is a good example of shared
memory. Data which is cut or copied to the clipboard is stored in shared memory. When
the stored data is pasted into another application, that application looks for the data in
the clipboard’s shared memory area. Named pipes can be used to communicate data
between two programs. Data can be pushed into the pipe by one program, and the other
program can pull the data out of the other end of the pipe. A program may collect data
very quickly and push it into the pipe. Another program may take the data out of the
other end of the pipe and either display it on the screen or store it to the disk, but it can
handle the data at its own rate.

- Device management
> The kernel manages access to the physical hardware through the use of device drivers

Although we tend to think of this in terms of various types of hard drives and other
storage devices, it also manages other input/output (I/O) devices such as the keyboard,
mouse, display, printers, and so on. This includes management of pluggable devices
such as USB storage devices and external USB and eSATA hard drives.

Access to physical devices must be managed carefully, or more than one application
might attempt to control the same device at the same time. The Linux kernel manages
devices so that only one program actually has control of or access to a device at any given
moment. One example of this is a COM port.12 Only one program can communicate
through a COM port at any given time. If you are using the COM port to get your e-mail
from the Internet, for example, and try to start another program which attempts to use
the same COM port, the Linux kernel detects that the COM port is already in use. The
kernel then uses the hardware error handler to display a message on the screen that the
COM port is in use.

For managing disk I/O devices, including USB, parallel and serial port I/O, and filesystem I/O, the kernel does not actually handle physical access to the disk but rather manages the requests for disk I/O submitted by the various running programs. It passes these requests on to the filesystem, whether it be EXT[2,3,4], VFAT, HPFS, CDFS (CDROM file system), or NFS (Network Filesystem, or some other filesystem types), and manages the transfer of data between the filesystem and the requesting programs.

We will see later how all types of hardware – whether they are storage devices or
something else attached to a Linux host – are handled as if they were files. This results in
some amazing capabilities and interesting possibilities.

- Error handling and logging
>Errors happen. As a result, the kernel needs to identify these errors when they occur. The
 kernel may take some action such as retrying the failing operation, displaying an error
 message to the user, and logging the error message to a log file.

