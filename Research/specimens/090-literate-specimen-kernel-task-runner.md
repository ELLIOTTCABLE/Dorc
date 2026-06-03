# 090 — literate specimen: a kernel-dev task-runner as a Dorc design quarry

> AI-generated literate read (round 9, 2026-06-03). The shell script below is reproduced
> VERBATIM (fetched via gh/curl, byte-checked); commentary between code blocks was added
> afterward by editing *around* the source, so neither source nor attribution can be
> corrupted by paraphrase. Per the human steer this is a DESIGN QUARRY, not an
> inferability measurement: we mine how facts are spelled idiomatically so the designer
> can pick which spellings to *bless* (collapse to a fixed analyzer-recognized form) and
> which to *abdicate* (delegate to community-named state-kinds). See "The lens" below.

## Source & attribution
- Repo: FlorentRevest/linux-kernel-vscode — file `tasks.sh`
- Permalink (commit-pinned): https://github.com/FlorentRevest/linux-kernel-vscode/blob/a031e02118c6197878e2fbde7468aa18a40885fa/tasks.sh
- Raw: https://raw.githubusercontent.com/FlorentRevest/linux-kernel-vscode/a031e02118c6197878e2fbde7468aa18a40885fa/tasks.sh
- Commit: a031e02118c6197878e2fbde7468aa18a40885fa
- License: GPL-2.0
- Retrieved: 2026-06-03 via gh api + curl; sha256 b6e7f3504cdd02713a4181ef34bcd85a317f28ace6ff2aa7004653fd3a1594b5

## The lens: bless vs abdicate

Two design verbs recur below; tagging idioms with them is the point of the read. **Bless** = collapse a fact to *one* fixed, mechanical spelling the analyzer recognises, and deliberately ignore every other spelling — as TypeScript narrows only on a guard of the exact form `x is T`, never on an arbitrary runtime predicate. Blessing *bounds library bloat*: instead of teaching Dorc the open-ended zoo of ways to express a fact, you require oracle authors to wrap it the blessed way or be missed. **Abdicate** = refuse to enumerate an open-ended equivalence (every package manager, every mount mechanism) and lean instead on a community-defined *named kind* (`package`, `service`, `filesystem-access`) that an oracle's provider declares it satisfies — as TypeScript declines to enumerate every URL flavour and leaves you to define `URL<T>`. Each idiom below is tagged `[bless?]` or `[abdicate?]` as a *candidate only*; the designer decides, this just surfaces the raw material.

The script is a VSCode-integrated Linux-kernel dev task-runner: build the kernel, build a Debian rootfs image, boot it in QEMU, ssh in, fuzz, trace. It is *local dev/build glue*, not remote-fleet ops — but it is unusually dense with the patterns Dorc cares about (transient state, idempotency guards, atomic publish, a real two-user split), and comment-rich, so the author's intent is legible.

## The script, annotated

```bash
#!/bin/bash

# If you want to modify how tasks behave:
# - Keep features specific to your special use cases in the local.sh file.
# - Send PRs to github.com/FlorentRevest/linux-kernel-vscode for changes that
#   would benefit all users.
# This improves the framework and makes sure you can always run the update task.

function depend_on() {
  $SCRIPT $@
  if [[ "$CLEAR" == 1 ]]; then
    clear
  fi
}

function spinner() {
  local pid=$1

  if [[ "$SPINNER" -eq 1 ]]; then
    local spin='⣾⣽⣻⢿⡿⣟⣯⣷'
    local i=0
    tput civis # Hide cursor
    while kill -0 $pid 2>/dev/null; do
      local i=$(((i + 1) % ${#spin}))
      printf "%s" "${spin:$i:1}" # Print one character
      echo -en "\033[$1D" # Go back one character
      sleep .1
    done
    tput cnorm # Restore cursor
  fi

  wait $pid
  return $?
}
```

