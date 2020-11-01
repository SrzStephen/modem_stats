
build
```

```
check 
```
pi@raspberrypi:~/stuff $ readelf -A target/armv7-unknown-linux-musleabihf/release/stats_server
Attribute Section: aeabi
File Attributes
  Tag_CPU_name: "7-A"
  Tag_CPU_arch: v7
  Tag_CPU_arch_profile: Application
  Tag_ARM_ISA_use: Yes
  Tag_THUMB_ISA_use: Thumb-2
  Tag_FP_arch: VFPv3
  Tag_Advanced_SIMD_arch: NEONv1
  Tag_ABI_PCS_GOT_use: GOT-indirect
  Tag_ABI_PCS_wchar_t: 4
  Tag_ABI_FP_rounding: Needed
  Tag_ABI_FP_denormal: Needed
  Tag_ABI_FP_exceptions: Needed
  Tag_ABI_FP_number_model: IEEE 754
  Tag_ABI_align_needed: 8-byte
  Tag_ABI_enum_size: int
  Tag_ABI_VFP_args: VFP registers
  Tag_ABI_optimization_goals: Aggressive Size
  Tag_CPU_unaligned_access: v6
  Tag_ABI_FP_16bit_format: IEEE 754

```

features

```
pi@raspberrypi:~/stuff $ cat /proc/cpuinfo
processor	: 0
model name	: ARMv7 Processor rev 3 (v7l)
BogoMIPS	: 270.00
Features	: half thumb fastmult vfp edsp neon vfpv3 tls vfpv4 idiva idivt vfpd32 lpae evtstrm crc32 

```

features_modem
```
root@mymodem:~# cat /proc/cpuinfo 
processor	: 0
model name	: ARMv7 Processor rev 1 (v7l)
BogoMIPS	: 398.13
Features	: half thumb fastmult edsp tls 
```

original build
```
CARGO_TARGET_ARM_UNKNOWN_LINUX_MUSLEABIHF_LINKER=arm-linux-gnueabihf-ld REALGCC=arm-linux-gnueabihf-gcc-8 TARGET_CC=musl-gcc cross build --release --target armv7-unknown-linux-musleabihf
```