import {benchmarkWasm} from 'rust-lib'

/**
 * The JavaScript variant of our WASM benchmark
 */
function benchmarkJs(n: number) {
    if (n === 1 || n === 2) return 1;
    return benchmarkJs(n - 1) + benchmarkJs(n - 2);
}

/**
 * The test executor
 * @param fn
 */
function runTest(fn: typeof benchmarkWasm) {
    fn(40) // warmup

    const start = Date.now()
    let result: unknown = 0

    for (let n = 0; n < 5; n++) {
        result = fn(40)
    }

    return {
        result,
        time: `${Date.now() - start}ms`
    }
}

// run the tests
console.log(
    [benchmarkJs, benchmarkWasm].map(runTest)
)
