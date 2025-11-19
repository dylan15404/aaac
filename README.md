# 
<h1 align="center">
  archangelAC
  <br>
</h1>

<h4 align="center">a 'safe' modular linux anticheat solution written in rust</h4>
# 
## feature progress

* base features
  - 🟩 modular structure with dynamic loading of module libraries
  - 🟥 control via config file

* tier 1 modules
  - 🟥 TPM2 EK + PCR0–10 quote + nonce
  - 🟥 TPM2 sealed keys (offline attestation)
  - 🟥 verified Boot + sbverify/sbattach checks
  - 🟥 seccomp-bpf strict profile
  - 🟥 landlock filesystem restrictions
  - 🟥 fs-verity signatures on game files/libs
  - 🟥 bubblewrap / systemd-nspawn sandbox
  - 🟥 systemd-confine + DynamicUser + ProtectSystem=strict

* tier 2 modules
  - 🟥 IMA-appraisal + custom policy (verify every executable & library the game loads)
  - 🟥 kernel lockdown = integrity + signed modules only
  - 🟥 signed eBPF CO-RE monitoring/blocking
  - 🟥 exact PCR whitelisting + mandatory sealed keys




## aim
- purely user-mode anti-cheat baseline that is objectively stronger and less invasive than current windows kernel-level solutions by leveraging linux-native security subsystems
- experimental more agressive kernel level mode (tier 2)
- extra features for  immutable distros to increase security further

---

> GitHub [@dylan15404](https://github.com/dylan15404) &nbsp;&middot;&nbsp;
> X [@minorponky](https://x.com/minorponky)
