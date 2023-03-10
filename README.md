# TI-Smoked

## Rewrite it in Rust (RIIR)

[![Rewrite it in Rust (RIIR)](img/riir.jpg)](https://www.redbubble.com/i/poster/Rewrite-It-In-Rust-Rust-Programming-by-tribaltattoo/130040421.LVTDI)

It is supposed to be so much better.
Anyway, a small play attempt to rewrite a Smoke Testing tool that we use at work.
I learned a lot by doing this silly project and it was fun :)
Happy coding

Kenneth

Notes:
The poster is not mine, but you can buy it from the link provided.

## How to run

To run form repo:
```sh
> cargo run --release -- <env>
```

If not build it, and run it:
```sh
> cargo build --release
> ./target/release/ti-smoked <env>
```

`<env>` is anything you call the configuration files stored in local.
### Example output

```shell
❯ cargo run --release -- prod
    Finished release [optimized] target(s) in 0.24s
     Running `target/release/ti-smoked prod`
Hello, world!

BasePath: /Users/kenneth.fossen/GIT/ti-smoked/local
Opening file: /Users/kenneth.fossen/GIT/ti-smoked/local/prod.json

Test Target: Common Library PROD

| Detector       | Failure       | Duration | Details    |
----------------------------------------------------------
| Get Schema     |               | 1997 ms      |               |
| Dummy Test     |               | 0 ms         | Good  |
| Alive Test     |               | 559 ms       |               |
| Codes Test     |               | 221 ms       |               |
| Get Libraries  |               | 455 ms       |               |
| Get View Def   |               | 102 ms       | Good  |
| MappedCode     |               | 943 ms       |               |
----------------------------------------------------------
```
## Performance

I tried to stay true to the design of the other application. 
This came with some bottlenecks in Rust, and I, especially made some changes to the token fetching procedure to make it faster.
Creating new instances of `ClientSecretCredentials` was expensive and slow, and was changed to just store the token needed to do the requests.
This has some implications, e.g. if the tests will have a combined runtime of over 1 hour this approach will fail.
The token is only valid for that timespan.
Other than that the application performs similar if not better.
Also, the data structures where very large and complex, so we do skip deserializing some fields, if not entire datastructures. 

## This vs Original project
PS: Im missing one test from the original project.

```shell
❯ tokei
===============================================================================
Language            Files        Lines         Code     Comments       Blanks
===============================================================================
Markdown                1           29            0           20            9
Rust                   12         1075          914           40          121
TOML                    1           20           15            2            3
===============================================================================
Total                  14         1124          929           62          133
===============================================================================
```

```shell
❯ tokei
===============================================================================
Language            Files        Lines         Code     Comments       Blanks
===============================================================================
C#                     25         1058          909            4          145
JSON                    1           10           10            0            0
Markdown                1            5            0            3            2
MSBuild                 3           56           45            0           11
Visual Studio Sol|      1           69           68            0            1
YAML                    1           30           20            5            5
===============================================================================
Total                  32         1228         1052           12          164
===============================================================================
```

### FlameGraph

The flamegraph shows that the most time is used on Authentication.
In the first edition of this, we ran authentication for each test and that was really slow.
This was corrected and we have a much better performance.

![Flamegraph](ti-smoked.svg)