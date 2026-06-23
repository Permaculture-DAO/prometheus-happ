# Signing & Publication — `v1.1.3-runtime-proof` (maintainer action)

> You sign; the agent never signs/publishes for you. Sign with key
> **`8C8CB0D48C7F60DA`** (the only valid key). Sign from **native PowerShell**, not Git
> Bash (Git Bash breaks GnuPG keyboxd). Do this only after you accept the candidate +
> (per governance) steward ratification. Nothing here changes the prose canon.

## 0. Stage the controlling artefacts
Copy the two WP4-controlling binaries into a clean Windows working dir, e.g.
`D:\prometheus-release\v1.1.3-runtime-proof\`:
- `hearth_prometheus.happ`  (from the signed runtime dir, expected SHA256 `a485337506bae6ea605c77e497fba429b52a240d4b6312e1d49f3819cce3de9b`)
- `dnas\hearth\hearth.dna`   (expected SHA256 `e18b4de1034c810af71a5a6ca06b44d5cec4791989b6bf969c1c9e6025932b55`)

Copy this `release/v1.1.3-runtime-proof/` folder (the .md docs) alongside them.

## 1. Checksums — confirm they match the manifest BEFORE signing
```powershell
cd D:\prometheus-release\v1.1.3-runtime-proof
Get-FileHash hearth_prometheus.happ -Algorithm SHA256
Get-FileHash dnas\hearth\hearth.dna -Algorithm SHA256
# both must equal RELEASE_MANIFEST.md. If not, STOP — do not sign.
"a485337506bae6ea605c77e497fba429b52a240d4b6312e1d49f3819cce3de9b *hearth_prometheus.happ`ne18b4de1034c810af71a5a6ca06b44d5cec4791989b6bf969c1c9e6025932b55 *dnas/hearth/hearth.dna" | Out-File -Encoding ascii SHA256SUMS.txt
```

## 2. Detached GPG signatures (you supply the passphrase)
```powershell
$gpg = "C:\Program Files\GnuPG\bin\gpg.exe"
foreach ($f in @("hearth_prometheus.happ","dnas\hearth\hearth.dna","RELEASE_MANIFEST.md","RELEASE_NOTES.md","CHANGELOG.md","SHA256SUMS.txt")) {
  & $gpg --armor --detach-sign --local-user 8C8CB0D48C7F60DA $f
}
```

## 3. Verify before publishing
```powershell
foreach ($f in @("hearth_prometheus.happ","dnas\hearth\hearth.dna","RELEASE_MANIFEST.md","SHA256SUMS.txt")) {
  & $gpg --verify "$f.asc" $f   # expect: Good signature / Firma valida
}
```

## 4. Publish to the public registry
Import into `prometheus-canonical-releases` under `releases/v1.1.3-runtime-proof/`
(binaries are distributed via the GitHub release, only `.asc`+manifest+sums in-repo, as
v1.1.2-genesis did), update `CURRENT_RELEASE` only after ratification, then:
```bash
git add releases/v1.1.3-runtime-proof
git commit -m "release(runtime): v1.1.3-runtime-proof signed artefacts + manifest"
git tag -s v1.1.3-runtime-proof -u 8C8CB0D48C7F60DA -m "v1.1.3-runtime-proof (runtime; prose canon unchanged)"
git push origin main v1.1.3-runtime-proof
gh release create v1.1.3-runtime-proof --repo Permaculture-DAO/prometheus-canonical-releases \
  --title "v1.1.3-runtime-proof (runtime)" \
  --notes-file releases/v1.1.3-runtime-proof/RELEASE_NOTES.md \
  hearth_prometheus.happ hearth_prometheus.happ.asc dnas/hearth/hearth.dna dnas/hearth/hearth.dna.asc SHA256SUMS.txt SHA256SUMS.txt.asc
```

## Boundary
This publishes a **runtime** release with **synthetic-proof** scope only (see
`CLAIM_BOUNDARY.md`). It does not amend the prose canon (`v1.1.2-genesis`), assert
validation/value/rights, or become `CURRENT_RELEASE` without steward ratification.