The header is already a Dorc artifact. `local.sh` (sourced in the next block) is the **admin's** environment-specific layer; the upstreamed `tasks.sh` is the **engineer's** reusable framework — the *book*/*oracle* split (AGENTS reminder #2), built by a real author, and the closing line *"makes sure you can always run the update task"* names the no-cliff seam between them (that `update` task is the final block). `depend_on` is a hand-rolled task-DAG: `$SCRIPT $@` recurses into the script with a sub-command, so the dependency graph is spelled as *self-invocation* — this recursion is the CFG edge Dorc would lift into a real graph. `spinner` is pure terminal UI with no system-state effect: the kind of leaf analysis treats as a no-op.

```bash
set -e

# Arguments extraction
if [ "$#" -lt 1 ]; then
  echo "Usage: $0 command"
  exit 1
fi
COMMAND=$1

# See https://www.gnu.org/software/bash/manual/html_node/Shell-Parameter-Expansion.html
# for the `: ${var:=DEFAULT}` syntax
: ${SCRIPT:=`realpath -s "$0"`}
: ${SCRIPT_DIR:=`dirname "${SCRIPT}"`}

# Let the user override environment variables for their special needs
files_to_source=$(find ${SCRIPT_DIR}/ -maxdepth 1 -xtype f -name "local*.sh")
for file in $files_to_source; do
  source "$file"
done

# Default context variables, can be overridden by local.sh or in environment.
: ${WORKSPACE_DIR:=`realpath -s "${SCRIPT_DIR}/.."`}
: ${MAKE:="make -j`nproc` LLVM=1 LLVM_IAS=1 CC='ccache clang'"}
: ${TARGET_ARCH:="x86_64"}
: ${TARGET_GDB:="gdb-multiarch"}
: ${SILENT_BUILD_FLAG="-s"}
: ${SUCCESSFUL_EXIT_COMMAND:=""}
: ${BPF_SELFTESTS_DIR:="${WORKSPACE_DIR}/tools/testing/selftests/bpf"}
: ${VM_START_ARGS:=''}
: ${SYZ_MANAGER_CFG_EXTRA:=''}
: ${SYZKALLER_DIR:="${SCRIPT_DIR}/syzkaller/"}
: ${KERNEL_CMDLINE_EXTRA:=''}
: ${SPINNER:=1}
: ${IMAGE_DIR:="${HOME}/.linux-kernel-vscode"}
: ${IMAGE_PATH:="${IMAGE_DIR}/debian-${TARGET_ARCH}.img"}
: ${TRACER_PATH:=".vscode/autostart/tracer.stp"}
if [[ "$TERM_PROGRAM" == "vscode" ]]; then
  : ${CLEAR:=1}
fi
if [[ $SKIP_SYSTEMD == 1 ]]; then
  KERNEL_CMDLINE_EXTRA="init=/sbin/init-minimal $KERNEL_CMDLINE_EXTRA"
fi

# Convenience environment variables derived from the context
if [ "${TARGET_ARCH}" = "x86_64" ]; then
  : ${VMLINUX:="bzImage"}
  : ${CLANG_TARGET:="x86_64-linux-gnu"}
  : ${MKOSI_TARGET_ARCH:="x86-64"}
  : ${TOOLS_SRCARCH:="x86"}
  : ${QEMU_BIN:="qemu-system-x86_64"}
  : ${QEMU_CMD:="${QEMU_BIN} -enable-kvm -cpu host -machine q35 -bios qboot.rom"}
  : ${SERIAL_TTY:="ttyS0"}
  : ${SYZKALLER_TARGETARCH:="amd64"}
  : ${ROOT_MNT:="/dev/sda"}
elif [ "${TARGET_ARCH}" = "arm64" ]; then
  : ${VMLINUX:="Image"}
  : ${CLANG_TARGET:="aarch64-linux-gnu"}
  : ${MKOSI_TARGET_ARCH:="arm64"}
  : ${TOOLS_SRCARCH:="arm64"}
  : ${QEMU_BIN:="qemu-system-aarch64"}
  : ${QEMU_CMD:="${QEMU_BIN} -cpu max -machine virt"}
  : ${SERIAL_TTY:="ttyAMA0"}
  : ${PROOT_ARGS:="-q qemu-aarch64-static"}
  : ${SYZKALLER_TARGETARCH:="arm64"}
  : ${ROOT_MNT:="/dev/vda"}
else
  echo "Unsupported TARGET_ARCH:" $TARGET_ARCH
  exit 2
fi

: ${KERNEL_PATH:="${WORKSPACE_DIR}/arch/${TARGET_ARCH}/boot/${VMLINUX}"}

# When called outside of a VSCode task, the current working directory can be
# somewhere else than the workspace. Since we implicitly rely on pwd being the
# top of the kernel tree quite often, cd there.
pushd "$WORKSPACE_DIR" >/dev/null

if [[ "$CLEAR" == 1 ]]; then
  clear
fi

# SSH Keys
: ${SSH_KEY:="${HOME}/.ssh/linux-kernel-vscode-rsa"}
: ${SSH_CMD:="ssh -p 5555 -i ${SSH_KEY} -o IdentitiesOnly=yes -o NoHostAuthenticationForLocalhost=yes root@localhost"}
: ${SCP_CMD:="scp -P 5555 -r -i ${SSH_KEY} -o IdentitiesOnly=yes -o NoHostAuthenticationForLocalhost=yes"}
if [ ! -f ${SSH_KEY} ]; then
  ssh-keygen -t rsa -f ${SSH_KEY} -N "" -q
fi

# QEMU start command
: ${VM_START:="${QEMU_CMD} -s -nographic -smp 4 -m 4G -qmp tcp:localhost:4444,server,nowait -serial mon:stdio \
    -net nic,model=virtio-net-pci -net user,hostfwd=tcp::5555-:22 \
    -virtfs local,path=/,mount_tag=hostfs,security_model=none,multidevs=remap \
    -append \"console=${SERIAL_TTY},115200 root=${ROOT_MNT} rw nokaslr init=/lib/systemd/systemd debug systemd.log_level=info ${KERNEL_CMDLINE_EXTRA}\" \
    -drive file=${IMAGE_PATH},format=raw -kernel ${KERNEL_PATH} ${VM_START_ARGS}"}
```

Four spelled facts here. `set -e` turns textual order into dependency — with it an earlier failure aborts the rest, so order *matters*; without it the commands read as a bundle of independents. `[bless?]` presence-of-`set -e` is a candidate native signal for "treat sequence as ordered" vs "parallelisable" (TODO line 8's hypothesis, in the wild). The `: ${VAR:=default}` wall plus `source local*.sh` is the **override seam**: the admin injects environment without forking — `[abdicate?]` the framework never tries to *know* the admin's environment, it just exposes a declared injection point. The arch dispatch (`if x86_64 … elif arm64 … else exit 2`) is provider-selection-by-host-fact with explicit rejection of the unknown, the same shape as package-manager dispatch one rung up — `[abdicate?]` don't enumerate arches, treat "arch" as a kind. And `if [ ! -f ${SSH_KEY} ]; then ssh-keygen` is the first idempotency guard — `[bless?]` `if [ -f X ]` is the cleanest candidate for *the* blessed file-existence-dependency form (the human's own example: bless `[ -f thefile ]`, never `cool-checker --exists`). This is also the most boring block; the `: ${VAR:=…}` list is repetitive, kept whole per lean-to-keep, and the obvious elision candidate if ever trimmed.

```bash
case "${COMMAND}" in
# Virtual machine life-cycle
  "start")
    depend_on install-autostart
    eval ${VM_START}
    ;;
  "start-wait-dbg")
    depend_on install-autostart
    eval ${VM_START} -S
    ;;
  "stop")
    # With SKIP_SYSTEMD, nothing handles ACPI shutdowns so clean shutdown does not work.
    if [[ -z $SKIP_SYSTEMD ]]; then
      echo -n '{"execute":"qmp_capabilities"} {"execute": "system_powerdown"}' | nc -q 1 localhost 4444
    else
      killall ${QEMU_BIN}
    fi
    ;;
  "ssh")
    eval ${SSH_CMD}
    ;;
  "run")
    shift
    eval ${SSH_CMD} $@
    ;;
  "wait-for-vm")
    # On the first boot, a rootfs isn't yet available. Because debootstrap can
    # take a while to run, this waits for the rootfs file to show up.
    timeout 120 bash -c "until [ -f ${IMAGE_PATH} ] ; do sleep 0.01; done"
    ;;
# Kernel build
  "defconfig")
    # Only generate .config if it doesn't already exist
    if [ ! -f ${WORKSPACE_DIR}/.config ]; then
      eval ${MAKE} ARCH=${TARGET_ARCH} defconfig kvm_guest.config
      scripts/config --enable DEBUG_INFO_DWARF_TOOLCHAIN_DEFAULT
      eval ${MAKE} ARCH=${TARGET_ARCH} olddefconfig
    fi
    ;;
  "menuconfig")
    # It's important to run menuconfigs with the same parameters as builds
    eval ${MAKE} ARCH=${TARGET_ARCH} menuconfig
    ;;
  "clean")
    eval ${MAKE} ARCH=${TARGET_ARCH} clean
    ;;
  "build")
    depend_on defconfig

    # Enable reproducible builds for ccache
    export KBUILD_BUILD_TIMESTAMP=""
    # Generate not only the kernel but also the clangd config
    CMD="${MAKE} ${SILENT_BUILD_FLAG} ARCH=${TARGET_ARCH} all compile_commands.json"
    echo ${CMD}
    eval ${CMD} &
    spinner $!

    # A gdb index may need to be re-generated. Don't clear the above make logs.
    CLEAR=0 $SCRIPT gdb-index
    # A tracer module may need to be re-built
    CLEAR=0 $SCRIPT systemtap-build
    ;;
  "gdb-index")
    # Hitting a breakpoint is *much* faster if we pre-build a gdb symbol index
    if ! readelf -S vmlinux | grep -q ".gdb_index"; then
      OBJCOPY=llvm-objcopy GDB=${TARGET_GDB} gdb-add-index vmlinux
    fi
    ;;
```

The `case "${COMMAND}"` is the book's task table, and the author's own section comments (`# Virtual machine life-cycle`, `# Kernel build`) are candidate role boundaries. Two marks. `wait-for-vm` polls `until [ -f ${IMAGE_PATH} ]` — it is a *consumer* of an ambient fact used as a synchronisation barrier; hold it, because the next block's `create-rootfs` is the *producer* and the pair forms a TOCTOU-safe handshake. `defconfig` and `gdb-index` are both VALUE-band guards (a cheap check fronting an expensive apply), but they differ in *depth*: `[ ! -f .config ]` is bare existence, while `readelf -S vmlinux | grep -q ".gdb_index"` probes *a property inside* an artifact. `[bless?]` these may want to be two distinct blessed forms — "artifact exists" vs "artifact has property P" — since the second is still cheap-and-shallow yet is not a file-existence check.

```bash
# Rootfs management
  "create-rootfs")
    # Only generate a rootfs if it doesn't already exist
    if [ ! -f ${IMAGE_PATH} ]; then
      img="$(mktemp -u --suffix=.img)"
      img_mnt="$(mktemp -d)"
      img_bind_mnt="$(mktemp -d)"
      trap 'rm -f ${img}; sudo umount -l ${img_bind_mnt}; sudo umount -l ${img_mnt}; rmdir ${img_mnt} ${img_bind_mnt}' ERR
      # Image file creation
      qemu-img create ${img} 20G
      mkfs -t ext4 ${img}

      # Mounts (bind mounts for permission)
      mkdir -p ${img_mnt} ${img_bind_mnt}
      echo "password required to mount the rootfs:"
      sudo mount -o loop ${img} ${img_mnt}
      sudo bindfs --uid-offset=$(id -u) --gid-offset=$(id -g) \
          --create-with-perms=0644,ud+X:gd-rwX:od-rwX ${img_mnt} ${img_bind_mnt}

      # Debian rootfs generation and config setting
      sudo mkosi --architecture=${MKOSI_TARGET_ARCH} --distribution=debian --release=unstable --output-dir=${img_mnt} --format=directory \
      --package=ssh,acpid,acpi-support-base,gdb,systemtap,file,psmisc,strace,vim,bpftool,bpftrace,trace-cmd,linux-perf \
      --package=apt,less,login,iputils-ping,iproute2,cron,e2fsprogs,systemd-sysv,cpio,dhcpd,fdisk,udev,man

      # Move mkosi-generated rootfs from ${img_mnt}/image to ${img_mnt} to match script's expected directory structure
      sudo mv ${img_mnt}/image/* ${img_mnt} && sudo rmdir ${img_mnt}/image

      echo "debian-vm" > ${img_bind_mnt}/etc/hostname
      echo "nameserver 8.8.8.8" > ${img_bind_mnt}/etc/resolv.conf
      echo "hostfs /host 9p trans=virtio,rw,nofail 0 0" > ${img_bind_mnt}/etc/fstab
      printf "[Match]\nName=en*\n[Network]\nDHCP=yes" > ${img_bind_mnt}/etc/systemd/network/80-dhcp.network
      sed -i 's~^ExecStart=.*~ExecStart=-/sbin/agetty --autologin root -o "-p -f root" --keep-baud 115200,57600,38400,9600 - $TERM~' ${img_bind_mnt}/lib/systemd/system/serial-getty@.service
      mkdir -p ${img_bind_mnt}/root/.ssh/
      cp ${SSH_KEY}.pub ${img_bind_mnt}/root/.ssh/authorized_keys
      sudo chroot ${img_mnt} systemctl enable systemd-networkd acpid
      cat << EOF > ${img_bind_mnt}/sbin/init-minimal
#!/bin/sh

# Mount various important file systems
mkdir -p /proc /sys /run/ /tmp /dev
mount -t proc none /proc
mount -t sysfs none /sys
mount -t tmpfs none /run
mount -t tmpfs none /tmp
mount -t devtmpfs none /dev
mkdir -p /dev/pts
mount -t devpts none /dev/pts
# And the content of /etc/fstab
mount -a

# Set the network interface up
cat /etc/hostname > /proc/sys/kernel/hostname
ip link set eth0 up
dhclient eth0

# Change cwd to /root
HOME=/root
cd $HOME

# Start the SSH server
mkdir /run/sshd/
/usr/sbin/sshd

# Start the autostart script if there is one
[ -f /usr/bin/autostart.sh ] && /usr/bin/autostart.sh &

# Set up the serial line and get to a bash prompt
setsid /sbin/getty -l /bin/bash -n 115200 ${SERIAL_TTY}
EOF
      chmod +x ${img_bind_mnt}/sbin/init-minimal
      cat << EOF > ${img_bind_mnt}/etc/bash.bashrc
# Use a green or red prompt depending on the previous command's exit value
__prompt () {
   if [[ $? = "0" ]]; then
     PS1='\[\033[01;32m\]'
   else
     PS1='\[\033[01;31m\]'
  fi
  PS1="$PS1\u@\h:\w\$ \[\033[00m\]"
}
PROMPT_COMMAND=__prompt

# On the serial tty, ask the host terminal for dimensions before each command
__resize () {
  local escape r c
  IFS='[;' read -t 1 -sd R -p "$(printf '\e7\e[r\e[999;999H\e[6n\e8')" escape r c
  if [[ "$r" -gt 0 && "$c" -gt 0 ]]; then
    stty cols $c rows $r
  fi
}
if [[ $TERM = "vt102" ]]; then
  trap __resize DEBUG
fi
EOF

      # Atomically make the rootfs file available to unblock wait-for-vm tasks
      sync
      sudo umount -l ${img_bind_mnt}
      sudo umount -l ${img_mnt}
      mkdir -p "${IMAGE_DIR}"
      mv "${img}" "${IMAGE_PATH}"
    fi
    ;;
```

The centre of gravity — five points.

1. `if [ ! -f ${IMAGE_PATH} ]` guards a multi-minute `mkosi` Debian build: the VALUE band at its starkest (cents of check, minutes of apply).
2. `mktemp` paths + `trap '…umount…' ERR` establish transient state. `[bless?]` this is the named candidate — bless `trap … EXIT/ERR` as *the* canonical "this fact is transient, do not hoist its dependents into the resting-state probe" marker, and deliberately miss every ad-hoc mount/save spelling. That one mechanical rule is what lets the designer *refuse* to enumerate the mount zoo and push the work onto oracle authors (wrap transient state in `trap`, or be missed) — the library-bloat bound, TS-type-guard-style.
3. The body provisions *through* the mount — `mkosi` packages, then writes `/etc/hostname`, `resolv.conf`, `fstab`, a systemd network unit, `sed`s the getty unit, drops `authorized_keys`, `chroot … systemctl enable`. That is a Dorc role's worth of work (packages + files + services + users), with the mount acting as the **transport** — Dorc's SSH hop, here spelled as a loop-mount. "The connection" can be a mount, not only a network hop.
4. The two `cat << EOF` heredocs write *whole scripts into the image* (`init-minimal`, which does its own `mount -t proc …`; `bash.bashrc`, which installs its own `trap __resize DEBUG`). Those execute later, in the guest — a **code-generation boundary**: generated-then-deferred code is data here, not this script's control-flow, and the analyzer must recognise the heredoc edge and stop rather than reason into it.
5. The close (comment: *"Atomically make the rootfs file available to unblock wait-for-vm tasks"*) is `sync; umount; umount; mv "${img}" "${IMAGE_PATH}"`. The build runs entirely in transient temp space and the result appears by one atomic rename, so `IMAGE_PATH` exists *iff* the build fully succeeded — which makes *both* consumers sound at once: this case's own idempotency guard, and `wait-for-vm`'s barrier two blocks up (a probe→use gap closed by atomic publish — note 090's TOCTOU/OCC, in real ops). `[abdicate?]` this can't be a syntactic rule; it is a *contract on oracle authors* ("publish atomically so the existence-probe stays sound"), and it shows transient-state hygiene and elision-soundness are one property.

(Across the whole file, five providers of "make image contents accessible": `mount -o loop`, `bindfs`, `guestmount`, `guestfish`, `proot` — the abdicate case for a `filesystem-access` kind.)

```bash
  "install-autostart")
    depend_on create-rootfs

    cd .vscode/autostart/
    BUILT_AUTOSTART=${IMAGE_DIR}/autostart-${TARGET_ARCH}

    # The poor man's make. We use the last built /tmp/autostart to track if any
    # of the source file has changed. Only if one changed, rebuild and install.
    if [ ${BUILT_AUTOSTART} -nt autostart.c ] && \
       [ ${BUILT_AUTOSTART} -nt autostart.sh ] &&
       [ ${BUILT_AUTOSTART} -nt autostart.service ]; then
      echo "Autostart already up to date"
      exit 0
    fi

    clang --target=${CLANG_TARGET} -fuse-ld=lld `cat compile_flags.txt` autostart.c -o ${BUILT_AUTOSTART}

    echo Installing autostart on `basename ${IMAGE_PATH}`
    guestfish --rw -a "${IMAGE_PATH}" << EOF
      run
      mount /dev/sda /

      upload ${BUILT_AUTOSTART} /usr/bin/autostart
      chmod 755 /usr/bin/autostart

      upload autostart.sh /usr/bin/autostart.sh
      chmod 755 /usr/bin/autostart.sh

      upload autostart.service /lib/systemd/system/autostart.service
      ln-sf /lib/systemd/system/autostart.sh /etc/systemd/system/multi-user.target.wants/autostart.service
EOF
    ;;
  "push")
    if [ "$#" -lt 2 ]; then
      echo "Usage: $0 push /file/to/push [/destination]"
      exit 1
    fi
    popd >/dev/null
    eval ${SCP_CMD} ${2} root@localhost:${3:-/root}
    ;;
  "pull")
    if [ "$#" -lt 2 ]; then
      echo "Usage: $0 pull /file/to/pull [/destination]"
      exit 1
    fi
    popd >/dev/null
    eval ${SCP_CMD} root@localhost:${2} ${3:-.}
    ;;
  "chroot")
      img_mnt="$(mktemp -d)"
      echo "password required to mount the rootfs:"
      sudo mount -o loop ${IMAGE_PATH} ${img_mnt}
      trap 'sudo umount -l ${img_mnt}; rmdir ${img_mnt}' EXIT
      sudo proot -S ${img_mnt} -w / ${PROOT_ARGS}
    ;;
# BPF selftests
  "install-bpf-selftests")
    # Mount the poor man's sysroot
    ROOTFS_MOUNT_POINT=${HOME}/.linux-kernel-vscode/mnt
    echo "Mounting the VM's rootfs as a sysroot under ${ROOTFS_MOUNT_POINT}. If you miss any library, just install them in the VM:"
    echo "  apt install libstdc++-12-dev libz-dev libelf-dev libcap-dev"
    mkdir -p ${ROOTFS_MOUNT_POINT}
    guestmount -a ${IMAGE_PATH} -m /dev/sda --ro -o dev ${ROOTFS_MOUNT_POINT}
    trap "guestunmount ${ROOTFS_MOUNT_POINT}" EXIT

    # Compile
    CLANG_CROSS_FLAGS="--target=${CLANG_TARGET} --sysroot=${ROOTFS_MOUNT_POINT}" \
      eval ${MAKE} CROSS_COMPILE=${CLANG_TARGET}- SRCARCH=${TOOLS_SRCARCH} -C ${BPF_SELFTESTS_DIR}

    eval ${SCP_CMD} ${BPF_SELFTESTS_DIR}/test_progs root@localhost:/root
    ;;
  "run-bpf-selftests")
    depend_on install-bpf-selftests
    eval ${SSH_CMD} /root/test_progs
    ;;
  "run-bpf-selftest")
    if [ "$#" -ne 2 ]; then
      echo "Usage: $0 run-bpf-selftest selected_file"
      exit 1
    fi
    SELECTED_FILE=$2
    if [ `dirname ${SELECTED_FILE}` == ${BPF_SELFTESTS_DIR}/prog_tests ]; then
      depend_on install-bpf-selftests
      eval ${SSH_CMD} "/root/test_progs -t `basename ${SELECTED_FILE} .c`"
    else
      echo -e "\e[31mOpen a test in ${BPF_SELFTESTS_DIR}/prog_tests/\e[0m"
    fi
    ;;
# Fuzzing
  "enable-kcov")
    depend_on defconfig
    if grep -q -F "CONFIG_KCOV=y" .config; then
      echo KCOV is already enabled
    else
      echo Enabling KCOV...
      scripts/config -e KCOV -e KCOV_ENABLE_COMPARISONS
      eval ${MAKE} ARCH=${TARGET_ARCH} olddefconfig

      echo Rebuilding the kernel with KCOV...
      ${SCRIPT} build
    fi
    ;;
  "fuzz")
    depend_on enable-kcov

    if [ ! -d "${SYZKALLER_DIR}" ] ; then
      git clone https://github.com/google/syzkaller ${SYZKALLER_DIR}
    fi
    make -C ${SYZKALLER_DIR} TARGETARCH=${SYZKALLER_TARGETARCH} manager fuzzer execprog executor

    cat > /tmp/syz-manager.cfg << EOF
{
    "target": "linux/${SYZKALLER_TARGETARCH}",
    "http": "0.0.0.0:56741",
    "sshkey": "${SSH_KEY}",
    "workdir": "${SCRIPT_DIR}/syzkaller-workdir",
    "kernel_obj": "${WORKSPACE_DIR}",
    "syzkaller": "${SYZKALLER_DIR}",
    "type": "isolated",
    "reproduce": false,
    ${SYZ_MANAGER_CFG_EXTRA}
    "vm": {
        "targets": [ "127.0.0.1:5555" ],
        "target_dir": "/root/fuzzing/",
        "target_reboot": false
    }
}
EOF
    ${SYZKALLER_DIR}/bin/syz-manager -config /tmp/syz-manager.cfg
    ;;
```

More of the same vocabulary, plus one sharp contrast. `install-autostart` opens with the author's *"poor man's make"* — `[ ${BUILT} -nt autostart.c ] && [ … -nt … ]` then `exit 0` — i.e. mtime freshness / early-cutoff (build-systems prior-art, note 075). `[bless?]` `[ A -nt B ]` is the candidate blessed *staleness* spelling. The contrast worth holding: `install-autostart` edits the image with `guestfish` (the mount happens *inside* libguestfs — no host-visible transient state, the whole edit is one opaque leaf), while `install-bpf-selftests` uses `guestmount` (host-visible FUSE) and `chroot` uses `mount -o loop` + `trap … EXIT`. The choice is driven by *whether another process must see the mount*: the cross-compiler needs the sysroot on the host VFS (guestmount), the file-upload does not (guestfish). That is a real degradation lever — *pushing transient state inside a single oracle-wrapped leaf (guestfish-style) makes it stop being the analyzer's problem*; the same fact, made analyzable by encapsulation. `enable-kcov` (`grep -q -F "CONFIG_KCOV=y" .config`) and `fuzz` (`[ ! -d "${SYZKALLER_DIR}" ]`) are two more VALUE-band guards (property-probe, directory-existence). Note too the EXIT-vs-ERR trap distinction across cases: `chroot`/`install-bpf-selftests` use EXIT (always tear down, they end in an interactive/long command), `create-rootfs` used ERR (tear down only on failure, the happy path publishes) — a considered lifecycle choice, not boilerplate.

```bash
# Tracing
  "systemtap-build")
    if [ -f ${TRACER_PATH} ]; then
      echo Re-building ${TRACER_PATH} ...
      # Workaround the presence of mcount nops with PR15123_ASSUME_MFENTRY
      # Skip the loading part of the pipeline with -p4. Use Guru mode with -g
      # Workaround clang warnings (treated as errors) with -Wno-everything
      PR15123_ASSUME_MFENTRY=1 stap -p4 -g -r ${WORKSPACE_DIR} -m tracer \
        -B LLVM=1 -B CFLAGS_MODULE="-Wno-everything" ${TRACER_PATH} > /dev/null
      # The guest doesn't know $WORKSPACE_DIR but it can hardcode /host/tmp/
      echo Installing to /tmp/tracer.ko ...
      mv tracer.ko /tmp/
    else
      rm -f /tmp/tracer.ko
    fi
    ;;
# linux-kernel-vscode pull
  "update")
    cd .vscode

    trap "cp -r /tmp/local.sh /tmp/autostart ." EXIT
    cp local.sh /tmp/
    cp -r autostart/ /tmp/

    git checkout -- local.sh autostart/*
    git pull

    chmod u+x "${SCRIPT_DIR}/tasks.sh"
    # see comments in the .jsonnet file to understand this magic.
    if [ ! -e "settings.json" ]; then
      # Seed JSonnet with empty object
      echo "{}" > "settings.json"
    fi
    if [ ! -e "settings-extra.json" ]; then
      # Seed JSonnet with empty object
      echo "{}" > "settings-extra.json"
    fi
    tmp="$(mktemp --suffix=.json)"
    jsonnet settings.jsonnet \
      --ext-code-file old_settings="settings.json" \
      --ext-code-file extra_settings="settings-extra.json" > "${tmp}"
    mv "$tmp" settings.json
    ;;
  *)
    echo "Invalid command"
    ;;
esac
```

## The last block, and the ledger

`systemtap-build` is a two-sided guard: `if [ -f ${TRACER_PATH} ]` builds the tracer, `else rm -f /tmp/tracer.ko` removes a now-stale artifact — the absence-branch *maintains an invariant*, it doesn't merely skip. `update` is the two-user seam the header promised: `trap "cp -r /tmp/local.sh /tmp/autostart ." EXIT; cp local.sh /tmp/; cp -r autostart/ /tmp/; git checkout -- local.sh autostart/*; git pull` — save the admin's layer, run the destructive framework update, restore it. That is the protect-across-a-destructive-step transient (a compensating pair / saga, note 090 D6), and the concrete no-cliff mechanism that lets admin and engineer co-evolve. The `settings.json` seeding (`[ ! -e settings.json ] && echo "{}"`) is one last idempotency guard.

### The bless/abdicate ledger this one script suggests
**Bless** (collapse to a fixed, analyzer-recognised form; miss the rest on purpose):
- `if [ -f X ]` — file-existence dependency.
- `[ A -nt B ]` — freshness / staleness dependency.
- `trap … EXIT/ERR` — transient-state ("do not hoist") marker.
- presence of `set -e` — "sequence is ordered" vs "bundle of independents".
- (open) a *second* existence form for "artifact has property P" (`readelf|grep`, `grep CONFIG=`), distinct from bare file-existence.

**Abdicate** (delegate to community-named kinds + provider declarations; refuse to enumerate):
- the package-manager / arch / filesystem-access zoos → named kinds an oracle's provider satisfies (the RAL).
- atomic-publish soundness → a *contract* on oracle authors ("publish atomically so the probe is sound"), not a syntactic rule.

**Meta**: even this careful, comment-rich author derives rich idempotency / transient / freshness signal for free, yet leaves dependency-presence (assumes `mkosi`, `guestfish`, `bindfs`, `proot`, `stap`, `jsonnet` exist — no `command -v` guards) and provider-equivalence entirely to convention. That is the right division: those are the abdicate column. The empirical shape is "bless the cheap structural facts, abdicate the open-ended equivalences."

> Specimen slug `sp-tr-1` from the round-9 corpus hunt; verbatim copy + annotation. Companion design points (`d1`–`d3`, `why-1`/`why-2`, `why-makes-guard-sound`, `new-twousers`) were developed in the round-9 chat.
