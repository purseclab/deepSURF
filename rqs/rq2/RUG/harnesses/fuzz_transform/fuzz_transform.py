import subprocess
import os
import sys
import json
import copy
import time
import multiprocessing


def recur_scan(folder:str, ans: list):
    if not os.path.exists(folder):
        return
    for f in os.listdir(folder):
        path = folder +'/'+f
        if os.path.isfile(path) and (path.endswith(".rs")):
            ans.append(path)
        if os.path.isdir(path):
            recur_scan(path, ans)


def execute_one_fd(arg):
    fd = arg[0]
    exe = arg[1]
    fin = subprocess.run('cargo ws list -l', shell=True, cwd=fd, capture_output=True)
    uid = 0
    total = 0
    succeed = 0
    skip = 0
    with open(fd+'/trans.log', 'w') as sys.stdout:
        for l in fin.stdout.decode('utf-8').splitlines():
            ls = l.split(' ')
            crate = ls[0].strip()
            path = ls[-1]
            files = []
            recur_scan(os.getcwd() +'/'+fd+'/'+path+'/src',files)
            ans = {}
            fin = subprocess.run("cargo test --no-run", shell=True, cwd=fd +'/' +path, capture_output=True)
            if fin.returncode != 0:
                print('err fin', fd, crate, path)
                continue
            for f in files:
                ret = subprocess.run("{} {}".format(exe, f), shell=True, capture_output=True)
                if ret.returncode != 0:
                    print('err in scan file', f)
                    continue
                for tar in ret.stdout.decode('utf-8').splitlines():
                    with open(f, 'r+') as fp:
                        origin = fp.readlines()
                        ret = subprocess.run("{} {} {}".format(exe, f, tar), shell=True, capture_output=True)
                        total += 1
                        fin = subprocess.run("cargo test --no-run", shell=True, cwd=fd +'/' +path, capture_output=True)
                        if fin.returncode != 0:
                            # err
                            print('err', fd, f, tar)
                            fp.truncate(0)
                            fp.seek(0)
                            fp.writelines(origin)
                            fp.flush()
                            os.fsync(fp.fileno())
                        else:
                            succeed += 1
                            for l in ret.stdout.decode('utf-8').split('_rrrruuuugggg_'):
                                ls = l.split('~')
                                if len(l) == 0:
                                    continue
                                mod = ls[0]
                                fn = ls[1]
                                var = ls[2]
                                var_idx = l.find(var)
                                st = l.find('~', var_idx)
                                val = l[st+1:]
                                if mod not in ans:
                                    ans[mod] = {}
                                if fn not in ans[mod]:
                                    ans[mod][fn] = {}
                                if var not in ans[mod][fn]:
                                    ans[mod][fn][var] = val
                                else:
                                    print('err', mod, fn, var, val, ans[mod])
                        fp.close()
            fin = subprocess.run("cargo test --no-run", shell=True, cwd=fd, capture_output=True)
            print(crate, path, succeed, total, fin.returncode)
            with open(fd+'/'+crate+'_fuzz_trans.json', 'w') as jp:
                json.dump(ans, jp)






if __name__ == '__main__':
    execute_one_fd((sys.argv[1], sys.argv[2]))
    exit(0)
    args = []
    for fd in os.listdir('.'):
        if not os.path.isdir(fd):
            continue
        # ret = subprocess.run("cargo test --no-run", shell=True, cwd=fd, capture_output=True)
        # if ret.returncode == 0:
            # execute_one_fd(fd, sys.argv[1])
        args.append((fd, sys.argv[1]))
    print(args)
    with multiprocessing.Pool(24) as p:
        p.map(execute_one_fd, args)
