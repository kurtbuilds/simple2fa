import simple2fa
import pyotp
import timeit

BENCHES = []


def bench(f):
    BENCHES.append(f.__name__)
    return f


def run_benchmark():
    timings = []
    for func_name in BENCHES:
        time = timeit.timeit(func_name + '()', number=1000, globals=globals())
        timings.append((func_name, time))

    results = sorted(timings, key=lambda x: x[1])

    fastest = results[0]

    print("The winner is: {}\n".format(fastest[0]))

    for k, v in results:
        time_per_run = v / 1000
        units = 's'
        if time_per_run < 0.001:
            time_per_run *= 1000
            units = 'ms'
            if time_per_run < 0.001:
                time_per_run *= 1000
                units = 'µs'
                if time_per_run < 0.001:
                    time_per_run *= 1000
                    units = 'ns'
        comparison = ''
        if k != fastest[0]:
            comparison = f' ({round(time_per_run / fastest[1], 2)}x slower than the winner)'
        print(f'{k}: Took {time_per_run:.3f}{units} per run.{comparison}')


SECRET = simple2fa.generate_2fa_secret()


@bench
def bench_simple2fa():
    simple2fa.generate_2fa_secret()
    code = simple2fa.generate_2fa_code(SECRET)
    simple2fa.check_2fa_code(SECRET, code)


@bench
def bench_pyotp():
    pyotp.random_base32()
    totp = pyotp.TOTP(SECRET)
    code = totp.now()
    totp.verify(code)


run_benchmark()