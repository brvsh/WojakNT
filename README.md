## WojakNT
Prints an ASCII wojak to the NT kernel debug buffer.

### Building & Signing
`cargo make sign --profile production`

### Loading
1. Disable Secure Boot
2. Using `bcdedit.exe`, enable testsigning.
3. Using OsrLoader, load and run the `WojakNT.sys` driver.
