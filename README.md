# memfd-ed25519
PoC with built-in app (app.c) with ed25519 verification

Keys were generated using python `cryptography` package

* make build - Build executable with app.c inside
* make clean - Remove build files
* make run - Run executable with app.c inside
* make tamper - Modify executable to trigger wrong signature check