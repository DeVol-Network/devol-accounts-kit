# Tests

### Start

Regular start of all tests (simultaneously):

`cargo test`

Start tests in 1 thread (sequentially):

`cargo test -- --test-threads=1`

Start tests in 1 thread (with console debug prints):

`cargo test -- --nocapture`