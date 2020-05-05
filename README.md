# Sysunit

Minimal system state management tool

## Rationale

Configuration managment tools such as Ansible and Puppet, while fitting for
many applications, are generally overly complex for basic environments.
Shell scripts, on the other extreme, are generally brittle and difficult to
maintain over time.

Sysunit aims to facilitate management of a single system's state by providing an
execution engine that composes simple executables, written in any language, to
idempotentally apply small, easily tracked changes to a system.

## Units

Units are executables which are meant to be very simple, and apply only minor
alterations to system state.  They are invoked by `sysunit` to perform some
operation.

For example, this unit will idempotently download a cat image to
some target file:

```sh
#/etc/units/kitty

#!/bin/sh

set -eu

case $1 in
check) [ -f $target ] && echo "ok";;
apply) curl 'https://placekitten.com/500x500' > $target;;
rollback) rm $target;;
deps) echo pkg name=curl;;
esac

```

When applied with `sysunit apply kitty target=~/pics/awww.jpg`, sysunit
will first invoke the unit with the equivalent of
`target=~/pics/awww.jpg /etc/units/kitty check`, followed by 
`target=~/pics/awww.jpg /etc/units/kitty rollback`, if it is not present.

### Directory Units

Units may contain data which needs a place in the filesystem, or supporting
libraries which a unit author may wish to break out into their own files.
Sysunit allows units to be provided as directories, which must contain a
`./unit` executable which behaves as a stand-alone executable unit would.  This
will be executed in its definition directory so accompanying files are easily
referenced with relative paths.

Care should be given not to alter accompanying files in the unit's directory, as
these changes will not be reverted at the end of unit execution and may result
in subsequent unit executions having their behavior altered from intended.

## Operations

- *apply* alters the system with the unit's target state if it is not present
- *rollback* removes the unit's target state from the system
- *check* determines whether the state which the unit applies is present
- *deps* provides a list of other units with the parameters which the state
         this unit affects is dependant upon

## Configuration

*SYSUNIT_PATH* may contain a colon-delimited list of directories which will
               be searched for unit executables or directories.
