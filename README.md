# RNV (Ruby Nano Vim)

**RNV** is a modular terminal text editor written from scratch in pure Ruby. It seamlessly merges the invisible execution speed and modality of **Vim** with the core hidden control keys of **Nano**.

### Decryption
- **R** — **Ruby** (Development Language Engine)
- **N** — **Nano** (Core hidden `Ctrl` combinations and behavior)
- **V** — **Vim** (Modality complexity, navigation via `h,j,k,l` and `ESC`)

---

## Project Structure

```text
rnv-editor/
├── rnv          # Main executable script (Application entry point)
├── core.rb      # Text buffer logic, file I/O operations and states
├── ui.rb        # ANSI render loop and keyboard capture engine
└── README.md    # Documentation and deployment guide
```

---

## Control Scheme

### Global Hotkeys (Active in Command Mode)
- `Ctrl + O` : Save active file.
- `Ctrl + X` : Flush memory and exit to shell.

### Vim-style Modality
- `i` : Transition into **INSERT Mode** (type text).
- `ESC` : Drop back into **COMMAND Mode**.
- `h` / `j` / `k` / `l` : Grid navigation (Left / Down / Up / Right).

##  Installation & Deployment

1. Clone or download files into a single directory and make the entry point executable:
```bash
chmod +x rnv
```

2. Run the editor:
```bash
./rnv your_file.txt
```

3. Flags:
```bash
./rnv --help       # Show application usage help
./rnv --version    # Check production build release
```
