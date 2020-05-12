# The Linux Philosophy for SysAdmins

# Objectives
- The historical background of the Linux Philosophy for SysAdmins
- A basic introduction to the tenets of the Linux Philosophy for SysAdmins
- How the Linux Philosophy for SysAdmins can help you learn to be a better SysAdmin

# The structure of the philosophy

There are three layers to the Linux Philosophy for System Administrators in a way that
is similar to Maslow’s hierarchy of needs.3 These layers are also symbolic of our growth
through progressively higher levels of enlightenment.

The bottom layer is the foundation – the basic commands and knowledge that we
as SysAdmins need to know in order to perform the lowest level of our jobs. The middle
layer consists of those practical tenets that build on the foundation and inform the daily
tasks of the SysAdmin. The top layer contains the tenets that fulfill our higher needs as
SysAdmins and which encourage and enable us to share our knowledge.

In the first and most basic layer of the philosophy is the foundation. It is about “The
Linux Truth,” data streams, Standard Input/Output (STDIO), transforming data streams,
small command-line programs, and the meaning of “everything is a file,” for example.

The middle layer contains the functional aspects of the philosophy. Embracing
the command line, we expand our command-line programs to create tested and
maintainable shell programs that we save and can use repeatedly and even share. We
become the “lazy admin” and begin to automate everything. We use the Linux filesystem
hierarchy appropriately and store data in open formats. These are the functional
portions of the philosophy

# Data streams are a universal interface
> Everything in Linux revolves around streams of data - particularly text streams.

The use of Standard Input/Output (STDIO) for program input and output is a key
foundation of the Linux way of doing things and manipulating data streams. STDIO was
first developed for Unix and has found its way into most other operating systems since
then, including DOS, Windows, and Linux.

# Transforming data streams
> This tenet explores the use of pipes to connect streams of data from one utility program to another using STDIO

Data streams can be manipulated by inserting transformers into the stream
using pipes. Each transformer program is used by the SysAdmin to perform some
transformational operation on the data in the stream, thus changing its contents in some
manner. Redirection can then be used at the end of the pipeline to direct the data stream
to a file. As has already been mentioned, that file could be an actual data file on the hard
drive, or a device file such as a drive partition, a printer, a terminal, a pseudo-terminal,
or any other device connected to a computer.

I use the term “transform” in conjunction with these programs because the primary
task of each is to transform the incoming data from STDIO in a specific way as intended
by the SysAdmin and to send the transformed data to STDOUT for possible use by
another transformer program or redirection to a file.

The standard term for these programs, “filters,” implies something with which I don’t
agree. By definition, a filter is a device or a tool that removes something, such as an air
filter removes airborne contaminants so that the internal combustion engine of your
automobile does not grind itself to death on those particulates. In my high school and
college chemistry classes, filter paper was used to remove particulates from a liquid. The
air filter in my home HVAC system removes particulates that I don’t want to breathe.
So, although they do sometimes filter out unwanted data from a stream, I much prefer
the term “transformers” because these utilities do so much more. They can add data to
a stream, modify the data in some amazing ways, sort it, rearrange the data in each line,
perform operations based on the contents of the data stream, and so much more. Feel
free to use whichever term you prefer, but I prefer transformers

- Use the Linux FHS
> The Linux Filesystem Hierarchical Standard(FHS) defines the structure of the Linux directory tree.

- Embrace the CLI
> The force is with Linux, and the force is the command-line interface - the CLI.

The command line is a tool that provides a text mode interface between the user
and the operating system. The command line allows the user to type commands into the
computer for processing and to see the results.

