import subprocess
import os
import copy
import json
import multiprocessing
from datetime import datetime
import sys


def read_targets(folder):
    ans = {}
    try:
        ret = subprocess.run("cd {} && cargo bolero list".format(folder), shell=True, capture_output=True, timeout=1800)
        if ret.returncode != 0:
            print('empty driver', folder)
        for l in ret.stdout.decode('utf-8').splitlines():
            if 'tests_llm_16' in l or 'tests_rug' in l:
                obj = json.loads(l)
                ans[obj['test']] = obj['test'].replace(':','_')
    except subprocess.TimeoutExpired:
        print('timeout driver', folder)
    return ans


def execute_fuzzing(f):
    fin = subprocess.run('cargo ws list -l', shell=True, cwd=f, capture_output=True)
    with open(f+'/fuzz_log.txt', 'w') as sys.stdout:
        for l in fin.stdout.decode('utf-8').splitlines():
            ls = l.split(' ')
            crate = ls[0].strip()
            path = ls[-1]
            ans = read_targets(f+'/'+path)
            print(f, crate, ans)
            if len(ans) <= 0:
                print('empty ', f+'/'+path)
                continue
            for k, v in ans.items():
                mod = ''
                func = ''
                cs = k.split('::')
                for i in range(len(cs)):
                    if 'tests_llm_16' in cs[i] or 'tests_rug_' in cs[i]:
                        mod = cs[i]
                        func = cs[i+1]
                st_path = 'inputs/{}/{}'.format(mod, func)
                cmd = 'RUSTFLAGS="-C debug-assertions=off" cargo bolero test -t 3sec -T 60sec --corpus-dir {} {}'
                fcmd = cmd.format(st_path, k)
                try:
                    fin = subprocess.run(fcmd, cwd=f+'/'+path, shell=True, capture_output=True, timeout=40)
                except Exception:
                    print('timeout or err', k)
        print(f, 'done')


if __name__ == '__main__':
    execute_fuzzing((sys.argv[1]))
    # args = []
    # for fd in os.listdir('.'):
    #     if not os.path.isdir(fd):
    #         continue
    #     # fd = sys.argv[1]
    #     fin = subprocess.run('cargo ws list -l', shell=True, cwd=fd, capture_output=True)
    #     if fin.returncode == 0:
    #         args.append(fd)
    # print(args)
    # with multiprocessing.Pool(24) as p:
    #     p.map(execute_fuzzing, args)
    # print('done')
