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



def normalize_and_rename_paths(file_paths):
    """
    Replaces '/./' with '/' in file paths, renames files on disk, 
    and returns the updated list of file paths.
    """
    updated_paths = []

    for old_path in file_paths:
        if "/./" in old_path:
            new_path = old_path.replace("/./", "/")
            try:
                os.makedirs(os.path.dirname(new_path), exist_ok=True)
                os.rename(old_path, new_path)
                print(f"Renamed: {old_path} -> {new_path}")
                updated_paths.append(new_path)
            except FileNotFoundError:
                print(f"File not found: {old_path}")
                updated_paths.append(old_path)  # fallback to old path
            except Exception as e:
                print(f"Error renaming {old_path}: {e}")
                updated_paths.append(old_path)
        else:
            updated_paths.append(old_path)

    return updated_paths

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
            # print(files)
            # files = normalize_and_rename_paths(files)
            ans = {}
            fin = subprocess.run("cargo test --no-run", shell=True, cwd=fd +'/' +path, capture_output=True)
            #print(fin)
            if fin.returncode != 0:
                print('err fin', fd, crate, path)
                continue
            for f in files:
                print(" {} {}".format(exe, f))
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
    # execute_one_fd(sys.argv[1])
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
