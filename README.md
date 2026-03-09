# To build:

Replace the test [sqlite file](data/data-02.db) 

```
nix-build default.nix
```

Then run the resulting binary

app is available at:
localhost:3001

Some personal notes and thinking process is located [here](docs/dev-journal.md)


# Known bugs and limitations:

Longer sessions causes in memory "intermediate" sqlitedb to drop, resulting in data going missing.
Hovering over cells to get the example apps is unimplemented
