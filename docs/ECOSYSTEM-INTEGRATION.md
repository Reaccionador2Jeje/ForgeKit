# ForgeKit — Ecosystem Integration

Status: planning document; current read-only diagnostic behavior is the baseline.

## Safety contract
- Diagnostics must remain read-only unless a separately reviewed repair design is introduced.
- Do not run shell commands from collected system metadata.
- Report unavailable metrics as unavailable, not as zero.
- Avoid collecting or printing secrets, personal files or unnecessary identifiers.
- Clearly distinguish host platform support from cross-compilation.

## Next tasks
1. Add fixture tests for command output and error handling.
2. Test unsupported operating systems and permission-limited metrics.
3. Document platform-specific metric differences.
4. Keep any future repair command behind a separate explicit confirmation design.