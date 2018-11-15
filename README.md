# Redux Duck Helper

## Usage:

Only Mac OS binary provided (for now).

Run from root of you project:

```
./redux-duck duckname
```

It would create following files.

```
ceated: ./appState/duckname/actions/ducknameActions.ts
ceated: ./appState/duckname/reducers/ducknameReducers.ts
ceated: ./appState/duckname/sagas/ducknameSagas.ts
ceated: ./appState/duckname/types/ducknameTypes.ts
```

`tree` :

```
appState
└── duckname
    ├── actions
    │   └── ducknameActions.ts
    ├── reducers
    │   └── ducknameReducers.ts
    ├── sagas
    │   └── ducknameSagas.ts
    └── types
        └── ducknameTypes.ts

5 directories, 4 files

```

Note: It can also take multiple arguments at the same time.

## WIP:

1. Populate files with boilerplate code.
2. Link `reducers` in `rootReducer`.

#### dev: [Ahmad](https://github.com/ahmad2smile)
