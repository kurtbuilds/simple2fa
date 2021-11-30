const Benchmark = require('benchmark')
const suite = new Benchmark.Suite;
const simple2fa = require('../index.node');
const {authenticator} = require('otplib');

console.log()

// add tests
suite
    .add('bench_simple2fa', function () {
        const secret = simple2fa.generate_2fa_secret();
        const code = simple2fa.generate_2fa_code(secret);
        simple2fa.check_2fa_code(secret, code);
    })
    .add('bench_otplib', function () {
        const secret = authenticator.generateSecret(); // base32 encoded hex secret key
        const code = authenticator.generate(secret);
        authenticator.check(code, secret);
    })
    .on('cycle', function (event) {
    })
    .on('complete', function () {
        console.log('The winner is: ' + this.filter('fastest').map('name') + '\n');
        let benchmarks = this.sort(k => 1/k.hz)

        let fastest = benchmarks[0];

        benchmarks.forEach(function (bench) {
            let v = 1/bench.hz;
            let time_per_run = v / 1000
            let units = 's'
            if (time_per_run < 0.001) {
                time_per_run *= 1000
                units = 'ms'
                if (time_per_run < 0.001) {
                    time_per_run *= 1000
                    units = 'µs'
                    if (time_per_run < 0.001) {
                        time_per_run *= 1000
                        units = 'ns'
                    }
                }
            }
            let comparison = ''
            if (bench.name !== fastest.name) {
                comparison = ` (${Math.round(100 * (1/bench.hz) / (1/fastest.hz)) / 100}x slower than the winner)`
            }
            console.log(`${bench.name}: Took ${time_per_run}${units} per run. ${comparison}`)
        });
    })
    // run async
    .run()